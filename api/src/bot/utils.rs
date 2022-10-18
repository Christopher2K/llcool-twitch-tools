use sqlx::{Pool, Postgres};

use crate::errors::{AppError, AppErrorType};
use crate::models::v2;
use crate::models::user::get_user_by_username;
use crate::states::app_config::AppConfig;
use crate::twitch::id_api::renew_token;
use crate::types::DbPool;

pub const LOG_TARGET: &'static str = "twitch_bot::manager";
pub const WEBSOCKET_CLIENT_URL: &'static str = "wss://irc-ws.chat.twitch.tv:443";

pub async fn get_bot_access_token(
    config: &AppConfig,
    pool: &DbPool,
    _pool: &Pool<Postgres>,
    log_target: &str,
) -> Result<String, AppError> {
    log::info!(
        target: log_target,
        "Getting and refreshing bot credentials..."
    );
    let mut db = pool.get()?;

    /* Automatically renewing bot credentials each and everytime we are connecting
     * the bot to Twitch WS
     */
    let user = get_user_by_username(&mut db, &config.chat_bot_username)?;
    let credentials = v2::BotCredentials::get_by_user_id(_pool, &user.id)
        .await?
        .ok_or(AppError::from(AppErrorType::EntityNotFoundError))?;

    let tokens = renew_token(config, &credentials.refresh_token).await?;

    v2::BotCredentials::update_by_user_id(_pool, &v2::UpdateBotCredentials {
        user_id: &user.id,
        refresh: &tokens.refresh_token,
        access: &tokens.access_token
    }).await?;

    Ok(tokens.access_token)
}
