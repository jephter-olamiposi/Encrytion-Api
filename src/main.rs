use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod handlers;
mod models;
mod services;

use crate::models::decrypt_request::DecryptRequest;
use crate::models::decrypt_response::DecryptResponse;
use crate::models::encrypt_request::EncryptRequest;
use crate::models::encrypt_response::EncryptResponse;
use config::Config;
use handlers::{decrypt_handler, encrypt_handler};

// OpenAPI documentation setup
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
    dotenv().ok(); // Load environment variables from .env if present

    // Retrieve the port from the environment, defaulting to 8080
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let config = Config::from_env().expect("Failed to load configuration");
    let config_data = Arc::new(config);

    // OpenAPI documentation
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config_data.clone()))
            // Swagger UI route
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", openapi.clone()))
            // API routes
            .route("/encrypt", web::post().to(encrypt_handler::encrypt))
            .route("/decrypt", web::post().to(decrypt_handler::decrypt))
    })
    .bind(("0.0.0.0", port.parse::<u16>().expect("Invalid port")))? // Bind to 0.0.0.0 and the port
    .run()
    .await
}
