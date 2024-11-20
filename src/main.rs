use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod errors;
mod handlers;
mod models;
mod services;

use config::Config;
use handlers::{decrypt_handler, encrypt_handler};
use models::encrypt_decrypt::{DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        encrypt_handler::encrypt,
        decrypt_handler::decrypt
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
    env_logger::init(); // Initialize logger for middleware::Logger

    info!("Loading configuration...");
    let config = Config::from_env().expect("Failed to load configuration");
    info!("Configuration loaded successfully.");

    let port = config.port; // Extract port for server binding

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(middleware::Logger::default()) // Enable request/response logging
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()))
            .route("/encrypt", web::post().to(encrypt_handler::encrypt))
            .route("/decrypt", web::post().to(decrypt_handler::decrypt))
    })
    .bind(("0.0.0.0", port))? // Use port from Config
    .run()
    .await
}
