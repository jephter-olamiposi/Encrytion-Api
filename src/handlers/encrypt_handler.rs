use crate::config::Config;
use crate::models::{encrypt_request::EncryptRequest, encrypt_response::EncryptResponse};
use crate::services::encryption;
use actix_web::{web, HttpResponse};
use std::sync::Arc;

pub async fn encrypt(
    req: web::Json<EncryptRequest>,
    config: web::Data<Arc<Config>>,
) -> HttpResponse {
    match encryption::encrypt_message(&req.message, config.get_ref().clone()) {
        Ok(encrypted_message) => HttpResponse::Ok().json(EncryptResponse { encrypted_message }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
