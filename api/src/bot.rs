use actix_web::web::Data;
use futures::{SinkExt, StreamExt, TryStreamExt};
use reqwest::Url;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol, MaybeTlsStream, WebSocketStream};

use crate::models::bot_credentials::get_bot_credentials_by_user_id;
use crate::models::user::get_user_by_username;
use crate::types::DbPool;

const WEBSOCKET_CLIENT_URL: &'static str = "wss://irc-ws.chat.twitch.tv:443";

type Sender = mpsc::Sender<BotMessage>;

#[derive(Debug)]
pub enum BotMessage {
    HelloWorld,
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

        // OPEN APP COMMUNICATION CHANNEL
        let channels = mpsc::channel::<BotMessage>(32);
        let (tx, mut rx) = channels;

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

        tokio::spawn(async move {
            read.for_each(|message| async {
                let data = message.unwrap();
                println!("{}", &data);
            })
            .await;
        });

        // let connected_status = BotStatus::Connected(tx.clone());

        // let handlers_receivers_task = tokio::spawn(async move {
        //     let (mut write, read) = ws_stream.split();
        //
        //     while let Some(message) = rx.recv().await {
        //         println!("Message: {:?}", &message);
        //     }
        // });

        // write.send(protocol::Message::Text(format!("PASS "))).await.expect("Cannot send PASS");
        // write
        //     .send(protocol::Message::Text("NICK llcoolbot_".to_string()))
        //     .await
        //     .expect("Cannot send NICK");
        //
        // read.for_each(|message| async {
        //     let data = message.unwrap();
        //     println!("{}", data);
        // })
        // .await;
    }

    pub async fn join_chat() {}

    pub async fn disconnect_from_chat(&self) -> () {}
}
