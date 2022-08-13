use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct TwitchDataResponse<T> {
    pub data: Vec<T>
}

#[derive(Deserialize, Debug)]
pub struct TwitchTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TwitchTokenWithRefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: u64,
    // pub email: String,
    pub created_at: String, // TODO: Cast this one in a Datetime Object
}

pub type UsersResponse = TwitchDataResponse<User>;
