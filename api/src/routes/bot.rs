use std::sync::RwLock;

use actix_web::{get, web, HttpResponse};
use sqlx::{Pool, Postgres};

use crate::bot::manager;
use crate::bot::types::BotExternalAction;
use crate::errors::*;
use crate::extractors::user_from_cookie::UserFromCookie;
use crate::models;
use crate::states::app_config::AppConfig;
use crate::twitch::id_api::validate_user_token;

#[get("/info")]
pub async fn get_bot_info(
    user: UserFromCookie,
    pool: web::Data<Pool<Postgres>>,
    app_config: web::Data<AppConfig>,
    bot: web::Data<RwLock<manager::BotManager>>,
) -> Result<HttpResponse, AppError> {
    let bot = bot.read()?;

    let status = bot.status()?;
    let channel_registry = bot.channel_registry.clone();

    let name = &app_config.chat_bot_username;
    let connected = matches!(status, manager::BotStatus::Connected(_));

    let mb_credentials = models::BotCredentials::get_by_username(&pool, name).await?;

    let credentials_state = match mb_credentials {
        Some(credentials) => {
            let token_is_valid = validate_user_token(&credentials.access_token).await?;

            if token_is_valid {
                models::CredentialsState::Valid
            } else {
                models::CredentialsState::Invalid
            }
        }
        None => models::CredentialsState::NotFound,
    };

    let connected_to_chat = channel_registry.get(&user.logged.username).is_some();

    Ok(HttpResponse::Ok().json(models::BotInfo {
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
    pool: web::Data<Pool<Postgres>>,
    bot_manager: web::Data<RwLock<manager::BotManager>>,
) -> Result<HttpResponse, AppError> {

    if user.logged.username != app_config.chat_bot_username {
        Err(AppError::from(AppErrorType::Unauthorized))
    } else {
        let mut manager = bot_manager.write()?;
        let status = manager.status()?;

        match status {
            manager::BotStatus::Connected(_) => Ok(HttpResponse::Ok().finish()),
            manager::BotStatus::Disconnected => {
                models::BotCredentials::get_or_create(&pool, &models::CreateBotCredentials {
                    access: &user.session.access_token,
                    refresh: &user.session.refresh_token,
                    user_id: &user.logged.id,
                }).await?;

                manager.connect().await?;
                Ok(HttpResponse::Ok().finish())
            }
        }
    }
}
