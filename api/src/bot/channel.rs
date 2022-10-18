use std::sync::Arc;

use dashmap::DashMap;
use log::{error, info};
use tokio::sync::{mpsc, oneshot, watch};
use twitch_irc::message::PrivmsgMessage;

use crate::models::v2;

use super::types::BotExternalAction;

pub type ChannelDef = (ChannelHandler, oneshot::Sender<()>);
pub type ChannelRegistry = DashMap<ChannelName, ChannelDef>;
pub type ChannelName = String;

pub struct ChannelHandler {
    channel_name: String,
    pub commands: Vec<v2::UserCommand>,
    messsage_receiver: watch::Receiver<Option<PrivmsgMessage>>,
    bot_external_action_sender: mpsc::Sender<BotExternalAction>,
    pub channel_registry: Arc<ChannelRegistry>,
}

impl ChannelHandler {
    pub fn new(
        channel_name: String,
        commands: Vec<v2::UserCommand>,
        channel_registry: Arc<ChannelRegistry>,
        messsage_receiver: watch::Receiver<Option<PrivmsgMessage>>,
        bot_external_action_sender: mpsc::Sender<BotExternalAction>,
    ) -> Self {
        Self {
            channel_name,
            commands,
            channel_registry,
            messsage_receiver,
            bot_external_action_sender,
        }
    }
    pub fn run(&self) -> oneshot::Sender<()> {
        let name = self.channel_name.clone();
        let mut receiver = self.messsage_receiver.clone();
        let external_action_sender = self.bot_external_action_sender.clone();
        let log_target = format!("twitch_bot::channel::{}", &name);
        let commands = self.commands.clone();

        let (tx_kill_sig, mut rx_kill_sig) = oneshot::channel::<()>();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Ok(_) = receiver.changed() => {
                        let msg = {
                            let opt = &*receiver.borrow();
                            opt.clone()
                        };

                        if let Some(msg) = msg {
                            if msg.channel_login == name {
                                // Do something
                                info!(target: &log_target, "Handled message: {}", msg.message_text);

                                for command in commands.iter() {
                                    if format!("!{}", command.name) == msg.message_text {
                                        let send_external_message = external_action_sender
                                            .send(BotExternalAction::Respond {
                                                channel_name: String::from(&msg.channel_login),
                                                message: format!("{}", command.message),
                                            })
                                        .await;

                                        if let Err(e) = send_external_message {
                                            error!(
                                                target: &log_target,
                                                "Cannot send message in {} : {:?}", &name, e
                                                );
                                        };
                                    }
                                }
                            }
                        }
                    }
                    Ok(()) = &mut rx_kill_sig => {
                        break;
                    }
                }
            }
        });

        tx_kill_sig
    }
}
