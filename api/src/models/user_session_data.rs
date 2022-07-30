use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSessionData {
    pub oauth_state: Option<String>
}
