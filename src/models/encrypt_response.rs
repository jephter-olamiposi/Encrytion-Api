use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct EncryptResponse {
    pub encrypted_message: String,
}
