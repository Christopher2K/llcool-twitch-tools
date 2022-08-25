use actix_web::{get, web, HttpResponse};
use diesel::OptionalExtension;

use crate::bot::{Bot, BotMessage, BotStatus};
use crate::errors::*;
use crate::extractors::user_from_cookie::UserFromCookie;
use crate::models::bot_credentials::get_bot_credentials_by_user_id;
use crate::models::bot_info::{BotInfo, CredentialsState};
use crate::models::user::get_user_by_username;
use crate::states::app_config::AppConfig;
use crate::twitch::id_api::validate_user_token;
use crate::types::DbPool;

#[get("/info")]
pub async fn get_bot_info(
    user: UserFromCookie,
    db: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    bot: web::Data<Bot>,
) -> Result<HttpResponse, AppError> {
    // TODO: In the current user chat
    let db = db
        .get()
        .map_err(|e| AppError::from(AppErrorType::DatabaseError).inner_error(&e.to_string()))?;
    let name = app_config.chat_bot_username.clone();
    let connected = matches!(bot.status(), BotStatus::Connected(_));

    let mb_credentials = get_user_by_username(&db, &name)
        .optional()
        .and_then(|mb_bot_user| {
            if let Some(bot_user) = mb_bot_user {
                get_bot_credentials_by_user_id(&db, &bot_user.id.clone()).optional()
            } else {
                Ok(None)
            }
        })
        .map_err(|e| AppError::from(AppErrorType::DatabaseError).inner_error(&e.to_string()))?;

    let credentials_state = match mb_credentials {
        Some(credentials) => {
            let token_is_valid = validate_user_token(&credentials.access_token)
                .await
                .map_err(|e| {
                    AppError::from(AppErrorType::InternalError).inner_error(&e.to_string())
                })?;

            if token_is_valid {
                CredentialsState::Valid
            } else {
                CredentialsState::Invalid
            }
        }
        None => CredentialsState::NotFound,
    };

    let connected_to_chat = {
        let connected_channels = bot.connected_channels.clone();

        let lock = connected_channels
            .lock()
            .map_err(|e| AppError::from(AppErrorType::InternalError).inner_error(&e.to_string()))?;
        lock.contains(&user.logged.username)
    };

    Ok(HttpResponse::Ok().json(BotInfo {
        name,
        credentials_state,
        connected,
        connected_to_chat
    }))
}

#[get("/join")]
pub async fn join_chat(
    user: UserFromCookie,
    bot: web::Data<Bot>,
) -> Result<HttpResponse, AppError> {
    if let BotStatus::Connected(sender) = &bot.status() {
        let sender = sender.clone();
        sender
            .send(BotMessage::JoinChat(user.logged.username.clone()))
            .await
            .unwrap();
    }

    Ok(HttpResponse::Ok().finish())
}

#[get("/leave")]
pub async fn leave_chat(
    user: UserFromCookie,
    bot: web::Data<Bot>,
) -> Result<HttpResponse, AppError> {
    if let BotStatus::Connected(sender) = &bot.status() {
        let sender = sender.clone();
        sender
            .send(BotMessage::LeaveChat(user.logged.username.clone()))
            .await
            .unwrap();
    }

    Ok(HttpResponse::Ok().finish())
}
