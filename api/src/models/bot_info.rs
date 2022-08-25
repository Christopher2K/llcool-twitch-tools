use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CredentialsState {
    Invalid,
    Valid,
    NotFound,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BotInfo {
    pub name: String,
    pub connected: bool,
    pub credentials_state: CredentialsState,
    pub connected_to_chat: bool,
}
