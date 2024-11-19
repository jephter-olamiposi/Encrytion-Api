use crate::{
    config::Config,
    errors::ApiError,
    models::encrypt_decrypt::{DecryptRequest, DecryptResponse},
    services::decryption,
};
use actix_web::{web, HttpResponse};

#[utoipa::path(
    post,
    path = "/decrypt",
    request_body = DecryptRequest,
    responses(
        (status = 200, description = "Message decrypted successfully", body = DecryptResponse),
        (status = 400, description = "Validation error"),
        (status = 500, description = "Decryption failed")
    )
)]
pub async fn decrypt(
    req: web::Json<DecryptRequest>,
    config: web::Data<Config>,
) -> Result<HttpResponse, ApiError> {
    // Validate request
    req.validate()?;

    // Decrypt the message
    let original_message = decryption::decrypt_message(&req.encrypted_message, &config)?;

    // Return the response
    Ok(HttpResponse::Ok().json(DecryptResponse { original_message }))
}
