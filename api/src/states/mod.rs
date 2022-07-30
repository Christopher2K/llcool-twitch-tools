use std::sync::RwLock;

use time::ext::NumericalDuration;

use crate::twitch;

#[derive(Clone, Debug)]
pub struct TwitchClientCredentials{
    pub access_token: String,
    pub expire_at_utc: time::OffsetDateTime,
}

impl TwitchClientCredentials {
    pub async fn new() -> Self {
        let credentials = twitch::get_app_access_token()
            .await
            .expect("Can't get app credentials");

        Self {
            access_token: String::from(&credentials.access_token),
            expire_at_utc: time::OffsetDateTime::now_utc() + credentials.expires_in.seconds(),
        }
    }

    pub fn should_renew(&self) -> bool {
        let now = time::OffsetDateTime::now_utc();
        let limit = self.expire_at_utc - 1.days();
        now >= limit
    }
}

pub struct AppState {
    pub twitch_credentials: RwLock::<TwitchClientCredentials>
}
