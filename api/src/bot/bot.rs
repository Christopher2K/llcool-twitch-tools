use actix_web::web::Data;
use futures::stream::{self, SplitSink, SplitStream};
use futures::SinkExt;
use futures::StreamExt;
use reqwest::Url;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol, MaybeTlsStream, WebSocketStream};
use twitch_irc::irc;
use twitch_irc::message::{AsRawIRC, IRCMessage, ServerMessage};

use crate::errors::*;
use crate::models::bot_credentials::{
    get_bot_credentials_by_user_id, update_bot_credentials, UpdateBotCredentials,
};
use crate::models::user::get_user_by_username;
use crate::states::app_config::AppConfig;
use crate::twitch::id_api::renew_token;
use crate::types::DbPool;

pub const LOG_TARGET: &'static str = "twitch_bot";
const WEBSOCKET_CLIENT_URL: &'static str = "wss://irc-ws.chat.twitch.tv:443";

type BotActionSender = mpsc::Sender<BotAction>;
type SocketWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, protocol::Message>;
type SocketReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type ChannelSet = Arc<RwLock<HashSet<String>>>;

#[derive(Debug, Clone)]
pub enum BotAction {
    JoinChat(String),
    LeaveChat(String),
    Pong,
}

#[derive(Clone)]
pub enum BotStatus {
    Connected(BotActionSender),
    Disconnected,
}

pub struct Bot {
    app_config: Data<AppConfig>,
    pool: Data<DbPool>,
    bot_status: BotStatus,
    pub connected_channels: ChannelSet,
}

impl Bot {
    pub fn new(pool: Data<DbPool>, app_config: Data<AppConfig>) -> Self {
        Self {
            app_config,
            pool,
            bot_status: BotStatus::Disconnected,
            connected_channels: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn status(&self) -> BotStatus {
        self.bot_status.clone()
    }

    pub async fn connect(&mut self) -> Result<(), AppError> {
        let bot_username = self.app_config.chat_bot_username.clone();

        // GET BOT AUTHENTIFCATION
        let bot_access_token = {
            log::info!(
                target: LOG_TARGET,
                "Getting and refreshing bot credentials..."
            );
            let db = self.pool.get()?;

            // RENEW BY DEFAULT THE TWITCH BOT TOKEN
            let credentials = get_user_by_username(&db, &bot_username)
                .and_then(|user| get_bot_credentials_by_user_id(&db, &user.id))?;

            let tokens = renew_token(&self.app_config, &credentials.refresh_token).await?;
            update_bot_credentials(
                &db,
                &credentials.id,
                UpdateBotCredentials {
                    access_token: &tokens.access_token.clone(),
                    refresh_token: &tokens.refresh_token.clone(),
                },
            )?;

            tokens.access_token
        };

        // OPEN TASKS COMMUNICATION CHANNELS
        // -> Receive actions from HTTP handler and transmit them to the socket sender
        let (tx_action, rx_action) = mpsc::channel::<BotAction>(32);

        // -> Receive boolean messages to tell the struct to update its connection status
        let (tx_connected, mut rx_connected) = mpsc::channel::<bool>(1);

        // OPEN TWITCH TMI CONNECTION
        let url = Url::parse(WEBSOCKET_CLIENT_URL)?;
        let (ws_stream, _) = connect_async(url).await?;

        let (mut socket_writer, socket_reader) = ws_stream.split();
        log::info!(
            target: LOG_TARGET,
            "Bot connected to Twitch WS, proceed to authentication..."
        );
        self.authenticate_to_irc(&mut socket_writer, &bot_access_token)
            .await?;

        log::info!(
            target: LOG_TARGET,
            "Waiting for authentication confirmation..."
        );

        let _reading_task_handler =
            self.start_reading(socket_reader, tx_action.clone(), tx_connected.clone());
        let _reading_internal_msg_handler =
            self.start_reading_internal_and_respond(socket_writer, rx_action);

        if let Some(is_connected) = rx_connected.recv().await {
            if is_connected {
                self.bot_status = BotStatus::Connected(tx_action.clone());
            } else {
                self.bot_status = BotStatus::Disconnected
            }
        };

        Ok(())
    }

    async fn authenticate_to_irc(
        &self,
        socket_writer: &mut SocketWriter,
        access_token: &str,
    ) -> Result<(), AppError> {
        socket_writer
            .send(protocol::Message::Text(
                "CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".to_string(),
            ))
            .await?;

        socket_writer
            .send(protocol::Message::Text(format!(
                "PASS oauth:{}",
                access_token
            )))
            .await?;

        socket_writer
            .send(protocol::Message::Text("NICK llcoolbot_".to_string()))
            .await?;

        Ok(())
    }

    fn start_reading_internal_and_respond(
        &self,
        mut socket_writer: SocketWriter,
        mut bot_action_receiver: mpsc::Receiver<BotAction>,
    ) -> tokio::task::JoinHandle<()> {
        let shared_socket_list = self.connected_channels.clone();

        tokio::spawn(async move {
            while let Some(action) = bot_action_receiver.recv().await {
                let message = match action.clone() {
                    BotAction::Pong => protocol::Message::Text(irc!["PONG"].as_raw_irc()),
                    BotAction::JoinChat(channel_name) => protocol::Message::Text(
                        irc!["JOIN", format!("#{}", &channel_name)].as_raw_irc(),
                    ),
                    BotAction::LeaveChat(channel_name) => protocol::Message::Text(
                        irc!["PART", format!("#{}", &channel_name)].as_raw_irc(),
                    ),
                };

                if let Err(e) = socket_writer.send(message.clone()).await {
                    log::error!(
                        target: LOG_TARGET,
                        "Failed to send a message: MESSAGE => {} | ERROR => {} ",
                        &message,
                        &e
                    );
                } else {
                    let channel_list_edition = match action {
                        BotAction::JoinChat(channel_name) => shared_socket_list
                            .write()
                            .and_then(|mut lock| Ok(lock.insert(channel_name.clone()))),
                        BotAction::LeaveChat(channel_name) => shared_socket_list
                            .write()
                            .and_then(|mut lock| Ok(lock.insert(channel_name.clone()))),
                        _ => Ok(true),
                    };

                    if let Err(e) = channel_list_edition {
                        log::error!(target: LOG_TARGET, "Cannot update the channel list :{}", &e);
                    }

                    log::info!(target: LOG_TARGET, "SENT:{}", &message);
                }
            }
        })
    }

    fn start_reading(
        &self,
        socket_reader: SocketReader,
        pong_sender: mpsc::Sender<BotAction>,
        connection_status_sender: mpsc::Sender<bool>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            socket_reader
                .filter_map(|data| async move {
                    if let Ok(protocol::Message::Text(text)) = data {
                        let t = text.clone();
                        Some(t.lines().map(|l| String::from(l)).collect::<Vec<_>>())
                    } else {
                        None
                    }
                })
                .flat_map(|lines| {
                    stream::iter(
                        lines
                            .iter()
                            .filter_map(|line| {
                                IRCMessage::parse(line)
                                    .map_err(AppError::from)
                                    .and_then(|irc_message| {
                                        ServerMessage::try_from(irc_message).map_err(AppError::from)
                                    })
                                    .ok()
                            })
                            .collect::<Vec<_>>(),
                    )
                })
                .for_each(|twitch_message| async {
                    match twitch_message {
                        ServerMessage::GlobalUserState(_) => {
                            log::info!(target: LOG_TARGET, "Bot user authenticated");

                            if let Err(e) = connection_status_sender.send(true).await {
                                log::error!(
                                    target: LOG_TARGET,
                                    "Failed to update connexion status: {}",
                                    &e
                                );
                            }
                        }
                        ServerMessage::Ping(_) => {
                            log::info!(target: LOG_TARGET, "PING received");

                            if let Err(e) = pong_sender.send(BotAction::Pong).await {
                                log::error!(
                                    target: LOG_TARGET,
                                    "Failed to respond to PING message: {}",
                                    &e
                                );
                            }
                        }
                        unhandled_msg => {
                            log::info!(
                                target: LOG_TARGET,
                                "Unhandled socket message {:?}",
                                &unhandled_msg
                            );
                        }
                    }
                })
                .await
        })
    }
}
