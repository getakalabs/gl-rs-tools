use serde::{Serialize, Deserialize};
use mongodb::bson::{Bson, Document};

use crate::traits::prelude::*;

use super::manager::Manager;
use super::payload::Payload;

const MASTER_KEY: &str = "MASTER_KEY";
const WEB_KEY: &str = "WEB_KEY";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Cipher {
    Manager(Manager),
    Payload(Payload),
    String(String),
    I32(i32),
    None
}

impl Default for Cipher {
    fn default() -> Self {
        Self::None
    }
}

impl IsEmpty for Cipher {
    fn is_empty(&self) -> bool {
        match self.clone() == Self::default() {
            true => true,
            false => match self.clone() {
                Self::String(value) => match value.to_lowercase().as_str() == "none" {
                    true => true,
                    false => false
                }
                Self::None => true,
                _ => false
            }
        }
    }
}

impl From<String> for Cipher {
    fn from(value: String) -> Self {
        let value = value;
        match value.is_empty() {
            true => Self::None,
            false => Self::new(value).unwrap()
        }
    }
}

impl From<&String> for Cipher {
    fn from(value: &String) -> Self {
        let value = &(*value).clone();
        match value.is_empty() {
            true => Self::None,
            false => Self::new(value).unwrap()
        }
    }
}

impl From<&str> for Cipher {
    fn from(value: &str) -> Self {
        let value = value;
        match value.is_empty() {
            true => Self::None,
            false => Self::new(value).unwrap()
        }
    }
}

impl From<i32> for Cipher {
    fn from(value: i32) -> Self {
        Cipher::I32(value)
    }
}

impl From<&i32> for Cipher {
    fn from(value: &i32) -> Self {
        Cipher::I32(*value)
    }
}

impl<T: IsEmpty> GetSelf<T> for Cipher {}

impl SetToI32 for Cipher {
    fn set_to_i32(&self) -> Self {
        match self {
            Self::Manager(value) => {
                match value.clone().content { 
                    Some(value) => match value.parse::<i32>() {
                        Ok(value) => Self::I32(value),
                        Err(_) => Self::None
                    },
                    None => Self::None
                }
            },
            Self::String(value) => match value.parse::<i32>() {
                Ok(value) => Self::I32(value),
                Err(_) => Self::None
            },
            Self::I32(value) => Self::I32(*value),
            _ => Self::None
        }
    }
}

impl SetToManager for Cipher {
    fn set_to_manager(&self) -> Self {
        match self {
            Self::Manager(value) => Self::Manager(value.clone()),
            Self::String(value) => Self::Manager(Manager::new(value.clone())),
            Self::I32(value) => Self::Manager(Manager::new(value.clone().to_string())),
            _ => Self::None
        }
    }
}

impl SetToString for Cipher {
    fn set_to_string(&self) -> Self {
        match self.clone() {
            Self::Manager(value) => Self::String(value.content.unwrap_or(String::default())),
            Self::String(value) => Self::String(value),
            Self::I32(value) => Self::String(value.clone().to_string()),
            _ => Self::None
        }
    }
}

impl ToBson for Cipher {
    fn to_bson(&self) -> Option<Self> {
        match self.clone().set_to_manager() {
            Self::Manager(_) => Some(self.clone()),
            _ => None
        }
    }
}

impl ToJson for Cipher {
    fn to_json(&self) -> Option<Self> {
        match self.clone().set_to_string() {
            Self::String(_) => Some(self.clone()),
            _ => None
        }
    }
}

impl ToString for Cipher {
    fn to_string(&self) -> String {
        match self.clone() {
            Self::Manager(value) => value.content.unwrap_or(String::default()),
            Self::String(value) => value,
            Self::I32(value) => value.clone().to_string(),
            _ => String::default()
        }
    }
}

impl ToOptString for Cipher {
    fn to_opt_string(&self) -> Option<String> {
        match self {
            Self::Manager(value) => match value.clone().content {
                Some(value) => match value.is_empty() {
                    true => None,
                    false => Some(value)
                },
                None => None
            },
            Self::String(value) =>match value.is_empty() {
                true => None,
                false => Some(value.clone())
            },
            Self::I32(value) => Some(value.clone().to_string()),
            _ => None
        }
    }
}

impl From<Cipher> for Bson {
    fn from(value: Cipher) -> Self {
        match value {
            Cipher::Manager(value) => {
                let mut doc = Document::new();

                doc.insert("content", value.content);
                doc.insert("hash", value.hash);
                doc.insert("is_encrypted", value.is_encrypted);

                Bson::Document(doc)
            },
            Cipher::Payload(value) => {
                let mut doc = Document::new();

                doc.insert("minimum", value.minimum);
                doc.insert("maximum", value.maximum);
                doc.insert("uppercase", value.uppercase);
                doc.insert("lowercase", value.lowercase);
                doc.insert("number", value.number);
                doc.insert("special_character", value.special_character);

                Bson::Document(doc)
            },
            Cipher::String(value) => Bson::String(value),
            Cipher::I32(value) => Bson::Int32(value),
            Cipher::None => Bson::Null
        }
    }
}

impl Cipher {
    pub fn new<C>(content: C) -> Option<Self>
        where C: ToString
    {
        Some(Self::Manager(Manager::new(content)))
    }

    pub fn new_payload(content: &Payload) -> Option<Self> {
        match content.clone().is_empty() {
            true => None,
            false => Some(Self::Payload(content.clone()))
        }
    }

    pub fn new_string<C>(content: C) -> Option<Self>
        where C: ToString
    {
        let content = content.to_string();
        match content.is_empty() {
            true => None,
            false => Some(Self::String(content))
        }
    }

    pub fn default_payload() -> Payload {
        Payload::default()
    }

    pub fn encrypt_master(&self) -> Option<Self> {
        match self.clone() {
            Cipher::Manager(value) => Some(Self::Manager(value.encrypt(MASTER_KEY))),
            Cipher::String(value) => Some(Self::Manager(Manager::new(value).encrypt(MASTER_KEY))),
            Cipher::I32(value) => Some(Self::Manager(Manager::new(value.to_string()).encrypt(MASTER_KEY))),
            _ => None
        }
    }

    pub fn encrypt_web(&self) -> Option<Self> {
        match self.clone() {
            Cipher::Manager(value) => Some(Self::Manager(value.encrypt(WEB_KEY))),
            Cipher::String(value) => Some(Self::Manager(Manager::new(value).encrypt(WEB_KEY))),
            Cipher::I32(value) => Some(Self::Manager(Manager::new(value.to_string()).encrypt(WEB_KEY))),
            _ => None
        }
    }

    pub fn decrypt_master(&self) -> Option<Self> {
        match self.clone() {
            Cipher::Manager(value) => Some(Self::Manager(value.decrypt(MASTER_KEY))),
            Cipher::String(value) => Some(Self::Manager(Manager::new(value).decrypt(MASTER_KEY))),
            Cipher::I32(value) => Some(Self::Manager(Manager::new(value.to_string()).decrypt(MASTER_KEY))),
            _ => None
        }
    }

    pub fn decrypt_web(&self) -> Option<Self> {
        match self.clone() {
            Cipher::Manager(value) => Some(Self::Manager(value.decrypt(WEB_KEY))),
            Cipher::String(value) => Some(Self::Manager(Manager::new(value).decrypt(WEB_KEY))),
            Cipher::I32(value) => Some(Self::Manager(Manager::new(value.to_string()).decrypt(WEB_KEY))),
            _ => None
        }
    }

    pub fn to_i32(&self) -> Option<i32> {
        match self.clone() {
            Cipher::Manager(value) => {
                match value.get_content() {
                    None => None,
                    Some(value) => {
                        match value.parse::<i32>() {
                            Ok(value) => Some(value),
                            Err(_) => None,
                        }
                    }
                }
            },
            Cipher::String(value) => {
                match value.is_empty() {
                    true => None,
                    false => {
                        match value.parse::<i32>() {
                            Ok(value) => Some(value),
                            Err(_) => None,
                        }
                    }
                }
            },
            Cipher::I32(value) => Some(value),
            _ => None
        }
    }

    pub fn get_cipher_i32(value: &Option<Self>) -> Option<Self> {
        value.clone().unwrap_or(Self::None).to_i32().map(Self::I32)
    }

    pub fn to_string(&self) -> Option<String> {
        match self.clone() {
            Cipher::Manager(value) => value.get_content(),
            Cipher::String(value) => {
                match value.is_empty() {
                    true => None,
                    false => Some(value)
                }
            },
            Cipher::I32(value) => Some(value.to_string()),
            _ => None
        }
    }

    pub fn get_cipher_string(value: &Option<Self>) -> Option<Self> {
        value.clone().unwrap_or(Self::None).to_string().map(Self::String)
    }
}