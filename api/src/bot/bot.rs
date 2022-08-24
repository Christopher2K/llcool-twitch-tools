use actix_web::web::Data;
use futures::{SinkExt, StreamExt};
use reqwest::Url;
use std::str::FromStr;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol};

use crate::bot::types::{BotMessage, TwitchMessage};
use crate::errors::{AppError, AppErrorType};
use crate::models::bot_credentials::get_bot_credentials_by_user_id;
use crate::models::user::get_user_by_username;
use crate::types::DbPool;

const LOG_TARGET: &'static str = "twitch_bot";
const WEBSOCKET_CLIENT_URL: &'static str = "wss://irc-ws.chat.twitch.tv:443";

type Sender = mpsc::Sender<BotMessage>;

#[derive(Clone)]
pub enum BotStatus {
    Connected(Sender),
    Disconnected,
}

pub struct Bot {
    bot_name: String,
    pool: Data<DbPool>,

    bot_status: BotStatus,
}

impl Bot {
    pub fn new(pool: Data<DbPool>, bot_name: String) -> Self {
        Self {
            bot_name,
            pool,
            bot_status: BotStatus::Disconnected,
        }
    }

    pub fn status(&self) -> BotStatus {
        self.bot_status.clone()
    }

    pub async fn connect(&mut self) -> Result<(), AppError> {
        let bot_username = self.bot_name.clone();

        // GET BOT AUTHENTIFCATION
        let bot_access_token = {
            let db = self.pool.get().map_err(|e| {
                AppError::from(AppErrorType::InternalError).inner_error(&e.to_string())
            })?;

            let db_error = AppError::from(AppErrorType::DatabaseError);

            get_user_by_username(&db, &self.bot_name)
                .and_then(|user| get_bot_credentials_by_user_id(&db, &user.id))
                .map_err(|e| db_error.clone().inner_error(&e.to_string()))?
                .access_token
        };

        // OPEN APP COMMUNICATION CHANNELS
        let (tx, mut rx) = mpsc::channel::<BotMessage>(32);
        let (tx_status, mut rx_status) = mpsc::channel::<()>(10);

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
            while let Some(message) = rx.recv().await {
                let socket_error = AppError::from(AppErrorType::WebSocketError);

                let result = match message {
                    BotMessage::JoinChat(channel) => {
                        log::info!(target: LOG_TARGET, "Bot joined channel #{}", channel);

                        write
                            .send(protocol::Message::Text(
                                format!("JOIN #{}", &channel).to_string(),
                            ))
                            .await
                    }
                    BotMessage::LeaveChat(channel) => {
                        log::info!(target: LOG_TARGET, "Bot left channel #{}", channel);

                        write
                            .send(protocol::Message::Text(
                                format!("PART #{}", &channel).to_string(),
                            ))
                            .await
                    }
                };

                if let Err(err) = result {
                    let e = socket_error.clone().inner_error(&err.to_string());
                    log::error!(target: LOG_TARGET, "Error when sending a message {}", e);
                };
            }
        });

        if let Some(_) = rx_status.recv().await {
            self.bot_status = BotStatus::Connected(tx.clone())
        };

        Ok(())
    }
}
