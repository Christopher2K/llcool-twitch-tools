use super::errors::*;
use crate::bot::types::BotExternalAction;

impl From<AppErrorType> for AppError {
    fn from(error_type: AppErrorType) -> Self {
        Self {
            error_type,
            extra_context: None,
            inner_error: None,
        }
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for AppError {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        AppError::from(AppErrorType::WebSocketError).inner_error(&error.to_string())
    }
}
impl From<url::ParseError> for AppError {
    fn from(error: url::ParseError) -> Self {
        AppError::from(AppErrorType::InternalError).inner_error(&error.to_string())
    }
}

impl From<twitch_irc::message::IRCParseError> for AppError {
    fn from(error: twitch_irc::message::IRCParseError) -> Self {
        AppError::from(AppErrorType::InternalError).inner_error(&error.to_string())
    }
}

impl From<twitch_irc::message::ServerMessageParseError> for AppError {
    fn from(error: twitch_irc::message::ServerMessageParseError) -> Self {
        AppError::from(AppErrorType::InternalError).inner_error(&error.to_string())
    }
}

impl<T> From<std::sync::PoisonError<std::sync::RwLockReadGuard<'_, T>>> for AppError {
    fn from(error: std::sync::PoisonError<std::sync::RwLockReadGuard<T>>) -> Self {
        AppError::from(AppErrorType::InternalError).inner_error(&error.to_string())
    }
}
impl<T> From<std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>> for AppError {
    fn from(error: std::sync::PoisonError<std::sync::RwLockWriteGuard<T>>) -> Self {
        AppError::from(AppErrorType::InternalError).inner_error(&error.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::from(AppErrorType::InternalError)
            .inner_error(&error.to_string())
            .extra_context(&format!("Status: {:?}", &error.status()))
    }
}

impl From<tokio::sync::mpsc::error::SendError<BotExternalAction>> for AppError {
    fn from(error: tokio::sync::mpsc::error::SendError<BotExternalAction>) -> Self {
        AppError::from(AppErrorType::BotCommunicationError).inner_error(&error.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::from(AppErrorType::DatabaseError).inner_error(&error.to_string())
    }
}
