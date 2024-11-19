use crate::{config::Config, errors::ApiError};
use aes_gcm::aead::{Aead, Nonce};
use aes_gcm::Aes256Gcm;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::engine::Engine;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

pub fn encrypt_message(message: &str, config: &Config) -> Result<String, ApiError> {
    let nonce_bytes = rand::random::<[u8; 12]>();
    let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce_bytes);

    let compressed = compress_message(message)?;

    let result = config
        .cipher
        .encrypt(nonce, compressed.as_ref())
        .map(|ciphertext| BASE64_STANDARD.encode([nonce.as_slice(), &ciphertext].concat()))
        .map_err(|_| ApiError::EncryptionError);

    result
}

fn compress_message(message: &str) -> Result<Vec<u8>, ApiError> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(message.as_bytes())
        .map_err(|_| ApiError::InternalError)?;
    encoder.finish().map_err(|_| ApiError::InternalError)
}
