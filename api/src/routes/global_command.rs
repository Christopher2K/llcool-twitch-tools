use actix_web::{delete, get, patch, post, web, HttpResponse};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::errors::{AppError, AppErrorType};
use crate::extractors::UserFromCookie;
use crate::models::{self, CommandDefinitionPayload};
use crate::states::app_config::AppConfig;

#[get("")]
pub async fn get_all_global_commands(
    pool: web::Data<Pool<Postgres>>,
    config: web::Data<AppConfig>,
    user: UserFromCookie,
) -> Result<HttpResponse, AppError> {
    if config.chat_bot_username != user.logged.username {
        return Err(AppError::from(AppErrorType::Forbidden));
    }

    let global_commands = models::GlobalCommand::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(global_commands))
}

#[post("")]
pub async fn create_global_command(
    pool: web::Data<Pool<Postgres>>,
    user: UserFromCookie,
    config: web::Data<AppConfig>,
    payload: web::Json<CommandDefinitionPayload>,
) -> Result<HttpResponse, AppError> {
    if config.chat_bot_username != user.logged.username {
        return Err(AppError::from(AppErrorType::Forbidden));
    }

    let new_command = models::GlobalCommand::create(&pool, payload.clone()).await?;

    Ok(HttpResponse::Ok().json(new_command))
}

#[patch("/{command_id}")]
pub async fn edit_global_command(
    pool: web::Data<Pool<Postgres>>,
    user: UserFromCookie,
    config: web::Data<AppConfig>,
    payload: web::Json<CommandDefinitionPayload>,
    global_command_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    if config.chat_bot_username != user.logged.username {
        return Err(AppError::from(AppErrorType::Forbidden));
    }

    let edited_command =
        models::GlobalCommand::update(&pool, payload.clone(), &global_command_id).await?;

    Ok(HttpResponse::Ok().json(edited_command))
}

#[delete("/{command_id}")]
pub async fn delete_global_command(
    pool: web::Data<Pool<Postgres>>,
    user: UserFromCookie,
    config: web::Data<AppConfig>,
    command_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    if config.chat_bot_username != user.logged.username {
        return Err(AppError::from(AppErrorType::Forbidden));
    }

    let rows_affected = models::GlobalCommand::delete(&pool, &command_id)
        .await?
        .rows_affected();

    { rows_affected > 1 }
        .then(|| HttpResponse::Accepted().finish())
        .ok_or(AppError::from(AppErrorType::EntityNotFoundError))
}
