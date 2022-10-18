use std::pin::Pin;

use actix_session::SessionExt;
use actix_web::{web, FromRequest, HttpRequest};
use futures::Future;
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;

use crate::enums::session_key::SessionKey;
use crate::errors::{AppError, AppErrorType};
use crate::models::v2;
use crate::models::user_session::UserSession;
use crate::states::app_config::AppConfig;
use crate::twitch::id_api;

const LOG_TARGET: &'static str = "actix_web::extractors::user_from_cookie";

pub struct UserFromCookie {
    pub session: UserSession,
    pub logged: v2::User,
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
            log::info!(target: LOG_TARGET, "Getting user info...");

            let session = request.get_session();
            let app_config = request
                .app_data::<web::Data<AppConfig>>()
                .expect("Cannot get the app config!!!");

            let pool = request.app_data::<web::Data<Pool<Postgres>>>().unwrap();

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
                    log::info!(target: LOG_TARGET, "Checking user validity...");

                    let is_valid = {
                        let now = OffsetDateTime::now_utc();
                        user_session.expire_at > now.unix_timestamp()
                    };

                    log::info!(target: LOG_TARGET, "Getting db user...");

                    let user_session_clone = user_session.clone();

                    let db_user = v2::User::get_by_username(pool, &user_session_clone.username)
                        .await?
                        .ok_or(
                            AppError::from(AppErrorType::DatabaseError)
                                .extra_context("Cannot get user db record"),
                        )?;

                    if is_valid {
                        log::info!(target: LOG_TARGET, "User is valid, proceed...");

                        Ok(Self {
                            session: user_session,
                            logged: db_user,
                        })
                    } else {
                        log::info!(
                            target: LOG_TARGET,
                            "User is invalid, renew token attempt..."
                        );

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

                        log::info!(target: LOG_TARGET, "Token renew success, proceed...");
                        Ok(Self {
                            session: new_user_session,
                            logged: db_user,
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
