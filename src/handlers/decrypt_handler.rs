use crate::config::Config;
use crate::models::{decrypt_request::DecryptRequest, decrypt_response::DecryptResponse};
use crate::services::decryption;
use actix_web::{web, HttpResponse};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/decrypt",
    request_body = DecryptRequest,
    responses(
        (status = 200, description = "Message decrypted successfully", body = DecryptResponse),
        (status = 500, description = "Decryption failed")
    )
)]
pub async fn decrypt(
    req: web::Json<DecryptRequest>,
    config: web::Data<Arc<Config>>,
) -> HttpResponse {
    match decryption::decrypt_message(&req.encrypted_message, config.get_ref().clone()) {
        Ok(original_message) => HttpResponse::Ok().json(DecryptResponse { original_message }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
