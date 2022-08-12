use reqwest::{header, Client};

use crate::twitch::types;

const TWITCH_API_URL: &'static str = "https://api.twitch.tv";

fn get_ressource_url(path: &str) -> String {
    format!("{}{}", TWITCH_API_URL, path)
}

fn get_request_base_headers(client_id: &str, access_token: &str) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    let formatted_authorization = format!("Bearer {}", access_token);
    headers.insert(
        header::AUTHORIZATION,
        formatted_authorization.parse().unwrap(),
    );
    headers.insert("Client-Id", client_id.parse().unwrap());

    headers
}

pub async fn get_current_user(
    client_id: &str,
    access_token: &str,
) -> Result<types::User, reqwest::Error> {
    let users_url = get_ressource_url("/users");
    let client = Client::new();
    let headers = get_request_base_headers(client_id, access_token);

    let user = client
        .get(users_url)
        .headers(headers)
        .send()
        .await?
        .json::<types::User>()
        .await?;

    Ok(user)
}
