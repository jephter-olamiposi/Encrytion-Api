use crate::config::Config;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64;
use rand::Rng;
use std::sync::Arc;

pub fn encrypt_message(
    message: &str,
    config: Arc<Config>,
) -> Result<String, Box<dyn std::error::Error>> {
    let key = Key::<Aes256Gcm>::from_slice(config.aes_key.as_bytes());
    let cipher = Aes256Gcm::new(key);

    let nonce_bytes = rand::thread_rng().gen::<[u8; 12]>();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, message.as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(format!(
        "{}:{}",
        base64::encode(nonce),
        base64::encode(ciphertext)
    ))
}
