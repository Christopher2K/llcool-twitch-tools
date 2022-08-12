use std::env;

pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub backend_url: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, &'static str> {
        let config_data = {
            (
                env::var("TWITCH_CLIENT_ID"),
                env::var("TWITCH_CLIENT_SECRET"),
                env::var("BACKEND_URL"),
            )
        };

        match config_data {
            (Ok(client_id), Ok(client_secret), Ok(backend_url)) => Ok(Self {
                client_id,
                client_secret,
                backend_url,
            }),
            (Err(_), _, _) => Err("TWITCH_CLIENT_ID env var is missing"),
            (_, Err(_), _) => Err("TWITCH_CLIENT_SECRET env var is missing"),
            (_, _, Err(_)) => Err("BACKEND_URL env var is missing"),
        }
    }
}
