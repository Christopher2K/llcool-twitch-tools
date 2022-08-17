use actix_session::SessionExt;
use actix_web::{FromRequest, web, HttpRequest};
use std::pin::Pin;

use futures::Future;

use crate::{
    enums::session_key::SessionKey,
    errors::{AppError, AppErrorType},
    models::user::User,
    models::{user::get_user_by_username, user_session::UserSession},
    states::app_config::AppConfig,
    twitch::id_api,
    types::DbPool,
};

pub struct UserFromCookie {
    pub session: UserSession,
    pub logged: User,
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

            let db = request
                .app_data::<web::Data<DbPool>>()
                .expect("Cannot get db pool config")
                .get()
                .map_err(|err| AppError::new(None).inner_error(&err.to_string()))?;

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

                    let db_user =
                        get_user_by_username(&db, &user_session.username).map_err(|e| {
                            AppError::new(Some(AppErrorType::DatabaseError))
                                .extra_context("Cannot get user db record")
                        })?;

                    if is_valid {
                        Ok(Self {
                            session: user_session,
                            logged: db_user,
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
