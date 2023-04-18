use anyhow::Result;
use mongodb::bson::{Bson, Document};
use serde::{Serialize, Deserialize};
use std::default::Default;
use std::fmt::Debug;
use xsalsa20poly1305::aead::{Aead, KeyInit};
use xsalsa20poly1305::aead::generic_array::{GenericArray, typenum};
use xsalsa20poly1305::aead::generic_array::typenum::Unsigned;
use xsalsa20poly1305::XSalsa20Poly1305;

use crate::traits::GetString;
use crate::traits::IsEmpty;
use crate::traits::ToOption;

use crate::ciphers::CipherAction;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CipherManager {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_encrypted: Option<bool>,
}

impl From<CipherManager> for Bson {
    fn from(value: CipherManager) -> Self {
        Bson::Document(value.into())
    }
}

impl From<CipherManager> for Document {
    fn from(value: CipherManager) -> Document {
        match value.is_empty() {
            true => Document::new(),
            false => {
                let mut doc = Document::new();
                doc.insert("content", Bson::from(value.content));
                doc.insert("hash", Bson::from(value.hash));
                doc.insert("is_encrypted", Bson::from(value.is_encrypted));
                doc
            }
        }
    }
}

impl From<String> for CipherManager {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl GetString for CipherManager {
    fn get_string(&self) -> Option<String> {
        match self.clone().content {
            Some(value) => match value.is_empty() {
                true => None,
                false => Some(value)
            },
            None => None
        }
    }
}

impl IsEmpty for CipherManager {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

impl ToOption for CipherManager {
    fn to_option(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => Some(self.clone())
        }
    }
}

impl ToString for CipherManager {
    fn to_string(&self) -> String {
        match self.content.clone() {
            Some(value) => value,
            None => String::default()
        }
    }
}

impl CipherManager {
    pub(super) fn new<C>(content: C) -> Self
        where C: ToString
    {
        Self {
            content: Some(content.to_string()),
            hash: None,
            is_encrypted: Some(false),
        }
    }

    pub(super) fn is_ready(&self, action: CipherAction) -> bool {
        let is_encrypted = self.is_encrypted.unwrap_or(false);
        let is_empty_content = self.content.clone().unwrap_or(String::default()).is_empty();
        let is_empty_hash = self.hash.clone().unwrap_or(String::default()).is_empty();

        match action {
            CipherAction::Encrypt => !is_encrypted && !is_empty_content,
            CipherAction::Decrypt => is_encrypted && !is_empty_content && !is_empty_hash,
        }
    }

    pub(super) fn encrypt<K>(&self, key: K) -> Result<Self>
        where K: ToString
    {
        // Check if encryption is ready
        if !self.is_ready(CipherAction::Encrypt) {
            return Err(anyhow::anyhow!("Unable to encrypt content"));
        }

        // Get manager
        let mut manager = self.clone();

        // Encrypt content
        let hash = base64_url::decode(&super::generate())?;
        let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);
        let cipher = XSalsa20Poly1305::new(GenericArray::from_slice(&hash));
        let content = match cipher.encrypt(&nonce, manager.content.clone().unwrap_or(String::default()).as_bytes()) {
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("Unable to encrypt content"))
        };

        // Populate manager content
        manager.content = Some(base64_url::encode(&[&nonce[..], &content[..]].concat()));

        // Encrypt hash
        let binding = base64_url::decode(&std::env::var(key.to_string())?)?;
        let key =  GenericArray::from_slice(&binding);
        let cipher = XSalsa20Poly1305::new(key);

        // Encrypt hash
        let nonce = XSalsa20Poly1305::generate_nonce(&mut rand::rngs::OsRng);
        let hash = match cipher.encrypt(&nonce, hash.as_slice()) {
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("Unable to encrypt hash"))
        };

        // Populate manager
        manager.hash = Some(base64_url::encode( &[&nonce[..], &hash[..]].concat()));
        manager.is_encrypted = Some(true);

        // Return manager
        Ok(manager)
    }

    pub(super) fn decrypt<K>(&self, key: K) -> Result<Self>
        where K: ToString
    {
        // Check if decryption is ready
        if !self.is_ready(CipherAction::Decrypt) {
            return Err(anyhow::anyhow!("Unable to decrypt content"));
        }

        // Get manager
        let mut manager = self.clone();

        // Decrypt hash
        let binding = base64_url::decode(&std::env::var(key.to_string())?)?;
        let key =  GenericArray::from_slice(&binding);
        let cipher = XSalsa20Poly1305::new(key);
        let hash = base64_url::decode(&manager.hash.clone().unwrap_or(String::default()))?;
        let nonce = GenericArray::from_slice(&hash[..typenum::U24::to_usize()]);
        let hash = match cipher.decrypt(nonce, &hash[typenum::U24::to_usize()..]) {
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("Unable to decrypt hash"))
        };

        // Decrypt content
        let cipher = XSalsa20Poly1305::new(GenericArray::from_slice(&hash));
        let content = base64_url::decode(&manager.content.clone().unwrap_or(String::default()))?;
        let nonce = GenericArray::from_slice(&content[..typenum::U24::to_usize()]);
        let content = match cipher.decrypt(nonce, &content[typenum::U24::to_usize()..]) {
            Ok(value) => value,
            Err(_) => return Err(anyhow::anyhow!("Unable to decrypt content"))
        };

        // Populate manager
        manager.content = Some(String::from_utf8_lossy(content.as_slice()).to_string());
        manager.is_encrypted = Some(false);

        // Return manager
        Ok(manager)
    }
}