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
pub struct Mailer {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub sender: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub username: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub password: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub smtp_host: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub service: Option<Cipher>,
}

impl Decrypt for Mailer {
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

impl Encrypt for Mailer {
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

impl From<Mailer> for Bson {
    fn from(value: Mailer) -> Self {
        Bson::Document(value.into())
    }
}

impl From<Mailer> for Document {
    fn from(value: Mailer) -> Document {
        match value.is_empty() {
            true => Document::new(),
            false => {
                let mut doc = Document::new();
                doc.insert("sender", Bson::from(value.sender));
                doc.insert("username", Bson::from(value.username));
                doc.insert("password", Bson::from(value.password));
                doc.insert("smtp_host", Bson::from(value.smtp_host));
                doc.insert("service", Bson::from(value.service));
                doc
            }
        }
    }
}

impl From<Settings> for Mailer {
    fn from(value: Settings) -> Self {
        let sender = Cipher::from(value.sender.map_or(String::default(), |d| d));
        let username = Cipher::from(value.username.map_or(String::default(), |d| d));
        let password = Cipher::from(value.password.map_or(String::default(), |d| d));
        let smtp_host = Cipher::from(value.smtp_host.map_or(String::default(), |d| d));
        let service = Cipher::from(value.service.map_or(String::default(), |d| d));

        Self {
            sender: Some(sender),
            username: Some(username),
            password: Some(password),
            smtp_host: Some(smtp_host),
            service: Some(service)
        }
    }
}

impl From<&Settings> for Mailer {
    fn from(value: &Settings) -> Self {
        Self::from(value.clone())
    }
}

impl IsEmpty for Mailer {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

impl ToBson for Mailer {
    fn to_bson(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => self.encrypt()
        }
    }
}

impl ToJson for Mailer {
    fn to_json(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => self.decrypt()
        }
    }
}

impl ToOption for Mailer {
    fn to_option(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => Some(self.clone())
        }
    }
}