use serde::Serialize;

#[derive(Serialize)]
pub struct DecryptResponse {
    pub original_message: String,
}
