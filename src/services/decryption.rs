use crate::{config::Config, errors::ApiError};
use aes_gcm::aead::{Aead, Nonce};
use aes_gcm::Aes256Gcm;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::engine::Engine;
use flate2::read::GzDecoder;
use std::io::Read;

pub fn decrypt_message(encrypted_message: &str, config: &Config) -> Result<String, ApiError> {
    let data = BASE64_STANDARD
        .decode(encrypted_message)
        .map_err(|_| ApiError::DecryptionError)?;

    if data.len() < 12 {
        return Err(ApiError::DecryptionError);
    }

    let (nonce, ciphertext) = data.split_at(12);
    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce); // No need for map_err

    let decompressed = config
        .cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| ApiError::DecryptionError)?;

    decompress_message(&decompressed)
}

fn decompress_message(data: &[u8]) -> Result<String, ApiError> {
    let mut decoder = GzDecoder::new(data);
    let mut result = String::new();
    decoder
        .read_to_string(&mut result)
        .map_err(|_| ApiError::InternalError)?;
    Ok(result)
}
