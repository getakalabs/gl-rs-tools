pub mod cipher;
pub mod manager;
pub mod payload;

pub use cipher::Cipher;

use rand::Rng;
use std::default::Default;
use xsalsa20poly1305::aead::{Aead, KeyInit};
use xsalsa20poly1305::aead::generic_array::{GenericArray, typenum};
use xsalsa20poly1305::XSalsa20Poly1305;

use crate::Errors;

pub fn generate() -> String {
    base64_url::encode(&rand::thread_rng().gen::<[u8; 32]>())
}

/// Decrypt string content
pub fn decrypt<C, K>(content: C, key: K) -> Result<String, Errors>
    where C: ToString,
          K: ToString
{
    // Set key and content bindings
    let content = content.to_string();
    let key = key.to_string();

    // Retrieve key
    let result = base64_url::decode(&std::env::var(key).unwrap_or(String::default()));
    if result.is_err() {
        return Err(Errors::new("Invalid key"));
    }

    // Bind result
    let binding = result.unwrap();
    if binding.is_empty() {
        return Err(Errors::new("Invalid key"));
    }

    // Set key and cipher
    let key = GenericArray::from_slice(&binding);
    let cipher = XSalsa20Poly1305::new(key);

    // decode hash
    let result = base64_url::decode(&content);
    if result.is_err() {
        return Err(Errors::new("Unable to decode base64 url string"));
    }

    // Set encrypted content
    let encrypted_content = result.unwrap();
    if encrypted_content.len() <= 24 {
        return Err(Errors::new("Invalid length"));
    }

    // Set chunks and actual content
    let nonce = &encrypted_content[0..24];
    let content = &encrypted_content[24..];

    // Set nonce
    let nonce:&GenericArray<u8, typenum::U24> = GenericArray::from_slice(nonce);

    // Unseal hash
    let result = cipher.decrypt(nonce, content);
    if result.is_err() {
        return Err(Errors::new("Unable to decrypt string"));
    }

    // Set content
    let content = String::from_utf8_lossy(&result.unwrap()).to_string();

    // Return content
    Ok(content)
}


/// Encrypts string content
pub fn encrypt<C, K>(content: C, key: K) -> Result<String, Errors>
    where C: ToString,
          K: ToString
{
    // Set key and content bindings
    let content = content.to_string();
    let key = key.to_string();

    // Retrieve key
    let result = base64_url::decode(&std::env::var(key).unwrap_or(String::default()));
    if result.is_err() {
        return Err(Errors::new("Invalid key"));
    }

    // Bind result
    let binding = result.unwrap();
    if binding.is_empty() {
        return Err(Errors::new("Invalid key"));
    }

    // Set key and cipher
    let key = GenericArray::from_slice(&binding);
    let cipher = XSalsa20Poly1305::new(key);

    // Set nonce
    let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);
    let result = cipher.encrypt(&nonce,  content.as_bytes());
    if result.is_err() {
        return Err(Errors::new("Unable to encrypt content"));
    }

    // Set encrypted content
    let mut encrypted_content = nonce.clone().to_vec();
    encrypted_content.append(&mut result.unwrap().to_vec());

    // Set content
    let content = base64_url::encode(&encrypted_content);

    // Return content
    Ok(content)
}
