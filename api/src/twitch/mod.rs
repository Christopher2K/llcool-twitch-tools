use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use std::{env, future::Future, marker::Send, pin::Pin};

struct TwitchApiConfig {
    client_id: String,
    client_secret: String,
}

impl TwitchApiConfig {
    pub fn new() -> Result<Self, &'static str> {
        let config_data = {
            (
                env::var("TWITCH_CLIENT_ID"),
                env::var("TWITCH_CLIENT_SECRET"),
            )
        };

        match config_data {
            (Ok(client_id), Ok(client_secret)) => Ok(Self {
                client_id,
                client_secret,
            }),
            (Err(_), _) => Err("TWITCH_CLIENT_ID env var is missing"),
            (_, Err(_)) => Err("TWITCH_CLIENT_SECRET env var is missing"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ClientCredentialGrantFlowResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

#[derive(Serialize)]
struct ClientCredentialGrantFlowRequest {
    client_id: String,
    client_secret: String,
    grant_type: String,
}

pub fn get_app_access_token(
) -> Pin<Box<dyn Future<Output = Result<ClientCredentialGrantFlowResponse, reqwest::Error>> + Send>>
{
    let url = "https://id.twitch.tv/oauth2/token";
    let config = TwitchApiConfig::new().expect("Missing env variables");

    let data = [
        ("client_id", config.client_id.clone()),
        ("client_secret", config.client_secret.clone()),
        ("grant_type", String::from("client_credentials")),
    ];

    let client = reqwest::Client::new();
    let future = client
        .post(url)
        .form(&data)
        .send()
        .and_then(|response| response.json::<ClientCredentialGrantFlowResponse>());

    Box::pin(future)
}
