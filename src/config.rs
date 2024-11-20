use aes_gcm::{Aes256Gcm, Key, KeyInit};
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct Config {
    pub cipher: Arc<Aes256Gcm>, // Shared cipher instance
    pub port: u16,              // Port configuration
}

impl Config {
    /// Loads configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        // Load and validate AES_KEY
        let aes_key = env::var("AES_KEY").map_err(|_| ConfigError::MissingKey)?;
        if aes_key.len() != 32 {
            return Err(ConfigError::InvalidKeyLength);
        }
        let cipher_key = Key::<Aes256Gcm>::from_slice(aes_key.as_bytes());
        let cipher = Aes256Gcm::new(cipher_key);

        // Load and validate PORT
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string()) // Default to 8080 if not set
            .parse::<u16>()
            .map_err(|_| ConfigError::InvalidPort)?;

        Ok(Self {
            cipher: Arc::new(cipher),
            port,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("AES_KEY environment variable is not set.")]
    MissingKey,
    #[error("AES_KEY must be exactly 32 bytes.")]
    InvalidKeyLength,
    #[error("PORT environment variable must be a valid u16.")]
    InvalidPort,
}
