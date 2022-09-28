#[derive(Debug)]
pub enum BotExternalAction {
    Pong,
    Respond {
        channel_name: String,
        message: String,
    },
    Join(String),
    Leave(String),
}
