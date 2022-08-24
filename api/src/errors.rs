use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use serde::Serialize;

#[derive(Debug, Display, Serialize, Clone)]
pub enum AppErrorType {
    InternalError,
    DatabaseError,
    AppStateError,
    WebSocketError,

    TwitchApiError,

    OAuthStateError,
    Unauthenticated,
}

#[derive(Debug, Display, Error, Clone)]
#[display(fmt = "AppError: {}", error_type)]
pub struct AppError {
    pub error_type: AppErrorType,
    pub extra_context: Option<String>,
    pub inner_error: Option<String>,
}

#[derive(Serialize)]
struct SerializableError {
    error_code: String,
    error_message: String,
}

impl AppError {
    pub fn new(error_type: Option<AppErrorType>) -> Self {
        Self {
            error_type: error_type.unwrap_or(AppErrorType::InternalError),
            extra_context: None,
            inner_error: None,
        }
    }

    pub fn from(error_type: AppErrorType) -> Self {
        Self {
            error_type,
            extra_context: None,
            inner_error: None,
        }
    }

    pub fn extra_context(self, extra_content: &str) -> Self {
        Self {
            extra_context: Some(extra_content.to_string()),
            ..self
        }
    }

    pub fn inner_error(self, inner_error: &str) -> Self {
        println!("{}", inner_error);
        Self {
            extra_context: Some(inner_error.to_string()),
            ..self
        }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(SerializableError {
                error_code: self.error_type.to_string(),
                error_message: self
                    .extra_context
                    .clone()
                    .unwrap_or(String::from("An error occured")),
            })
    }

    fn status_code(&self) -> reqwest::StatusCode {
        match self.error_type {
            AppErrorType::OAuthStateError => StatusCode::UNAUTHORIZED,
            AppErrorType::Unauthenticated => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
