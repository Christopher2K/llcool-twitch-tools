use serde::{Serialize, Deserialize};

use super::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub id: String,
    pub username: String,
    pub access_token: String,
}

impl UserSession {
    pub fn new(db_user: &User, access_token: &str) -> Self {
        Self {
            id: db_user.id.clone(),
            username: db_user.username.clone(),
            access_token: String::from(access_token),
        }
    }
}
