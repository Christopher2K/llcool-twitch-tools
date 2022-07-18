use time::ext::NumericalDuration;

use crate::twitch;

pub struct TwitchClientCredentialsState {
    pub access_token: String,
    pub expire_at_utc: time::OffsetDateTime,
}

impl TwitchClientCredentialsState {
    pub async fn new() -> Self {
        let credentials = twitch::get_app_access_token()
            .await
            .expect("Can't get app credentials");

        Self {
            access_token: String::from(&credentials.access_token),
            expire_at_utc: time::OffsetDateTime::now_utc() + credentials.expires_in.seconds()
            }
    }

    pub fn should_renew(&self) -> bool {
        let now = time::OffsetDateTime::now_utc();
        let limit = self.expire_at_utc - 1.days();
        now >= limit
    }

    pub async fn renew_client_credentials(&mut self) -> () {
        match twitch::get_app_access_token().await {
            Ok(new_credentials) => {
                self.access_token = String::from(new_credentials.access_token);
                self.expire_at_utc = time::OffsetDateTime::now_utc() + new_credentials.expires_in.seconds();
            },
            Err(e) => {
                // Log error
            }
        }
    }
}
