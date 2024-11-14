use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod handlers;
mod models;
mod services;

// Import your request and response models
use crate::models::decrypt_request::DecryptRequest;
use crate::models::decrypt_response::DecryptResponse;
use crate::models::encrypt_request::EncryptRequest;
use crate::models::encrypt_response::EncryptResponse;
use config::Config;
use handlers::{decrypt_handler, encrypt_handler};

// Define OpenAPI Documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::encrypt_handler::encrypt,
        handlers::decrypt_handler::decrypt
    ),
    components(schemas(EncryptRequest, EncryptResponse, DecryptRequest, DecryptResponse)),
    tags(
        (name = "Encryption API", description = "API for encrypting and decrypting messages")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().expect("Failed to load configuration");
    let config_data = Arc::new(config);

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config_data.clone()))
            // Add the Swagger UI service
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", openapi.clone()))
            // Define the API routes
            .route("/encrypt", web::post().to(encrypt_handler::encrypt))
            .route("/decrypt", web::post().to(decrypt_handler::decrypt))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
