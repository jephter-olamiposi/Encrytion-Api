use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::sync::Arc;

mod config;
mod handlers;
mod models;
mod services;

use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file

    let config = Config::from_env().expect("Failed to load configuration");
    let config_data = Arc::new(config);

    HttpServer::new(move || App::new().app_data(web::Data::new(config_data.clone())))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
