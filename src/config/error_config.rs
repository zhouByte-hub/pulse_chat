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
