use actix_session::SessionExt;
use actix_web::HttpRequest;
use std::pin::Pin;

use actix_web::FromRequest;
use futures::Future;

use crate::{
    enums::session_key::SessionKey,
    errors::{AppError, AppErrorType},
    models::user_session::UserSession,
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
                    // TODO: Check is twitch token is still valid
                    // If not, try to renew it
                    // else, throw authorization error

                    Ok(Self {
                        logged: user_session,
                    })
                }
                None => Err(authentication_error
                    .clone()
                    .extra_context("User not logged in")),
            }
        })
    }
}
