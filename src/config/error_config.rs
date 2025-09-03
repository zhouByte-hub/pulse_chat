use ::config::ConfigError;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PulseError {
    #[error("StdError: {0}")]
    StdError(#[from] std::io::Error),

    #[error("DbError: {0}")]
    DbError(#[from] sea_orm::DbErr),

    #[error("Database connection error: {0}")]
    ConnectionError(String),

    #[error("Database not initialized")]
    DatabaseNotInitialized,

    #[error("Config error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("Token error: {0}")]
    TokenError(#[from] jsonwebtoken::errors::Error),

    #[error("PulseStdError error: {0}")]
    PulseStdError(String),

    #[error("ActixWebError error: {0}")]
    ActixWebError(#[from] actix_web::error::Error),
}

impl ResponseError for PulseError {
    fn status_code(&self) -> StatusCode {
        match self {
            PulseError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PulseError::ConnectionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(("Content-Type", "text/plain; charset=utf-8"))
            .body(self.to_string())
    }
}
