use std::future::{ready, Ready};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use futures::future::LocalBoxFuture;
use futures::FutureExt;

use crate::states;

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
        let mb_lock = req.app_data::<Data<states::AppState>>().cloned();
        let fut = self.service.call(req);

        Box::pin(async move {
            match mb_lock {
                Some(lock) => {
                    // Can read the state
                    let should_renew = {
                        let credentials = lock.twitch_credentials.read().unwrap();
                        credentials.should_renew()
                    };

                    if should_renew {
                        let new_credentials = states::TwitchClientCredentials::new().await;
                        let mut credentials = lock.twitch_credentials.write().unwrap();
                        *credentials = new_credentials;
                    };

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
