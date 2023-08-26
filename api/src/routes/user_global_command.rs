use actix_web::{delete, get, post, web, HttpResponse};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::errors::{AppError, AppErrorType};
use crate::extractors::UserFromCookie;
use crate::models;

#[get("")]
pub async fn get_user_global_commands(
    pool: web::Data<Pool<Postgres>>,
    user: UserFromCookie,
) -> Result<HttpResponse, AppError> {
    let user_global_commands =
        models::GlobalCommand::get_all_for_user(&pool, &user.logged.id).await?;

    Ok(HttpResponse::Ok().json(user_global_commands))
}

#[post("/{command_id}")]
pub async fn enable_user_global_command(
    pool: web::Data<Pool<Postgres>>,
    user: UserFromCookie,
    command_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let added_command =
        models::GlobalCommand::add_to_user(&pool, &user.logged.id, &command_id).await?;

    { added_command.rows_affected() > 1 }
        .then(|| {
            HttpResponse::Accepted().json(models::EntityIdResponse {
                id: command_id.clone(),
            })
        })
        .ok_or(AppError::from(AppErrorType::DatabaseError))
}

#[delete("/{command_id}")]
pub async fn disable_user_global_command(
    pool: web::Data<Pool<Postgres>>,
    user: UserFromCookie,
    command_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let removed_command =
        models::GlobalCommand::remove_from_user(&pool, &user.logged.id, &command_id).await?;

    { removed_command.rows_affected() > 1 }
        .then(|| {
            HttpResponse::Ok().json(models::EntityIdResponse {
                id: command_id.clone(),
            })
        })
        .ok_or(AppError::from(AppErrorType::DatabaseError))
}
