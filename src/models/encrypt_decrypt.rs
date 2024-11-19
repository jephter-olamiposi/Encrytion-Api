use crate::errors::ApiError;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::engine::Engine;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request for encryption
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct EncryptRequest {
    pub message: String,
}

impl EncryptRequest {
    /// Validates the `EncryptRequest`
    /// Ensures the message is non-empty and does not exceed 1024 characters.
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.message.trim().is_empty() {
            return Err(ApiError::ValidationError(
                "Message cannot be empty.".to_string(),
            ));
        }
        if self.message.len() > 1024 {
            return Err(ApiError::ValidationError(
                "Message exceeds 1024 characters.".to_string(),
            ));
        }
        Ok(())
    }
}

/// Response for encryption
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct EncryptResponse {
    pub encrypted_message: String,
}

/// Request for decryption
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct DecryptRequest {
    pub encrypted_message: String,
}

impl DecryptRequest {
    /// Validates the `DecryptRequest`
    /// Ensures the encrypted message is non-empty and is valid Base64.
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.encrypted_message.trim().is_empty() {
            return Err(ApiError::ValidationError(
                "Encrypted message cannot be empty.".to_string(),
            ));
        }
        if BASE64_STANDARD.decode(&self.encrypted_message).is_err() {
            return Err(ApiError::ValidationError(
                "Encrypted message is not valid Base64.".to_string(),
            ));
        }
        Ok(())
    }
}

/// Response for decryption
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct DecryptResponse {
    pub original_message: String,
}
