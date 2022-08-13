use actix_session::SessionExt;
use actix_web::{web, HttpRequest};
use std::pin::Pin;

use actix_web::FromRequest;
use futures::Future;

use crate::{
    enums::session_key::SessionKey,
    errors::{AppError, AppErrorType},
    models::user_session::UserSession,
    states::app_config::AppConfig,
    twitch::id_api,
};

pub struct UserFromCookie {
    pub logged: UserSession,
}

impl FromRequest for UserFromCookie {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        Self::extract(req)
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        let request = req.clone();

        Box::pin(async move {
            let session = request.get_session();
            let app_config = request
                .app_data::<web::Data<AppConfig>>()
                .expect("Cannot get the app config!!!");

            let authentication_error = AppError::new(Some(AppErrorType::Unauthenticated));

            let mb_user_session = session
                .get::<UserSession>(&SessionKey::User.as_str())
                .map_err(|e| {
                    authentication_error
                        .clone()
                        .inner_error(&e.to_string())
                        .extra_context("Issue with the session.get method")
                })?;

            match mb_user_session {
                Some(user_session) => {
                    let is_valid = id_api::validate_user_token(&user_session.access_token)
                        .await
                        .map_err(|e| authentication_error.clone().inner_error(&e.to_string()))?;

                    if is_valid {
                        Ok(Self {
                            logged: user_session,
                        })
                    } else {
                        let new_user_data =
                            id_api::renew_token(&app_config, &user_session.refresh_token).await?;
                        let new_user_session = UserSession {
                            access_token: new_user_data.access_token,
                            refresh_token: new_user_data.refresh_token,
                            ..user_session
                        };

                        session
                            .insert(&SessionKey::User.as_str(), new_user_session.clone())
                            .map_err(|e| {
                                authentication_error
                                    .clone()
                                    .inner_error(&e.to_string())
                                    .extra_context("Cannot write new user into the session")
                            })?;

                        Ok(Self {
                            logged: new_user_session,
                        })
                    }
                }
                None => Err(authentication_error
                    .clone()
                    .extra_context("User not logged in")),
            }
        })
    }
}
