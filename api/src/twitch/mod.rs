use futures::{FutureExt, TryFutureExt};
use pct_str::{PctString, URIReserved};
use serde::{Deserialize, Serialize};

use std::{env, future::Future, marker::Send, pin::Pin};

pub struct TwitchApiConfig {
    client_id: String,
    client_secret: String,
    backend_url: String,
}

impl TwitchApiConfig {
    const ID_TWITCH_URL: &'static str = "https://id.twitch.tv";

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

    pub fn get_authorization_url(&self, state: &str) -> String {
        // TODO: Enum for this please
        let scope = PctString::encode(
            [
                "chat:read",
                "chat:edit",
                "bits:read",
                "channel:read:subscriptions",
                "user:edit",
            ]
            .join(" ")
            .chars(),
            URIReserved,
        );

        let redirect_uri = format!("{}/api/auth/login/authorized", self.backend_url);

        String::from(format!(
            "{}/oauth2/authorize?response_type=code&client_id={}&state={}&nonce={}&redirect_uri={}&scope={}",
            Self::ID_TWITCH_URL,
            self.client_id,
            state,
            state,
            redirect_uri,
            scope
        ))
    }

    pub fn get_app_access_token(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<TwitchTokenResponse, reqwest::Error>> + Send>> {
        let url = format!("{}/oauth2/token", Self::ID_TWITCH_URL);

        let data = [
            ("client_id", self.client_id.clone()),
            ("client_secret", self.client_secret.clone()),
            ("grant_type", String::from("client_credentials")),
        ];

        let client = reqwest::Client::new();
        let future = client
            .post(url)
            .form(&data)
            .send()
            .and_then(|response| response.json::<TwitchTokenResponse>());

        Box::pin(future)
    }

    pub fn get_user_access_token(
        &self,
        code: &str,
    ) -> Pin<Box<dyn Future<Output = Result<TwitchTokenWithRefreshResponse, reqwest::Error>> + Send>>
    {
        let url = format!("{}/oauth2/token", Self::ID_TWITCH_URL);
        let redirect_uri = format!("{}/api/auth/login/authorized", self.backend_url);
        let data = [
            ("client_id", self.client_id.clone()),
            ("client_secret", self.client_secret.clone()),
            ("grant_type", String::from("authorization_code")),
            ("code", String::from(code)),
            ("redirect_uri", redirect_uri),
        ];

        let client = reqwest::Client::new();

        client
            .post(url)
            .form(&data)
            .send()
            .and_then(|response| response.json::<TwitchTokenWithRefreshResponse>())
            .boxed()
    }
}

#[derive(Deserialize, Debug)]
pub struct TwitchTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TwitchTokenWithRefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}
