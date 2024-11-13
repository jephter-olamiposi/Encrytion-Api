// src/config/mod.rs
use dotenv::dotenv;
use std::env;

pub struct Config {
    pub aes_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenv().ok();
        let aes_key = env::var("AES_KEY").map_err(|_| "AES_KEY must be set in .env file")?;
        if aes_key.len() != 32 {
            return Err("AES_KEY must be exactly 32 bytes for AES-256 encryption.".to_string());
        }
        Ok(Config { aes_key })
    }
}
