use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSession {
    pub id: Uuid,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
}

impl UserSession {
    pub fn new(db_user: &User, access_token: &str, refresh_token: &str) -> Self {
        Self {
            id: db_user.id.clone(),
            username: db_user.username.clone(),
            access_token: String::from(access_token),
            refresh_token: String::from(refresh_token),
        }
    }
}
