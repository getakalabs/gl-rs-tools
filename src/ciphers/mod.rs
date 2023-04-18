pub mod action;
pub mod cipher;
pub mod manager;
pub mod payload;

pub use crate::ciphers::cipher::Cipher;
pub use crate::ciphers::action::CipherAction;

use anyhow::Result;
use rand::Rng;
use xsalsa20poly1305::aead::{Aead, KeyInit};
use xsalsa20poly1305::aead::generic_array::{GenericArray, typenum};
use xsalsa20poly1305::XSalsa20Poly1305;

pub fn generate() -> String {
    base64_url::encode(&rand::thread_rng().gen::<[u8; 32]>())
}

pub fn decrypt<C, K>(content: C, key: K) -> Result<String>
    where C: ToString,
          K: ToString
{
    // Create bindings and then generate cipher instance
    let bindings = base64_url::decode(&std::env::var(key.to_string())?)?;
    let key = GenericArray::from_slice(&bindings);
    let cipher = XSalsa20Poly1305::new(key);

    // Set content
    let content = base64_url::decode(&content.to_string())?;
    if content.len() <= 24 {
        return Err(anyhow::anyhow!("Invalid content length"));
    }

    // Split content
    let (nonce, content) = content.split_at(24);

    // Set nonce & content
    let nonce:&GenericArray<u8, typenum::U24> = GenericArray::from_slice(nonce);
    let content = match cipher.decrypt(nonce, content) {
        Ok(content) => content,
        Err(_) => return Err(anyhow::anyhow!("Unable to decrypt content"))
    };

    // Return decrypted content
    Ok(String::from_utf8_lossy(&content).to_string())
}

pub fn encrypt<C, K>(content: C, key: K) -> Result<String>
    where C: ToString,
          K: ToString
{
    // Create bindings and then generate cipher instance
    let bindings = base64_url::decode(&std::env::var(key.to_string())?)?;
    let key = GenericArray::from_slice(&bindings);
    let cipher = XSalsa20Poly1305::new(key);

    // Set nonce
    let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);
    let content = match cipher.encrypt(&nonce, content.to_string().as_bytes()) {
        Ok(content) => content,
        Err(_) => return Err(anyhow::anyhow!("Unable to encrypt content"))
    };

    // Return encrypted content
    Ok(base64_url::encode(&[&nonce[..], &content[..]].concat()))
}