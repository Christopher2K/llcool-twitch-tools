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
        let (
            client_id,
            client_secret,
            backend_url,
            frontend_url,
            chat_bot_username,
            domain,
            app_env,
        ) = {
            (
                env::var("TWITCH_CLIENT_ID")?,
                env::var("TWITCH_CLIENT_SECRET")?,
                env::var("BACKEND_URL")?,
                env::var("FRONTEND_URL")?,
                env::var("CHAT_BOT_USERNAME")?,
                env::var("DOMAIN")?,
                env::var("ENV").unwrap_or(String::from("local")),
            )
        };

        Ok(Self {
            client_id,
            client_secret,
            backend_url,
            frontend_url,
            chat_bot_username,
            domain,
            app_env: AppEnv::from(app_env),
        })
    }
}
