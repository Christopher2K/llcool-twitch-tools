use actix_session::Session;
use actix_web::{get, http::header, web, Error, HttpResponse};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

use crate::enums::session_key::SessionKey;
use crate::errors::{AppError, AppErrorType};
use crate::extractors::UserFromCookie;
use crate::models::user::{get_or_create_user, CreateUser};
use crate::models::user_session::UserSession;
use crate::states::app_config::AppConfig;
use crate::twitch::{api, id_api};
use crate::types::DbPool;

#[get("/login")]
pub async fn login_request_to_twitch(
    app_config: web::Data<AppConfig>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let oauth_state = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>();

    let authorization_url = id_api::get_authorization_url(&app_config, &oauth_state);
    session.insert(SessionKey::OAuthState.as_str(), &oauth_state)?;

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, authorization_url))
        .finish())
}

#[derive(Deserialize)]
pub struct AuthorizedData {
    pub code: String,
    pub scope: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthorizedError {
    pub error: String,
    pub error_description: String,
    pub state: String,
}

#[get("/login/authorized")]
pub async fn get_twitch_access_token(
    pool: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    session: Session,
    authorized_data: Option<web::Query<AuthorizedData>>,
    authorized_error: Option<web::Query<AuthorizedError>>,
) -> Result<HttpResponse, AppError> {
    let oauth_state = {
        let error = AppError::new(Some(AppErrorType::OAuthStateError))
            .extra_context("Cannot get the oauth state from the session");

        session
            .get::<String>(&SessionKey::OAuthState.as_str())
            .map_err(|_| error.clone())
            .and_then(|mb_state| match mb_state {
                Some(state) => Ok(state),
                None => Err(error.clone()),
            })
    }?;

    let db = pool
        .get()
        .map_err(|err| AppError::new(None).inner_error(&err.to_string()))?;

    let base_oauth_error = AppError::new(Some(AppErrorType::OAuthStateError));
    let base_twitch_request_error = AppError::new(Some(AppErrorType::TwitchApiError));

    if let Some(error) = authorized_error {
        let AuthorizedError {
            error_description, ..
        } = error.into_inner();
        Err(base_oauth_error.extra_context(&error_description))
    } else if let Some(query_data) = authorized_data {
        let AuthorizedData { code, state, .. } = query_data.into_inner();

        if oauth_state != state {
            return Err(base_oauth_error.extra_context("State does not match"));
        } else {
            let tokens = id_api::get_user_access_token(&app_config, &code)
                .await
                .map_err(|err| {
                    base_twitch_request_error
                        .clone()
                        .inner_error(&err.to_string())
                })?;

            let user_profile = api::get_current_user(&app_config, &tokens.access_token)
                .await
                .map_err(|err| {
                    base_twitch_request_error
                        .clone()
                        .inner_error(&err.to_string())
                })?;

            let db_user = get_or_create_user(
                &db,
                CreateUser {
                    username: user_profile.login,
                    twitch_id: user_profile.id,
                },
            )
            .map_err(|err| {
                AppError::new(Some(AppErrorType::DatabaseError))
                    .inner_error(&err.to_string())
                    .extra_context("Cannot create/get new twitch user")
            })?;

            let user_session =
                UserSession::new(&db_user, &tokens.access_token, &tokens.refresh_token);

            session.remove(&SessionKey::OAuthState.as_str());
            session
                .insert(SessionKey::User.as_str(), user_session)
                .map_err(|err| {
                    base_oauth_error
                        .inner_error(&err.to_string())
                        .extra_context("Cannot write user in the state")
                })?;

            Ok(HttpResponse::Found()
                .append_header((
                    header::LOCATION,
                    format!("{}", &app_config.frontend_url),
                ))
                .finish())
        }
    } else {
        Err(AppError::new(Some(AppErrorType::OAuthStateError))
            .extra_context("Twitch did not return data"))
    }
}

#[get("/logout")]
pub async fn logout(
    user: UserFromCookie,
    session: Session,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse, AppError> {
    session.remove(&SessionKey::User.as_str());
    id_api::revoke_token(&app_config, &user.session.access_token).await?;
    id_api::revoke_token(&app_config, &user.session.refresh_token).await?;

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, format!("{}", &app_config.frontend_url)))
        .finish())
}

#[get("/me")]
pub async fn me(user: UserFromCookie) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(user.logged))
}
