use std::sync::Arc;

use dashmap::DashMap;
use log::{info, error};
use tokio::sync::{mpsc, watch};
use twitch_irc::message::PrivmsgMessage;

use super::types::BotExternalAction;

pub type ChannelRegistry = DashMap<ChannelName, ChannelHandler>;
pub type ChannelName = String;

pub struct ChannelHandler {
    channel_name: String,
    messsage_receiver: watch::Receiver<Option<PrivmsgMessage>>,
    bot_external_action_sender: mpsc::Sender<BotExternalAction>,
    pub channel_registry: Arc<ChannelRegistry>,
}

impl ChannelHandler {
    pub fn new(
        channel_name: String,
        channel_registry: Arc<ChannelRegistry>,
        messsage_receiver: watch::Receiver<Option<PrivmsgMessage>>,
        bot_external_action_sender: mpsc::Sender<BotExternalAction>,
    ) -> Self {
        Self {
            channel_name,
            channel_registry,
            messsage_receiver,
            bot_external_action_sender,
        }
    }

    pub fn run(&self) {
        let name = self.channel_name.clone();
        let mut receiver = self.messsage_receiver.clone();
        let external_action_sender = self.bot_external_action_sender.clone();
        let log_target = format!("twitch_bot::channel::{}", &name);

        tokio::spawn(async move {
            while receiver.changed().await.is_ok() {
                let msg = {
                    let option = &*receiver.borrow();
                    option.clone()
                };

                if let Some(msg) = msg {
                    if msg.channel_login == name {
                        // Do something
                        info!(target: &log_target, "Handled message: {}", msg.message_text);

                        let send_external_message = external_action_sender.send(BotExternalAction::Respond {
                            channel_name: String::from(&msg.channel_login),
                            message: format!(
                                "Responding to {} - they said {}",
                                &msg.sender.name, &msg.message_text
                            ),
                        }).await;

                        if let Err(e) = send_external_message {
                            error!(target: &log_target, "Cannot send message in {} : {:?}", &name, e);
                        };
                    } 
                }
            }
        });
    }
}
