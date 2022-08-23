use actix_web::{get, web, HttpResponse};

use crate::bot::{Bot, BotMessage, BotStatus};
use crate::errors::*;
use crate::extractors::user_from_cookie::UserFromCookie;

#[get("/join")]
pub async fn join_chat(
    user: UserFromCookie,
    bot: web::Data<Bot>,
) -> Result<HttpResponse, AppError> {
    if let BotStatus::Connected(sender) = &bot.status {
        let sender = sender.clone();
        sender.send(BotMessage::HelloWorld).await;
    }

    Ok(HttpResponse::Ok().finish())
}

#[get("/leave")]
pub async fn leave_chat() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().finish())
}
