use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct DecryptRequest {
    pub encrypted_message: String,
}
