use actix_session::Session;
use actix_web::{get, http::header, web, Error, HttpResponse};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::enums::session_key::SessionKey;
use crate::errors::{AppError, AppErrorType};
use crate::extractors::user_from_cookie::UserFromCookie;
use crate::models;
use crate::states::app_config::AppConfig;
use crate::twitch::{api, id_api};

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
    pool: web::Data<Pool<Postgres>>,
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

            let db_user = {
                let mb_user = models::User::get_by_username(&pool, &user_profile.login).await?;

                match mb_user {
                    Some(user) => user,
                    None => {
                        models::User::create(
                            &pool,
                            &models::CreateUser {
                                username: &user_profile.login,
                                twitch_id: &user_profile.id,
                            },
                        )
                        .await?
                    }
                }
            };

            if user_profile.login == app_config.chat_bot_username {
                let data = models::CreateBotCredentials {
                    access: &tokens.access_token,
                    refresh: &tokens.refresh_token,
                    user_id: &db_user.id,
                };

                let mb_updated_credentials =
                    models::BotCredentials::update_by_user_id(&pool, &data).await?;
                match mb_updated_credentials {
                    Some(updated_credentials) => updated_credentials,
                    None => models::BotCredentials::create(&pool, &data).await?,
                };
            }

            let user_session = models::UserSession::new(
                &db_user,
                &tokens.access_token,
                &tokens.refresh_token,
                tokens.expires_in,
            );

            session.remove(&SessionKey::OAuthState.as_str());
            session
                .insert(SessionKey::User.as_str(), user_session)
                .map_err(|err| {
                    base_oauth_error
                        .inner_error(&err.to_string())
                        .extra_context("Cannot write user in the state")
                })?;

            Ok(HttpResponse::Found()
                .append_header((header::LOCATION, format!("{}", &app_config.frontend_url)))
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

    if user.logged.username != app_config.chat_bot_username {
        id_api::revoke_token(&app_config, &user.session.access_token).await?;
        id_api::revoke_token(&app_config, &user.session.refresh_token).await?;
    }

    Ok(HttpResponse::Found()
        .append_header((header::LOCATION, format!("{}", &app_config.frontend_url)))
        .finish())
}

#[get("/me")]
pub async fn me(user: UserFromCookie) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(user.logged))
}
