use std::env;

#[derive(Clone)]
pub enum AppEnv {
    Prod,
    Staging,
    Local,
}

impl From<String> for AppEnv {
    fn from(pattern: String) -> Self {
        match pattern.as_str() {
            "prod" => Self::Prod,
            "staging" => Self::Staging,
            "local" => Self::Local,
            _ => Self::Local,
        }
    }
}

#[derive(Clone)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub backend_url: String,
    pub frontend_url: String,
    pub chat_bot_username: String,
    pub domain: String,
    pub app_env: AppEnv,
}

impl AppConfig {
    pub fn new() -> Result<Self, env::VarError> {
        Ok(Self {
            client_id: env::var("TWITCH_CLIENT_ID")?,
            client_secret: env::var("TWITCH_CLIENT_SECRET")?,
            backend_url: env::var("BACKEND_URL")?,
            frontend_url: env::var("FRONTEND_URL")?,
            chat_bot_username: env::var("CHAT_BOT_USERNAME")?,
            domain: env::var("DOMAIN")?,
            app_env: AppEnv::from(env::var("ENV").unwrap_or(String::from("local"))),
        })
    }
}
