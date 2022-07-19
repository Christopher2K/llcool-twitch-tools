use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub enum TwitchCredentialsGuardErrors {
    CredentialsMissing,
}

pub struct TwitchCredentialsGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TwitchCredentialsGuard {
    type Error = TwitchCredentialsGuardErrors;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("[TwichCredentialsGuard] - Checking credentials validity");
        let maybe_credentials = request
            .rocket()
            .state::<std::sync::RwLock<crate::states::TwitchClientCredentialsState>>();

        match maybe_credentials {
            Some(lock) => {
                let should_renew = {
                    let credentials = lock.read().unwrap();
                    credentials.should_renew()
                };

                if should_renew {
                    println!("[TwichCredentialsGuard] - Renew credentials");
                    let new_credentials = crate::states::TwitchClientCredentialsState::new().await;
                    let mut credentials = lock.write().unwrap();
                    *credentials = new_credentials;
                } else {
                    println!("[TwichCredentialsGuard] - Credentials are still valid");
                }

                Outcome::Success(TwitchCredentialsGuard)
            }
            None => Outcome::Failure((
                Status::InternalServerError,
                TwitchCredentialsGuardErrors::CredentialsMissing,
            )),
        }
    }
}
