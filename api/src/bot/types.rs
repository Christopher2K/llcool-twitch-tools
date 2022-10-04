#[derive(Debug)]
pub enum BotExternalAction {
    Pong,
    Respond {
        channel_name: String,
        message: String,
    },
    Join {
        channel_name: String,
        user_id: Option<uuid::Uuid>,
    },
    Leave(String),
}
