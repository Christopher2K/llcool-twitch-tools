use actix_web::{delete, get, patch, post, web, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::extractors::UserFromCookie;
use crate::models;
use crate::types::DbPool;

// TODO: CHECK IF THE COMMAND DOES BELONG TO THE USER
// YOU DON'T WANT TO LET PEOPLE EDIT COMMANDS OF OTHER PEOPLE

#[get("")]
pub async fn get_user_commands(
    user: UserFromCookie,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let mut db = db.get()?;
    let commands = models::user_command::get_all_users_commands(&mut db, &user.logged.id)?;

    Ok(HttpResponse::Ok().json(commands))
}

#[post("")]
pub async fn create_user_command(
    user: UserFromCookie,
    mut data: web::Json<models::user_command::NewUserCommand>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let mut db = db.get()?;
    data.user_id = Some(user.logged.id.clone());

    let command = models::user_command::create_user_command(&mut db, &data)?;
    Ok(HttpResponse::Created().json(command))
}

#[patch("/{command_id}")]
pub async fn update_user_command(
    command_id: web::Path<Uuid>,
    data: web::Json<models::user_command::UpdateUserCommand>,
    user: UserFromCookie,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let mut db = db.get()?;
    let command = models::user_command::update_user_command(&mut db, &command_id, &user.logged.id, &data)?;

    Ok(HttpResponse::Ok().json(command))
}

#[delete("/{command_id}")]
pub async fn delete_user_command(
    command_id: web::Path<Uuid>,
    _user: UserFromCookie,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    let mut db = db.get()?;
    models::user_command::delete_user_command(&mut db, &command_id)?;

    Ok(HttpResponse::NoContent().finish())
}
