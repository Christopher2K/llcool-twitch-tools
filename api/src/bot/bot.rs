use actix_web::web::Data;
use futures::{SinkExt, StreamExt};
use reqwest::Url;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol};

use crate::bot::types::{BotMessage, ConnectedChannelsSetMessage, TwitchMessage};
use crate::errors::{AppError, AppErrorType};
use crate::models::bot_credentials::{
    get_bot_credentials_by_user_id, update_bot_credentials, UpdateBotCredentials,
};
use crate::models::user::get_user_by_username;
use crate::states::app_config::AppConfig;
use crate::twitch::id_api::renew_token;
use crate::types::DbPool;

const LOG_TARGET: &'static str = "twitch_bot";
const WEBSOCKET_CLIENT_URL: &'static str = "wss://irc-ws.chat.twitch.tv:443";

type Sender = mpsc::Sender<BotMessage>;
type ChannelSet = Arc<Mutex<HashSet<String>>>;

#[derive(Clone)]
pub enum BotStatus {
    Connected(Sender),
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
            connected_channels: Arc::new(Mutex::new(HashSet::new())),
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
            let db = self.pool.get().map_err(|e| {
                AppError::from(AppErrorType::InternalError).inner_error(&e.to_string())
            })?;

            // RENEW BY DEFAULT THE TWITCH BOT TOKEN
            let credentials = get_user_by_username(&db, &bot_username)
                .and_then(|user| get_bot_credentials_by_user_id(&db, &user.id))
                .map_err(|e| {
                    AppError::from(AppErrorType::EntityNotFoundError)
                        .clone()
                        .inner_error(&e.to_string())
                })?;

            let tokens = renew_token(&self.app_config, &credentials.refresh_token).await?;
            update_bot_credentials(
                &db,
                &credentials.id,
                UpdateBotCredentials {
                    access_token: &tokens.access_token.clone(),
                    refresh_token: &tokens.refresh_token.clone(),
                },
            )
            .map_err(|e| {
                AppError::from(AppErrorType::DatabaseError)
                    .clone()
                    .inner_error(&e.to_string())
            })?;

            tokens.access_token
        };

        // OPEN APP COMMUNICATION CHANNELS
        let (tx, mut rx) = mpsc::channel::<BotMessage>(32);
        let (tx_status, mut rx_status) = mpsc::channel::<()>(10);
        let (tx_channel_set, mut rx_channel_set) = mpsc::channel::<ConnectedChannelsSetMessage>(10);

        // OPEN TWITCH TMI CONNECTION
        let ws_error = AppError::from(AppErrorType::WebSocketError);

        let url = Url::parse(WEBSOCKET_CLIENT_URL).expect("Cannot parse WS url");
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| ws_error.clone().inner_error(&e.to_string()))?;

        let (mut write, read) = ws_stream.split();
        log::info!(
            target: LOG_TARGET,
            "Bot connected to Twitch WS, proceed to authentication..."
        );

        // AUTHENTICATION PROCESS
        write
            .send(protocol::Message::Text(
                "CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".to_string(),
            ))
            .await
            .map_err(|e| ws_error.clone().inner_error(&e.to_string()))?;

        write
            .send(protocol::Message::Text(format!(
                "PASS oauth:{}",
                &bot_access_token
            )))
            .await
            .map_err(|e| ws_error.clone().inner_error(&e.to_string()))?;

        write
            .send(protocol::Message::Text("NICK llcoolbot_".to_string()))
            .await
            .map_err(|e| ws_error.clone().inner_error(&e.to_string()))?;

        let tx_pong = tx.clone();

        let _socket_reader = tokio::spawn(async move {
            read.for_each(|data| async {
                match data {
                    Ok(msg) => {
                        if let protocol::Message::Text(text) = msg {
                            match TwitchMessage::from_str(&text).unwrap() {
                                TwitchMessage::RplWelcome => {
                                    log::info!(
                                        target: LOG_TARGET,
                                        "Bot authenticated with user {}",
                                        &bot_username
                                    );
                                    tx_status.send(()).await.unwrap();
                                }
                                TwitchMessage::Ping => {
                                    log::info!(target: LOG_TARGET, "PING received",);
                                    tx_pong.send(BotMessage::Pong).await.unwrap();
                                }
                                _ => log::info!(
                                    target: LOG_TARGET,
                                    "Unhandled socket message: {}",
                                    text
                                ),
                            }
                        }
                    }
                    Err(e) => {
                        log::error!(
                            target: LOG_TARGET,
                            "Cannot read this message: {}",
                            e.to_string()
                        )
                    }
                }
            })
            .await;
        });

        let _http_handler_msg_reader = tokio::spawn(async move {
            // TODO: Better error handling for this channel
            while let Some(message) = rx.recv().await {
                let socket_error = AppError::from(AppErrorType::WebSocketError);

                let result = match message {
                    BotMessage::JoinChat(channel) => {
                        log::info!(target: LOG_TARGET, "Bot joined channel #{}", channel);
                        write
                            .send(protocol::Message::Text(
                                format!("JOIN #{}", &channel).to_string(),
                            ))
                            .await;

                        tx_channel_set
                            .send(ConnectedChannelsSetMessage::Join(channel.clone()))
                            .await
                            .unwrap();
                    }
                    BotMessage::LeaveChat(channel) => {
                        log::info!(target: LOG_TARGET, "Bot left channel #{}", channel);
                        write
                            .send(protocol::Message::Text(
                                format!("PART #{}", &channel).to_string(),
                            ))
                            .await;

                        tx_channel_set
                            .send(ConnectedChannelsSetMessage::Leave(channel.clone()))
                            .await
                            .unwrap();
                    }
                    BotMessage::Pong => {
                        log::info!(target: LOG_TARGET, "Respond to PING message");

                        write
                            .send(protocol::Message::Text("PONG".to_string()))
                            .await;
                    }
                };

                // if let Err(err) = result {
                //     let e = socket_error.clone().inner_error(&err.to_string());
                //     log::error!(target: LOG_TARGET, "Error when sending a message {}", e);
                // };
            }
        });

        let connected_channels = self.connected_channels.clone();
        let _hash_set_manager = tokio::spawn(async move {
            while let Some(msg) = rx_channel_set.recv().await {
                {
                    let lock = connected_channels.lock();
                    match lock {
                        Ok(mut connected_channels) => match msg {
                            ConnectedChannelsSetMessage::Join(channel) => {
                                connected_channels.insert(channel);
                            }
                            ConnectedChannelsSetMessage::Leave(channel) => {
                                connected_channels.remove::<String>(&channel);
                            }
                        },
                        Err(err) => {
                            log::error!(
                                target: LOG_TARGET,
                                "Error when getting the hashset connected_channels {}",
                                err
                            );
                        }
                    };
                }
            }
        });

        if let Some(_) = rx_status.recv().await {
            self.bot_status = BotStatus::Connected(tx.clone())
        };

        Ok(())
    }
}
