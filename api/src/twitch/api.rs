use reqwest::{header, Client};

use crate::{states::app_config::AppConfig, twitch::types};

const TWITCH_API_URL: &'static str = "https://api.twitch.tv";
const TWITCH_API_VERSION: &'static str = "helix";

fn get_ressource_url(path: &str) -> String {
    format!("{}/{}{}", TWITCH_API_URL, TWITCH_API_VERSION, path)
}

fn get_request_base_headers(app_config: &AppConfig, access_token: &str) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    let formatted_authorization = format!("Bearer {}", access_token);
    headers.insert(
        header::AUTHORIZATION,
        formatted_authorization.parse().unwrap(),
    );
    headers.insert(
        "Client-Id",
        String::from(&app_config.client_id).parse().unwrap(),
    );

    headers
}

pub async fn get_current_user(
    app_config: &AppConfig,
    access_token: &str,
) -> Result<types::User, reqwest::Error> {
    let users_url = get_ressource_url("/users");
    let client = Client::new();
    let headers = get_request_base_headers(app_config, access_token);

    let users = client
        .get(users_url)
        .headers(headers)
        .send()
        .await?
        .json::<types::UsersResponse>()
        .await?;

    let user = users
        .data
        .first()
        .expect("Current user does not exists")
        .clone();

    Ok(user)

}
