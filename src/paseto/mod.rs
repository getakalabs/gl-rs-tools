pub mod impls;
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
pub struct Paseto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub app_name: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub access_token_key_unit: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub access_token_key_time: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub access_token_key_signing: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub refresh_token_key_unit: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub refresh_token_key_time: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub refresh_token_key_signing: Option<Cipher>,
}

impl Decrypt for Paseto {
    fn decrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_array_ciphers() {
            *cipher = cipher.clone().and_then(|d| match d.is_empty() {
                true => None,
                false =>  match d.decrypt_master() {
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

impl Encrypt for Paseto {
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

impl From<Paseto> for Bson {
    fn from(value: Paseto) -> Self {
        Bson::Document(value.into())
    }
}

impl From<Paseto> for Document {
    fn from(value: Paseto) -> Document {
        match value.is_empty() {
            true => Document::new(),
            false => {
                let mut doc = Document::new();
                doc.insert("app_name", Bson::from(value.app_name));
                doc.insert("access_token_key_unit", Bson::from(value.access_token_key_unit));
                doc.insert("access_token_key_time", Bson::from(value.access_token_key_time));
                doc.insert("access_token_key_signing", Bson::from(value.access_token_key_signing));
                doc.insert("refresh_token_key_unit", Bson::from(value.refresh_token_key_unit));
                doc.insert("refresh_token_key_time", Bson::from(value.refresh_token_key_time));
                doc.insert("refresh_token_key_signing", Bson::from(value.refresh_token_key_signing));
                doc
            }
        }
    }
}

impl From<Settings> for Paseto {
    fn from(value: Settings) -> Self {
        let app_name = Cipher::from(value.app_name.map_or("App".to_string(), |d| d));
        let access_token_key_unit = Cipher::from(value.access_token_key_unit.map_or(5, |d| d.get_i32().unwrap_or(5)));
        let access_token_key_time = Cipher::from(value.access_token_key_time.map_or("Minutes".to_string(), |d| d));
        let access_token_key_signing = Cipher::from(value.access_token_key_signing.map_or(crate::ciphers::generate(), |d| d));
        let refresh_token_key_unit = Cipher::from(value.refresh_token_key_unit.map_or(5, |d| d.get_i32().unwrap_or(5)));
        let refresh_token_key_time = Cipher::from(value.refresh_token_key_time.map_or("Minutes".to_string(), |d| d));
        let refresh_token_key_signing = Cipher::from(value.refresh_token_key_signing.map_or(crate::ciphers::generate(), |d| d));

        Self {
            app_name: Some(app_name),
            access_token_key_unit: Some(access_token_key_unit),
            access_token_key_time: Some(access_token_key_time),
            access_token_key_signing: Some(access_token_key_signing),
            refresh_token_key_unit: Some(refresh_token_key_unit),
            refresh_token_key_time: Some(refresh_token_key_time),
            refresh_token_key_signing: Some(refresh_token_key_signing),
        }
    }
}

impl From<&Settings> for Paseto {
    fn from(value: &Settings) -> Self {
        Self::from(value.clone())
    }
}

impl From<String> for Paseto {
    fn from(value: String) -> Self {
        Self {
            app_name: Some(Cipher::from(value)),
            access_token_key_unit: Some(Cipher::from(5)),
            access_token_key_time: Some(Cipher::from("Minutes")),
            access_token_key_signing: Some(Cipher::from(crate::ciphers::generate())),
            refresh_token_key_unit: Some(Cipher::from(30)),
            refresh_token_key_time: Some(Cipher::from("Minutes")),
            refresh_token_key_signing: Some(Cipher::from(crate::ciphers::generate())),
        }
    }
}

impl IsEmpty for Paseto {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

impl ToBson for Paseto {
    fn to_bson(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => self.encrypt()
        }
    }
}

impl ToJson for Paseto {
    fn to_json(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => self.decrypt()
        }
    }
}

impl ToOption for Paseto {
    fn to_option(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => Some(self.clone())
        }
    }
}