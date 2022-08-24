use actix_web::{get, web, HttpResponse};

use crate::bot::{BotMessage, Bot, BotStatus};
use crate::errors::*;
use crate::extractors::user_from_cookie::UserFromCookie;

struct BotInfo {
    authenticated: bool,
}

#[get("/info")]
pub async fn get_bot_status(_: UserFromCookie) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().finish())
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
