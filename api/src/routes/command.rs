use actix_web::{delete, get, patch, post, web, HttpResponse};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::errors::{AppError, AppErrorType};
use crate::extractors::UserFromCookie;
use crate::models::v2;

#[get("")]
pub async fn get_user_commands(
    user: UserFromCookie,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, AppError> {
    let commands = v2::UserCommand::get_all_by_user_id(&pool, &user.logged.id).await?;

    Ok(HttpResponse::Ok().json(commands))
}

#[post("")]
pub async fn create_user_command(
    user: UserFromCookie,
    data: web::Json<v2::UserCommandPayload>,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, AppError> {
    let command = v2::UserCommand::create(
        &pool,
        &v2::CreateUserCommand {
            user_id: &user.logged.id,
            name: &data.name,
            message: &data.message,
        },
    )
    .await?;

    Ok(HttpResponse::Created().json(command))
}

#[patch("/{command_id}")]
pub async fn update_user_command(
    command_id: web::Path<Uuid>,
    data: web::Json<v2::UserCommandPayload>,
    user: UserFromCookie,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, AppError> {
    let command = v2::UserCommand::update(
        &pool,
        &v2::UpdateUserCommand {
            id: &command_id,
            name: &data.name,
            message: &data.message,
            user_id: &user.logged.id,
        },
    )
    .await?;

    Ok(HttpResponse::Ok().json(command))
}

#[delete("/{command_id}")]
pub async fn delete_user_command(
    command_id: web::Path<Uuid>,
    user: UserFromCookie,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, AppError> {
    let query_result = v2::UserCommand::delete(&pool, &user.logged.id, &command_id).await?;

    if query_result.rows_affected() < 1 {
        Err(AppError::from(AppErrorType::EntityNotFoundError))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}
