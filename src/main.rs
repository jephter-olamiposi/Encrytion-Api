use actix_web::{middleware, web, App, HttpServer}; // Added HttpResponse

use dotenv::dotenv;
use env_logger;
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
    // Load environment variables and initialize logger
    dotenv().ok();
    env_logger::init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(middleware::Logger::default())
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()))
            .route("/encrypt", web::post().to(encrypt_handler::encrypt))
            .route("/decrypt", web::post().to(decrypt_handler::decrypt))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
