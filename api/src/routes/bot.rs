use actix_web::{get, patch, web, HttpResponse};
use diesel::OptionalExtension;
use std::sync::RwLock;

use crate::bot::{Bot, BotMessage, BotStatus};
use crate::errors::*;
use crate::extractors::user_from_cookie::UserFromCookie;
use crate::models::bot_credentials::{
    create_bot_credentials, get_bot_credentials_by_user_id, CreateBotCredentials,
};
use crate::models::bot_info::{BotInfo, CredentialsState};
use crate::models::user::get_user_by_username;
use crate::states::app_config::{self, AppConfig};
use crate::twitch::id_api::validate_user_token;
use crate::types::DbPool;

#[get("/info")]
pub async fn get_bot_info(
    user: UserFromCookie,
    db: web::Data<DbPool>,
    app_config: web::Data<AppConfig>,
    bot: web::Data<RwLock<Bot>>,
) -> Result<HttpResponse, AppError> {
    let bot = bot
        .read()
        .map_err(|e| AppError::from(AppErrorType::InternalError).inner_error(&e.to_string()))?;

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
        connected_to_chat,
    }))
}

#[get("/join")]
pub async fn join_chat(
    user: UserFromCookie,
    bot: web::Data<RwLock<Bot>>,
) -> Result<HttpResponse, AppError> {
    let bot = bot
        .read()
        .map_err(|e| AppError::from(AppErrorType::InternalError).inner_error(&e.to_string()))?;

    match &bot.status() {
        BotStatus::Connected(sender) => {
            sender
                .send(BotMessage::JoinChat(user.logged.username.clone()))
                .await
                .unwrap();

            Ok(HttpResponse::Ok().finish())
        }
        _ => Err(AppError::from(AppErrorType::BotDisconnected)),
    }
}

#[get("/leave")]
pub async fn leave_chat(
    user: UserFromCookie,
    bot: web::Data<RwLock<Bot>>,
) -> Result<HttpResponse, AppError> {
    let bot = bot
        .read()
        .map_err(|e| AppError::from(AppErrorType::InternalError).inner_error(&e.to_string()))?;

    match &bot.status() {
        BotStatus::Connected(sender) => {
            sender
                .send(BotMessage::LeaveChat(user.logged.username.clone()))
                .await
                .unwrap();

            Ok(HttpResponse::Ok().finish())
        }
        _ => Err(AppError::from(AppErrorType::BotDisconnected)),
    }
}

#[get("/connect")]
pub async fn connect(
    user: UserFromCookie,
    app_config: web::Data<AppConfig>,
    db: web::Data<DbPool>,
    bot: web::Data<RwLock<Bot>>,
) -> Result<HttpResponse, AppError> {
    let db = db
        .get()
        .map_err(|e| AppError::from(AppErrorType::DatabaseError).inner_error(&e.to_string()))?;

    if user.logged.username != app_config.chat_bot_username {
        Err(AppError::from(AppErrorType::Unauthorized))
    } else {
        let mut bot = bot
            .write()
            .map_err(|e| AppError::from(AppErrorType::InternalError).inner_error(&e.to_string()))?;

        match &bot.status() {
            BotStatus::Connected(_) => Ok(HttpResponse::Ok().finish()),
            BotStatus::Disconnected => {
                // Ensure that bot credentials does exist
                let mb_credentials = get_bot_credentials_by_user_id(&db, &user.logged.id);
                let credentials_missing =
                    matches!(mb_credentials, Err(diesel::result::Error::NotFound));

                if credentials_missing {
                    create_bot_credentials(
                        &db,
                        &CreateBotCredentials {
                            access_token: user.session.access_token.clone(),
                            refresh_token: user.session.refresh_token.clone(),
                            user_id: user.logged.id.clone(),
                        },
                    )
                    .map_err(|e| {
                        AppError::from(AppErrorType::DatabaseError).inner_error(&e.to_string())
                    })?;
                };

                let _ = &bot.connect().await?;
                Ok(HttpResponse::Ok().finish())
            }
        }
    }
}
