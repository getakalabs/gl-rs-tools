use bstr::ByteSlice;
use serde::{Serialize, Deserialize};
use std::default::Default;
use std::fmt::Debug;
use xsalsa20poly1305::aead::{Aead, KeyInit};
use xsalsa20poly1305::aead::generic_array::{GenericArray, typenum};
use xsalsa20poly1305::XSalsa20Poly1305;

use crate::traits::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manager {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_encrypted: Option<bool>,
}

impl IsEmpty for Manager {
    fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }
}

impl ToString for Manager {
    fn to_string(&self) -> String {
        match self.clone().content {
            Some(value) => value,
            None => String::default()
        }
    }
}

impl ToOptString for Manager {
    fn to_opt_string(&self) -> Option<String> {
        match self.clone().content {
            Some(value) => match value.is_empty() {
                true => None,
                false => Some(value)
            },
            None => None
        }
    }
}

impl Manager {
    pub(super) fn new<C>(content: C) -> Self
        where C: ToString
    {
        Self {
            content: Some(content.to_string()),
            hash: None,
            is_encrypted: Some(false),
        }
    }

    pub(super) fn is_encryption_ready(&self) -> bool {
        let is_encrypted = self.is_encrypted.unwrap_or(false);
        let is_empty_content = self.content.clone().unwrap_or(String::default()).is_empty();

        !is_encrypted && !is_empty_content
    }

    pub(super) fn is_decryption_ready(&self) -> bool {
        let is_encrypted = self.is_encrypted.unwrap_or(false);
        let is_empty_content = self.content.clone().unwrap_or(String::default()).is_empty();
        let is_empty_hash = self.content.clone().unwrap_or(String::default()).is_empty();

        is_encrypted && !is_empty_content && !is_empty_hash
    }

    pub(super) fn encrypt<K>(&self, key: K) -> Self
        where K: ToString
    {
        // Return  if self is not ready for encryption
        if !self.is_encryption_ready() {
            return self.clone();
        }

        // Clone manager
        let mut manager = self.clone();

        // Retrieve key
        let key = key.to_string();
        let result = base64_url::decode(&std::env::var(key).unwrap_or(String::default()));
        if result.is_err() {
            return self.clone();
        }

        // Bind result
        let binding = result.unwrap();
        if binding.is_empty() {
            return self.clone();
        }


        // Set key & cipher
        let key = GenericArray::from_slice(&binding);
        let cipher = XSalsa20Poly1305::new(key);

        // Set nonce
        let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);

        // Generate hash
        let result = base64_url::decode(&super::generate());
        if result.is_err() {
            return self.clone();
        }

        // Set hash
        let hash = result.unwrap();

        // Encrypt hash
        let result = cipher.encrypt(&nonce, hash.as_bytes());
        if result.is_err() {
            return self.clone();
        }

        // Set encrypted hash
        let mut encrypted_hash = nonce.clone().to_vec();
        encrypted_hash.append(&mut result.unwrap().to_vec());

        // Set hash
        manager.hash = Some(base64_url::encode(&encrypted_hash));

        // Set nonce
        let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);
        let cipher = XSalsa20Poly1305::new(GenericArray::from_slice(&hash));

        // Encrypt
        let content = manager.content.clone().unwrap_or(String::default());
        let result = cipher.encrypt(&nonce, content.as_bytes());
        if result.is_err() {
            return self.clone();
        }

        // Set encrypted content
        let mut encrypted_content = nonce.clone().to_vec();
        encrypted_content.append(&mut result.unwrap().to_vec());

        // Set manager
        manager.content = Some(base64_url::encode(&encrypted_content));
        manager.is_encrypted = Some(true);

        // Return manager
        manager
    }

    pub(super) fn decrypt<K>(&self, key: K) -> Self
        where K: ToString
    {
        // Return  if self is not ready for decryption
        if !self.is_decryption_ready() {
            return self.clone();
        }

        // Clone manager
        let mut manager = self.clone();

        // Retrieve key
        let key = key.to_string();
        let result = base64_url::decode(&std::env::var(key).unwrap_or(String::default()));
        if result.is_err() {
            return self.clone();
        }

        // Bind result
        let binding = result.unwrap();
        if binding.is_empty() {
            return self.clone();
        }

        // Set key & cipher
        let key = GenericArray::from_slice(&binding);
        let cipher = XSalsa20Poly1305::new(key);

        // decode hash
        let hash = manager.hash.clone().unwrap_or(String::default());
        let result = base64_url::decode(&hash);
        if result.is_err() {
            return self.clone();
        }

        // Set encrypted hash
        let encrypted_hash = result.unwrap();
        if encrypted_hash.len() <= 24 {
            return self.clone();
        }

        // Set chunks and actual content
        let nonce = &encrypted_hash[0..24];
        let content = &encrypted_hash[24..];

        // Set nonce
        let nonce:&GenericArray<u8, typenum::U24> = GenericArray::from_slice(nonce);

        // Unseal hash
        let result = cipher.decrypt(nonce, content);
        if result.is_err() {
            return self.clone();
        }

        // Bind result
        let binding = result.unwrap();
        if binding.is_empty() {
            return self.clone();
        }

        // Create cipher
        let key = GenericArray::from_slice(&binding);
        let cipher = XSalsa20Poly1305::new(key);

        // decode content
        let content = manager.content.clone().unwrap_or(String::default());
        let result = base64_url::decode(&content);
        if result.is_err() {
            return self.clone();
        }

        // Set encrypted content
        let encrypted_content = result.unwrap();
        if encrypted_content.len() <= 24 {
            return self.clone();
        }

        // Set chunks and actual content
        let nonce = &encrypted_content[0..24];
        let content = &encrypted_content[24..];

        // Set nonce
        let nonce:&GenericArray<u8, typenum::U24> = GenericArray::from_slice(nonce);

        // Unseal hash
        let result = cipher.decrypt(nonce, content);
        if result.is_err() {
            return self.clone();
        }

        // Set content
        let content = String::from_utf8_lossy(&result.unwrap()).to_string();

        // Set manager
        manager.content = Some(content);
        manager.is_encrypted = Some(false);

        // Return manager
        manager
    }

    pub(super) fn get_content(&self) -> Option<String> {
        self.content.clone()
    }
}