use std::env::{self, VarError};

pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub backend_url: String,
    pub frontend_url: String,
    pub chat_bot_username: String,
    pub app_env: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, VarError> {
        let (client_id, client_secret, backend_url, frontend_url, chat_bot_username, app_env) = {
            (
                env::var("TWITCH_CLIENT_ID")?,
                env::var("TWITCH_CLIENT_SECRET")?,
                env::var("BACKEND_URL")?,
                env::var("FRONTEND_URL")?,
                env::var("CHAT_BOT_USERNAME")?,
                env::var("ENV")?,
            )
        };

        Ok(Self {
            client_id,
            client_secret,
            backend_url,
            frontend_url,
            chat_bot_username,
            app_env,
        })
    }
}
