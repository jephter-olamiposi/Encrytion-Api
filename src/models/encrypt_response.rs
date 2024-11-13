use serde::Serialize;

#[derive(Serialize)]
pub struct EncryptResponse {
    pub encrypted_message: String,
}
