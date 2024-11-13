use serde::Deserialize;

#[derive(Deserialize)]
pub struct DecryptRequest {
    pub encrypted_message: String,
}
