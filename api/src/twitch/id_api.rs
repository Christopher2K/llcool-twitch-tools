use futures::{FutureExt, TryFutureExt};
use pct_str::{PctString, URIReserved};

use std::{future::Future, marker::Send, pin::Pin};

use super::types;
use crate::states::app_config::AppConfig;

const ID_TWITCH_URL: &'static str = "https://id.twitch.tv";

pub fn get_authorization_url(app_config: &AppConfig, oauth_state: &str) -> String {
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

    let redirect_uri = format!("{}/api/auth/login/authorized", app_config.backend_url);

    String::from(format!(
            "{}/oauth2/authorize?response_type=code&client_id={}&state={}&nonce={}&redirect_uri={}&scope={}",
            ID_TWITCH_URL,
            app_config.client_id,
            &oauth_state,
            &oauth_state,
            redirect_uri,
            scope
        ))
}

pub fn get_app_access_token(
    app_config: &AppConfig,
) -> Pin<Box<dyn Future<Output = Result<types::TwitchTokenResponse, reqwest::Error>> + Send>> {
    let url = format!("{}/oauth2/token", ID_TWITCH_URL);

    let data = [
        ("client_id", app_config.client_id.clone()),
        ("client_secret", app_config.client_secret.clone()),
        ("grant_type", String::from("client_credentials")),
    ];

    let client = reqwest::Client::new();
    let future = client
        .post(url)
        .form(&data)
        .send()
        .and_then(|response| response.json::<types::TwitchTokenResponse>());

    Box::pin(future)
}

pub fn get_user_access_token(
    app_config: &AppConfig,
    code: &str,
) -> Pin<Box<dyn Future<Output = Result<types::TwitchTokenWithRefreshResponse, reqwest::Error>> + Send>> {
    let url = format!("{}/oauth2/token", ID_TWITCH_URL);
    let redirect_uri = format!("{}/api/auth/login/authorized", app_config.backend_url);
    let data = [
        ("client_id", app_config.client_id.clone()),
        ("client_secret", app_config.client_secret.clone()),
        ("grant_type", String::from("authorization_code")),
        ("code", String::from(code)),
        ("redirect_uri", redirect_uri),
    ];

    let client = reqwest::Client::new();

    client
        .post(url)
        .form(&data)
        .send()
        .and_then(|response| response.json::<types::TwitchTokenWithRefreshResponse>())
        .boxed()
}
