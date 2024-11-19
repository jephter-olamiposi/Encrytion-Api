use crate::{
    config::Config,
    errors::ApiError,
    models::encrypt_decrypt::{EncryptRequest, EncryptResponse},
    services::encryption,
};
use actix_web::{web, HttpResponse};

#[utoipa::path(
    post,
    path = "/encrypt",
    request_body = EncryptRequest,
    responses(
        (status = 200, description = "Message encrypted successfully", body = EncryptResponse),
        (status = 400, description = "Validation error"),
        (status = 500, description = "Encryption failed")
    )
)]
pub async fn encrypt(
    req: web::Json<EncryptRequest>,
    config: web::Data<Config>,
) -> Result<HttpResponse, ApiError> {
    // Validate request
    req.validate()?;

    // Encrypt the message
    let encrypted_message = encryption::encrypt_message(&req.message, &config)?;

    // Return the response
    Ok(HttpResponse::Ok().json(EncryptResponse { encrypted_message }))
}
