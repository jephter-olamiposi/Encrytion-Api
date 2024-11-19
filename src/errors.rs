use actix_web::{HttpResponse, ResponseError};
use log::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Encryption error.")]
    EncryptionError,
    #[error("Decryption error.")]
    DecryptionError,
    #[error("Internal server error.")]
    InternalError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::ValidationError(msg) => HttpResponse::BadRequest().body(msg.to_string()),
            ApiError::EncryptionError => {
                error!("Encryption error occurred.");
                HttpResponse::InternalServerError().body("Encryption failed.")
            }
            ApiError::DecryptionError => {
                error!("Decryption error occurred.");
                HttpResponse::InternalServerError().body("Decryption failed.")
            }
            ApiError::InternalError => {
                error!("An unexpected internal server error occurred.");
                HttpResponse::InternalServerError().body("Internal server error.")
            }
        }
    }
}
