pub mod mutations;
pub mod stages;

use arraygen::Arraygen;
use mongodb::bson::{Bson, Document};
use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::default::Default;

use crate::traits::prelude::*;
use crate::Cipher;
use crate::Settings;

#[derive(Debug, Default, Clone, PartialEq, Sanitize, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_array_ciphers: &mut Option<Cipher>)]
pub struct Base {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub api_url: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub web_url: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub admin_url: Option<Cipher>,
}

impl Decrypt for Base {
    fn decrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_array_ciphers() {
            *cipher = cipher.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => match d.decrypt_master() {
                    Ok(d) => Some(d.set_to_string()),
                    Err(_) => Some(d.set_to_string())
                }
            });
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl Encrypt for Base {
    fn encrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_array_ciphers() {
            *cipher = cipher.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => match d.encrypt_master() {
                    Ok(d) => Some(d),
                    Err(_) => Some(d)
                }
            });
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl From<Base> for Bson {
    fn from(value: Base) -> Self {
        Bson::Document(value.into())
    }
}

impl From<Base> for Document {
    fn from(value: Base) -> Document {
        match value.is_empty() {
            true => Document::new(),
            false => {
                let mut doc = Document::new();
                doc.insert("api_url", Bson::from(value.api_url));
                doc.insert("web_url", Bson::from(value.web_url));
                doc.insert("admin_url", Bson::from(value.admin_url));
                doc
            }
        }
    }
}

impl From<Settings> for Base {
    fn from(value: Settings) -> Self {
        Self {
            api_url: value.api_url.map(|d| Cipher::from(d.trim())),
            web_url: value.web_url.map(|d| Cipher::from(d.trim())),
            admin_url: value.admin_url.map(|d| Cipher::from(d.trim())),
        }
    }
}

impl From<&Settings> for Base {
    fn from(value: &Settings) -> Self {
        Self::from(value.clone())
    }
}

impl IsEmpty for Base {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

impl ToBson for Base {
    fn to_bson(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => self.encrypt()
        }
    }
}

impl ToJson for Base {
    fn to_json(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => self.decrypt()
        }
    }
}

impl ToOption for Base {
    fn to_option(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => Some(self.clone())
        }
    }
}