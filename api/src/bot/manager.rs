use std::sync::{Arc, RwLock};

use dashmap::DashMap;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use futures_util::stream;
use log::{error, info, warn};
use sqlx::{Pool, Postgres};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::tungstenite::protocol::Message as SocketMessage;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use twitch_irc::message::{IRCMessage, PrivmsgMessage, ServerMessage};
use url::Url;

use super::channel::{ChannelDef, ChannelHandler, ChannelName, ChannelRegistry};
use super::types::BotExternalAction;
use super::utils::{get_bot_access_token, LOG_TARGET, WEBSOCKET_CLIENT_URL};

use crate::errors::AppError;
use crate::models;
use crate::states::app_config::AppConfig;

// Type alias
type ThreadSafeRw<T> = Arc<RwLock<T>>;
type SocketStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type SocketSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, SocketMessage>;

// Constants
#[derive(Clone, Debug)]
pub enum BotStatus {
    Connected(mpsc::Sender<BotExternalAction>),
    Disconnected,
}

pub struct BotManager {
    config: AppConfig,
    pool: Pool<Postgres>,
    status: ThreadSafeRw<BotStatus>,
    pub channel_registry: Arc<ChannelRegistry>,

    bot_status_sender: Option<mpsc::Sender<BotStatus>>,
    bot_external_actions_sender: Option<mpsc::Sender<BotExternalAction>>,
}

impl BotManager {
    pub fn new(config: AppConfig, pool: Pool<Postgres>) -> Self {
        Self {
            config,
            pool,
            status: Arc::new(RwLock::new(BotStatus::Disconnected)),

            bot_status_sender: None,
            bot_external_actions_sender: None,

            channel_registry: Arc::new(DashMap::<ChannelName, ChannelDef>::new()),
        }
    }

    pub async fn connect(&mut self) -> Result<(), AppError> {
        // Getting bot identification informations
        let bot_access_token = get_bot_access_token(&self.config, &self.pool, LOG_TARGET).await?;

        // Prepare communication channels
        let (bot_status_sender, bot_status_consumer) = mpsc::channel::<BotStatus>(5);
        self.bot_status_sender = Some(bot_status_sender);

        let (bot_external_actions_sender, bot_external_actions_consumer) =
            mpsc::channel::<BotExternalAction>(32);
        self.bot_external_actions_sender = Some(bot_external_actions_sender);

        let (message_broadcast_sender, message_broadcast_consumer) =
            watch::channel::<Option<PrivmsgMessage>>(None);

        // Open Twitch Socket connection
        let url = Url::parse(WEBSOCKET_CLIENT_URL)?;
        let (ws_stream, _) = connect_async(url).await?;
        let (mut socket_sink, socket_stream) = ws_stream.split();

        info!(
            target: LOG_TARGET,
            "Bot connected to Twitch WS, proceed to authentication..."
        );

        self.start_twitch_socket_consumer(socket_stream, message_broadcast_sender);
        self.authenticate_to_irc(&mut socket_sink, &bot_access_token)
            .await?;

        self.start_bot_status_consumer(bot_status_consumer);
        self.start_bot_external_action_consumer(
            bot_external_actions_consumer,
            socket_sink,
            message_broadcast_consumer,
        );

        Ok(())
    }

    pub fn status(&self) -> Result<BotStatus, AppError> {
        Ok(self
            .status
            .read()
            .map(|inner_status| inner_status.clone())?)
    }

    /**
     * PRIVATE METHODS TO MAKE THE CODE MORE READABLE
     */
    async fn authenticate_to_irc(
        &self,
        socket_sink: &mut SocketSink,
        access_token: &str,
    ) -> Result<(), AppError> {
        socket_sink
            .send(SocketMessage::Text(
                "CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".to_string(),
            ))
            .await?;

        socket_sink
            .send(SocketMessage::Text(format!("PASS oauth:{}", access_token)))
            .await?;

        socket_sink
            .send(SocketMessage::Text(
                format!("NICK {}", &self.config.chat_bot_username).to_string(),
            ))
            .await?;

        Ok(())
    }

    fn start_twitch_socket_consumer(
        &self,
        socket_stream: SocketStream,
        message_broadcast_sender: watch::Sender<Option<PrivmsgMessage>>,
    ) {
        let bot_status_sender = self
            .bot_status_sender
            .clone()
            .expect("Incorrect state: bot_status communication channel is not set");

        let bot_external_actions_sender = self
            .bot_external_actions_sender
            .clone()
            .expect("Incorrect state: external_actions communication channel is not set");

        tokio::spawn(async move {
            let filtered_message_stream = socket_stream
                .filter_map(|data| async {
                    if let Ok(SocketMessage::Text(message)) = data {
                        let message = message.clone();
                        Some(
                            message
                                .lines()
                                .map(|line| String::from(line))
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        warn!(
                            target: LOG_TARGET,
                            "Unhandled a message that was NOT a Text: {:?}", &data
                        );
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
                });

            filtered_message_stream
                .for_each(|message| async {
                    match message {
                        ServerMessage::GlobalUserState(_) => {
                            let update_bot_status = bot_status_sender
                                .send(BotStatus::Connected(bot_external_actions_sender.clone()))
                                .await;

                            if let Err(e) = update_bot_status {
                                error!(
                                    target: LOG_TARGET,
                                    "Cannot send the message to update the bot status: {:?}", e
                                );
                            } else {
                                info!(
                                    target: LOG_TARGET,
                                    "Bot authenticated and ready to connect to channels"
                                );
                            };
                        }
                        ServerMessage::Ping(_) => {
                            info!(target: LOG_TARGET, "PING received");
                            let send_pong_msg = bot_external_actions_sender
                                .send(BotExternalAction::Pong)
                                .await;

                            if let Err(e) = send_pong_msg {
                                error!(target: LOG_TARGET, "Cannot send the PONG: {:?}", &e);
                            };
                        }
                        ServerMessage::Privmsg(priv_msg) => {
                            let send_msg_to_broadcast =
                                message_broadcast_sender.send(Some(priv_msg.clone()));

                            info!(target: LOG_TARGET, "Message: {}", priv_msg.message_text);

                            if let Err(e) = send_msg_to_broadcast {
                                error!(
                                    target: LOG_TARGET,
                                    "Cannot broadcast msgs to channels {:?}", &e
                                );
                            };
                        }
                        unhandled_message => {
                            warn!(
                                target: LOG_TARGET,
                                "Unhandled message: {:?}", &unhandled_message
                            );
                        }
                    }
                })
                .await;
        });
    }

    fn start_bot_status_consumer(&self, mut bot_status_consumer: mpsc::Receiver<BotStatus>) {
        let shared_bot_status = self.status.clone();

        tokio::spawn(async move {
            while let Some(new_status) = bot_status_consumer.recv().await {
                if let Ok(status) = shared_bot_status.write() {
                    let mut status = status;
                    *status = new_status.clone();
                } else {
                    // TODO: Do something to close the stream
                    error!(
                        target: LOG_TARGET,
                        "Cannot get the lock to write the bot status"
                    )
                }
            }
        });
    }

    fn start_bot_external_action_consumer(
        &self,
        mut bot_external_actions_consumer: mpsc::Receiver<BotExternalAction>,
        mut socket_sink: SocketSink,
        message_broadcast_consumer: watch::Receiver<Option<PrivmsgMessage>>,
    ) {
        let channel_registry_handle = self.channel_registry.clone();
        let bot_external_actions_sender = self.bot_external_actions_sender.clone();
        let pool = self.pool.clone();

        tokio::spawn(async move {
            while let Some(external_action) = bot_external_actions_consumer.recv().await {
                match external_action {
                    BotExternalAction::Pong => {
                        info!(target: LOG_TARGET, "Sending pong...");

                        let send_pong = socket_sink.send(SocketMessage::Text("PONG".to_string()));
                        if let Err(e) = send_pong.await {
                            error!(target: LOG_TARGET, "Cannot send message to WS {:?}", e);
                        };
                    }
                    BotExternalAction::Join {
                        channel_name,
                        user_id,
                    } => {
                        info!(target: LOG_TARGET, "Trying to join {}", channel_name);

                        let send_join = socket_sink.send(SocketMessage::Text(
                            format!("JOIN #{}", &channel_name).to_string(),
                        ));

                        if let Err(e) = send_join.await {
                            error!(target: LOG_TARGET, "Cannot send message to WS {:?}", e);
                        } else {
                            if let Some(external_action_sender) =
                                bot_external_actions_sender.clone()
                            {
                                if let Some(user_id) = user_id {
                                    let commands =
                                        models::UserCommand::get_all_by_user_id(&pool, &user_id)
                                            .await;

                                    match commands {
                                        Ok(commands) => {
                                            let channel_handler = ChannelHandler::new(
                                                channel_name.clone(),
                                                commands,
                                                channel_registry_handle.clone(),
                                                message_broadcast_consumer.clone(),
                                                external_action_sender.clone(),
                                            );
                                            let tx_kill_sig = channel_handler.run();
                                            channel_registry_handle.insert(
                                                channel_name,
                                                (channel_handler, tx_kill_sig),
                                            );
                                        }
                                        Err(_) => {
                                            error!(
                                                target: LOG_TARGET,
                                                "Cannot start channel actor"
                                            );
                                        }
                                    }
                                } else {
                                    let channel_handler = ChannelHandler::new(
                                        channel_name.clone(),
                                        vec![],
                                        channel_registry_handle.clone(),
                                        message_broadcast_consumer.clone(),
                                        external_action_sender.clone(),
                                    );
                                    let tx_kill_sig = channel_handler.run();
                                    channel_registry_handle
                                        .insert(channel_name, (channel_handler, tx_kill_sig));
                                };
                            } else {
                                error!(target: LOG_TARGET, "External action sender is not ready");
                            }
                        };
                    }
                    BotExternalAction::Leave(channel_name) => {
                        info!(target: LOG_TARGET, "Trying to leave {}", channel_name);

                        let send_part = socket_sink.send(SocketMessage::Text(
                            format!("PART #{}", &channel_name).to_string(),
                        ));

                        if let Err(e) = send_part.await {
                            error!(target: LOG_TARGET, "Cannot send message to WS {:?}", e);
                        } else {
                            let channel_def = channel_registry_handle.remove(&channel_name);

                            if let Some((_, (_, tx_kill_sig))) = channel_def {
                                if let Err(e) = tx_kill_sig.send(()) {
                                    error!(
                                        target: LOG_TARGET,
                                        "Cannot kill channel actor: {:?}", e
                                    );
                                }
                            }
                        };
                    }
                    BotExternalAction::Respond {
                        channel_name,
                        message,
                    } => {
                        info!(
                            target: LOG_TARGET,
                            "Sending message in {}...", &channel_name
                        );

                        let send_msg = socket_sink.send(SocketMessage::Text(
                            format!("PRIVMSG #{} :{}", &channel_name, &message).to_string(),
                        ));

                        if let Err(e) = send_msg.await {
                            error!(target: LOG_TARGET, "Cannot send message to WS {:?}", e);
                        }
                    }
                };
            }
        });
    }
}
