use actix_web::{delete, get, post, web, HttpResponse};
use sqlx::{Pool, Postgres};

use crate::errors::AppError;
use crate::extractors::UserFromCookie;

#[get("")]
pub async fn get_user_global_commands(
    pool: web::Data<Pool<Postgres>>,
    user: UserFromCookie,
) -> Result<HttpResponse, AppError> {
    // TODO: if user is bot, return all commands, if user is regular user return user commands
    todo!();
}

#[post("")]
pub async fn enable_user_global_command(
    pool: web::Data<Pool<Postgres>>,
    user: UserFromCookie,
    global_command_id: web::Json<()>,
) -> Result<HttpResponse, AppError> {
    todo!();
}

#[delete("")]
pub async fn disable_user_global_command(
    pool: web::Data<Pool<Postgres>>,
    user: UserFromCookie,
    global_command_id: web::Json<()>,
) -> Result<HttpResponse, AppError> {
    todo!();
}
