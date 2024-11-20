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
    // Initialize dotenv and logger
    dotenv().ok();
    env_logger::init(); // Enable logger
                        // Load configuration
    info!("Loading configuration...");
    let config = Config::from_env().expect("Failed to load configuration");
    info!(
        "Configuration loaded successfully with port: {}",
        config.port
    );
    // Store the port separately to avoid borrowing issues
    let port = config.port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone())) // Clone config to avoid move
            .wrap(middleware::Logger::default()) // Enable logging
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()))
            .route("/encrypt", web::post().to(encrypt_handler::encrypt))
            .route("/decrypt", web::post().to(decrypt_handler::decrypt))
    })
    .bind(("0.0.0.0", port))? // Use port from config
    .run()
    .await
}
