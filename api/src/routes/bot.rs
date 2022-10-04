use actix_web::{get, web, HttpResponse};
use diesel::OptionalExtension;
use std::sync::RwLock;

use crate::bot::manager;
use crate::bot::types::BotExternalAction;
use crate::errors::*;
use crate::extractors::user_from_cookie::UserFromCookie;
use crate::models::bot_credentials::{
    create_bot_credentials, get_bot_credentials_by_user_id, CreateBotCredentials,
};
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
    bot: web::Data<RwLock<manager::BotManager>>,
) -> Result<HttpResponse, AppError> {
    let bot = bot.read()?;
    let mut db = db.get()?;

    let status = bot.status()?;
    let channel_registry = bot.channel_registry.clone();

    let name = &app_config.chat_bot_username;
    let connected = matches!(status, manager::BotStatus::Connected(_));

    let mb_credentials =
        get_user_by_username(&mut db, name)
            .optional()
            .and_then(|mb_bot_user| {
                if let Some(bot_user) = mb_bot_user {
                    get_bot_credentials_by_user_id(&mut db, &bot_user.id.clone()).optional()
                } else {
                    Ok(None)
                }
            })?;

    let credentials_state = match mb_credentials {
        Some(credentials) => {
            let token_is_valid = validate_user_token(&credentials.access_token).await?;

            if token_is_valid {
                CredentialsState::Valid
            } else {
                CredentialsState::Invalid
            }
        }
        None => CredentialsState::NotFound,
    };

    let connected_to_chat = channel_registry.get(&user.logged.username).is_some();

    Ok(HttpResponse::Ok().json(BotInfo {
        name: name.to_string(),
        credentials_state,
        connected,
        connected_to_chat,
    }))
}

#[get("/join")]
pub async fn join_chat(
    user: UserFromCookie,
    bot: web::Data<RwLock<manager::BotManager>>,
) -> Result<HttpResponse, AppError> {
    let bot = bot.read()?;
    let status = bot.status()?.clone();

    match status {
        manager::BotStatus::Connected(sender) => {
            sender
                .send(BotExternalAction::Join {
                    channel_name: user.logged.username.clone(),
                    user_id: Some(user.logged.id.clone()),
                })
                .await?;

            // TODO: Remove this, set this for testing purposes
            sender
                .send(BotExternalAction::Join {
                    channel_name: String::from("namelessw05"),
                    user_id: None,
                })
                .await?;

            Ok(HttpResponse::Ok().finish())
        }
        _ => Err(AppError::from(AppErrorType::BotDisconnected)),
    }
}

#[get("/leave")]
pub async fn leave_chat(
    user: UserFromCookie,
    bot: web::Data<RwLock<manager::BotManager>>,
) -> Result<HttpResponse, AppError> {
    let bot = bot.read()?;
    let status = bot.status()?.clone();

    match status {
        manager::BotStatus::Connected(sender) => {
            sender
                .send(BotExternalAction::Leave(user.logged.username.clone()))
                .await?;

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
    bot_manager: web::Data<RwLock<manager::BotManager>>,
) -> Result<HttpResponse, AppError> {
    let mut db = db
        .get()
        .map_err(|e| AppError::from(AppErrorType::DatabaseError).inner_error(&e.to_string()))?;

    if user.logged.username != app_config.chat_bot_username {
        Err(AppError::from(AppErrorType::Unauthorized))
    } else {
        let mut manager = bot_manager.write()?;
        let status = manager.status()?;

        match status {
            manager::BotStatus::Connected(_) => Ok(HttpResponse::Ok().finish()),
            manager::BotStatus::Disconnected => {
                // Ensure that bot credentials does exist
                let mb_credentials = get_bot_credentials_by_user_id(&mut db, &user.logged.id);
                let credentials_missing =
                    matches!(mb_credentials, Err(diesel::result::Error::NotFound));

                if credentials_missing {
                    create_bot_credentials(
                        &mut db,
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

                manager.connect().await?;
                Ok(HttpResponse::Ok().finish())
            }
        }
    }
}
