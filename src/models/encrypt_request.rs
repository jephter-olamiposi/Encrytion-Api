use serde::Deserialize;

#[derive(Deserialize)]
pub struct EncryptRequest {
    pub message: String,
}
