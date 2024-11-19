use aes_gcm::{Aes256Gcm, Key, KeyInit};

use std::sync::Arc;

#[derive(Clone)]
pub struct Config {
    pub cipher: Arc<Aes256Gcm>, // Shared cipher instance
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        let aes_key = std::env::var("AES_KEY").map_err(|_| ConfigError::MissingKey)?;

        if aes_key.len() != 32 {
            return Err(ConfigError::InvalidKeyLength);
        }

        let cipher_key = Key::<Aes256Gcm>::from_slice(aes_key.as_bytes());
        let cipher = Aes256Gcm::new(cipher_key);

        Ok(Self {
            cipher: Arc::new(cipher),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("AES_KEY environment variable is not set.")]
    MissingKey,
    #[error("AES_KEY must be exactly 32 bytes.")]
    InvalidKeyLength,
}
