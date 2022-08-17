use actix_web::{web, FromRequest};
use futures::Future;
use std::pin::Pin;

use crate::{
    errors::{AppError, AppErrorType},
    states::app_config::AppConfig,
    states::twitch_credentials::{ThreadSafeTwitchClientCredentials, TwitchClientCredentials},
};

const LOG_TARGET: &'static str = "actix_web::extractors::twitch_app_token";

pub struct TwitchAppToken {
    pub value: String,
}

impl FromRequest for TwitchAppToken {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        Self::extract(req)
    }

    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        let request = req.clone();

        Box::pin(async move {
            let lock = request
                .app_data::<web::Data<ThreadSafeTwitchClientCredentials>>()
                .expect("Cannot get ThreadSafeTwitchClientCredentials state");

            let app_config = request
                .app_data::<web::Data<AppConfig>>()
                .expect("Cannot get the app config!!!");

            let (current_token, should_renew) = {
                let credentials = lock.read().map_err(|e| {
                    AppError::new(Some(AppErrorType::AppStateError)).inner_error(&e.to_string())
                })?;

                (credentials.access_token.clone(), credentials.should_renew())
            };

            if should_renew {
                log::info!(target: LOG_TARGET, "Renewing app twitch credentials...");

                let new_credentials = TwitchClientCredentials::new(&app_config).await;

                log::info!(target: LOG_TARGET, "Set new credentials in actix state...");

                let mut credentials = lock.write().map_err(|e| {
                    AppError::new(Some(AppErrorType::AppStateError)).inner_error(&e.to_string())
                })?;

                *credentials = new_credentials;
                log::info!(target: LOG_TARGET, "Success!");

                Ok(Self {
                    value: credentials.access_token.clone(),
                })
            } else {
                Ok(Self {
                    value: current_token,
                })
            }
        })
    }
}
