use std::str::FromStr;

use actix_web::web::Data;
use futures::{SinkExt, StreamExt, TryStreamExt};
use reqwest::Url;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::{connect_async, tungstenite::protocol, MaybeTlsStream, WebSocketStream};

use crate::models::bot_credentials::get_bot_credentials_by_user_id;
use crate::models::user::get_user_by_username;
use crate::types::DbPool;

const WEBSOCKET_CLIENT_URL: &'static str = "wss://irc-ws.chat.twitch.tv:443";

type Sender = mpsc::Sender<BotMessage>;

#[derive(Debug)]
pub enum TwitchMessage {
    RplWelcome,
    Unknown(String),
}

impl FromStr for TwitchMessage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.split(" ").collect::<Vec<&str>>();

        if let Some(code) = items.get(1) {
            let parsed_code = match *code {
                "001" => Self::RplWelcome,
                _ => Self::Unknown(String::from(s)),
            };
            Ok(parsed_code)
        } else {
            Ok(Self::Unknown(String::from(s)))
        }
    }
}

#[derive(Debug)]
pub enum BotMessage {
    JoinChat(String),
    LeaveChat(String),
}

pub enum BotStatus {
    Connected(Sender),
    Disconnected,
}

pub struct Bot {
    bot_name: String,
    pool: Data<DbPool>,

    pub status: BotStatus,
}

impl Bot {
    pub fn new(pool: Data<DbPool>, bot_name: String) -> Self {
        Self {
            bot_name,
            pool,
            status: BotStatus::Disconnected,
        }
    }

    pub async fn connect(&mut self) -> () {
        // GET BOT AUTHENTIFCATION
        let bot_access_token = {
            let db = self.pool.get().unwrap();
            let bot_user = get_user_by_username(&db, &self.bot_name).unwrap();
            get_bot_credentials_by_user_id(&db, &bot_user.id)
                .unwrap()
                .access_token
        };

        // OPEN APP COMMUNICATION CHANNELS
        let channels = mpsc::channel::<BotMessage>(32);
        let (tx, mut rx) = channels;

        let (tx_status, mut rx_status) = mpsc::channel::<()>(10);

        // OPEN TWITCH TMI CONNECTION
        let url = Url::parse(WEBSOCKET_CLIENT_URL).unwrap();
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (mut write, read) = ws_stream.split();
        println!("Bot connected..... proceed to authentication");

        // AUTHENTICATION PROCESS
        write
            .send(protocol::Message::Text(
                "CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".to_string(),
            ))
            .await
            .unwrap();

        write
            .send(protocol::Message::Text(format!(
                "PASS oauth:{}",
                &bot_access_token
            )))
            .await
            .unwrap();

        write
            .send(protocol::Message::Text("NICK llcoolbot_".to_string()))
            .await
            .unwrap();

        let _socket_reader = tokio::spawn(async move {
            read.for_each(|data| async {
                let msg = data.unwrap();
                if let protocol::Message::Text(text) = msg {
                    match TwitchMessage::from_str(&text).unwrap() {
                        TwitchMessage::RplWelcome => {
                            println!("Bot authenticated...");
                            tx_status.send(()).await.unwrap();
                        }
                        _ => println!("{:?}", text),
                    }
                }
            })
            .await;
        });

        let _http_handler_msg_reader = tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                match message {
                    BotMessage::JoinChat(channel) => {
                        println!("BOT JOIN CHANNEL {}", &channel);

                        write
                            .send(protocol::Message::Text(
                                format!("JOIN #{}", &channel).to_string(),
                            ))
                            .await
                            .unwrap();
                    }
                    BotMessage::LeaveChat(channel) => {
                        println!("BOT LEAVE CHANNEL {}", &channel);
                        write
                            .send(protocol::Message::Text(
                                format!("PART #{}", &channel).to_string(),
                            ))
                            .await
                            .unwrap();
                    }
                }
            }
        });

        if let Some(_) = rx_status.recv().await {
            self.status = BotStatus::Connected(tx.clone())
        }
    }
}
