use crate::{errors::AppError, types::DbPool, models::{user::get_user_by_username, bot_credentials::{get_bot_credentials_by_user_id, update_bot_credentials, UpdateBotCredentials}}, twitch::id_api::renew_token, states::app_config::AppConfig};

pub const LOG_TARGET: &'static str = "twitch_bot::manager";
pub const WEBSOCKET_CLIENT_URL: &'static str = "wss://irc-ws.chat.twitch.tv:443";

pub async fn get_bot_access_token(
    config: &AppConfig,
    pool: &DbPool,
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
    let credentials = get_user_by_username(&mut db, &config.chat_bot_username)
        .and_then(|user| get_bot_credentials_by_user_id(&mut db, &user.id))?;

    let tokens = renew_token(config, &credentials.refresh_token).await?;

    update_bot_credentials(
        &mut db,
        &credentials.id,
        UpdateBotCredentials {
            access_token: &tokens.access_token.clone(),
            refresh_token: &tokens.refresh_token.clone(),
        },
    )?;

    Ok(tokens.access_token)
}
