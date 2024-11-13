use crate::config::Config;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64;
use std::sync::Arc;

pub fn decrypt_message(
    encrypted_message: &str,
    config: Arc<Config>,
) -> Result<String, Box<dyn std::error::Error>> {
    let key = Key::<Aes256Gcm>::from_slice(config.aes_key.as_bytes());
    let cipher = Aes256Gcm::new(key);

    let parts: Vec<&str> = encrypted_message.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid encrypted message format".into());
    }

    let nonce_bytes = base64::decode(parts[0])?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = base64::decode(parts[1])?;
    let decrypted_bytes = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| e.to_string())?;
    Ok(String::from_utf8(decrypted_bytes)?)
}
