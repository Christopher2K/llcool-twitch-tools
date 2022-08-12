use std::future::{ready, Ready};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use futures::future::LocalBoxFuture;

use crate::states::twitch_credentials::{ThreadSafeTwitchClientCredentials, TwitchClientCredentials};
use crate::states::app_config::AppConfig;

const LOG_TARGET: &'static str = "actix_web::middlewares::twitch_client_credentials";

pub struct TwitchClientCredentialsMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TwitchClientCredentialsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mb_lock = req.app_data::<Data<ThreadSafeTwitchClientCredentials>>().cloned();
        let app_config = req.app_data::<Data<AppConfig>>().cloned().expect("Cannot read app configuration");
        let fut = self.service.call(req);

        Box::pin(async move {
            match mb_lock {
                Some(lock) => {
                    // Can read the state
                    log::info!(
                        target: LOG_TARGET,
                        "Checking if credentials are still valid"
                    );
                    let should_renew = {
                        let credentials = lock.read().unwrap();
                        credentials.should_renew()
                    };

                    if should_renew {
                        log::info!(target: LOG_TARGET, "Renewing app twitch credentials...");
                        let new_credentials = TwitchClientCredentials::new(&app_config).await;
                        log::info!(
                            target: LOG_TARGET,
                            "Setting new credentials in actix state..."
                        );
                        let mut credentials = lock.write().unwrap();
                        *credentials = new_credentials;
                        log::info!(target: LOG_TARGET, "Proceed to request...");
                    };

                    if !should_renew {
                        log::info!(
                            target: LOG_TARGET,
                            "Credentials are still valid, proceed..."
                        );
                    }

                    let res = fut.await?;
                    Ok(res)
                }
                None => Err(actix_web::error::ErrorBadRequest("")),
            }
        })
    }
}

pub struct TwitchClientCredentialsMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for TwitchClientCredentialsMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = TwitchClientCredentialsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TwitchClientCredentialsMiddleware { service }))
    }
}
