use serde::{Deserialize, Serialize};
use time::ext::NumericalDuration;
use uuid::Uuid;

use super::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSession {
    pub id: Uuid,
    pub username: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_at: i64,
}

impl UserSession {
    pub fn new(db_user: &User, access_token: &str, refresh_token: &str, expire_in: i64) -> Self {
        let now = time::OffsetDateTime::now_utc();
        let expire_at = {
            let future_date = now + expire_in.seconds();
            future_date.unix_timestamp()
        };

        Self {
            id: db_user.id.clone(),
            username: db_user.username.clone(),
            access_token: String::from(access_token),
            refresh_token: String::from(refresh_token),
            expire_at,
        }
    }
}
