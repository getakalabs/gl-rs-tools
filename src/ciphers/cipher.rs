use anyhow::Result;
use mongodb::bson::Bson;
use serde::{Serialize, Deserialize};

use crate::traits::GetI32;
use crate::traits::GetString;
use crate::traits::IsEmpty;
use crate::traits::SetToCipher;
use crate::traits::SetToI32;
use crate::traits::SetToString;
use crate::traits::ToBson;
use crate::traits::ToJson;

use super::manager::CipherManager;
use super::payload::Payload;

const MASTER_KEY: &str = "MASTER_KEY";
const WEB_KEY: &str = "WEB_KEY";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Cipher {
    CipherManager(CipherManager),
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

impl GetI32 for Cipher {
    fn get_i32(&self) -> Option<i32> {
        match self.set_to_i32() {
            Self::I32(value) => Some(value),
            _ => None
        }
    }
}

impl GetString for Cipher {
    fn get_string(&self) -> Option<String> {
        match self.set_to_string() {
            Self::String(value) => match value.is_empty() {
                true => None,
                false => Some(value)
            },
            _ => None
        }
    }
}

impl IsEmpty for Cipher {
    fn is_empty(&self) -> bool {
        match self {
            Self::CipherManager(value) => value.is_empty(),
            Self::Payload(value) => value.is_empty(),
            Self::String(value) => value.is_empty(),
            Self::None => true,
            _ => false
        }
    }
}

impl SetToCipher for Cipher {
    fn set_to_cipher(&self) -> Self {
        match self {
            Self::CipherManager(value) => Self::CipherManager(value.clone()),
            Self::String(value) => Self::CipherManager(CipherManager::from(value.to_string())),
            Self::I32(value) => Self::CipherManager(CipherManager::from(value.to_string())),
            _ => Self::None
        }
    }
}

impl SetToI32 for Cipher {
    fn set_to_i32(&self) -> Self {
        match self {
            Self::CipherManager(value) => {
                match value.to_string().parse::<i32>() {
                    Ok(value) => Self::I32(value),
                    Err(_) => Self::None
                }
            }
            Self::String(value) => {
                match value.parse::<i32>() {
                    Ok(value) => Self::I32(value),
                    Err(_) => Self::None
                }
            },
            _ => Self::None
        }
    }
}

impl SetToString for Cipher {
    fn set_to_string(&self) -> Self {
        match self {
            Self::CipherManager(value) => Self::String(value.to_string()),
            Self::I32(value) => Self::String(value.to_string()),
            Self::String(value) => Self::String(value.to_string()),
            _ => Self::None
        }
    }
}

impl ToBson for Cipher {
    fn to_bson(&self) -> Option<Self> {
        match self.set_to_cipher() {
            Self::CipherManager(value) => Some(Self::CipherManager(value)),
            _ => None
        }
    }
}

impl ToJson for Cipher {
    fn to_json(&self) -> Option<Self> {
        match self.set_to_string() {
            Self::String(value) => Some(Self::String(value)),
            _ => None
        }
    }
}

impl From<Cipher> for Bson {
    fn from(value: Cipher) -> Self {
        match value {
            Cipher::CipherManager(value) => Bson::from(value),
            Cipher::String(value) => Bson::from(value),
            Cipher::I32(value) => Bson::from(value),
            _ => Bson::Null
        }
    }
}

impl From<Payload> for Cipher {
    fn from(value: Payload) -> Self {
        match value.is_empty() {
            true => Self::None,
            false => Self::Payload(value)
        }
    }
}

impl From<&Payload> for Cipher {
    fn from(value: &Payload) -> Self {
        Self::Payload(value.clone())
    }
}

impl From<String> for Cipher {
    fn from(value: String) -> Self {
        match value.is_empty() {
            true => Self::None,
            false => Self::new(value)
        }
    }
}

impl From<&String> for Cipher {
    fn from(value: &String) -> Self {
        match value.is_empty() {
            true => Self::None,
            false => Self::new(value)
        }
    }
}

impl From<&str> for Cipher {
    fn from(value: &str) -> Self {
        match value.is_empty() {
            true => Self::None,
            false => Self::new(value)
        }
    }
}

impl From<usize> for Cipher {
    fn from(value: usize) -> Self {
        match value == 0 {
            true => Self::None,
            false => Self::new(value)
        }
    }
}

impl From<i32> for Cipher {
    fn from(value: i32) -> Self {
        match value == 0 {
            true => Self::None,
            false => Self::new(value)
        }
    }
}

impl ToString for Cipher {
    fn to_string(&self) -> String {
        match self.set_to_string() {
            Self::String(value) => value,
            _ => String::new()
        }
    }
}

impl Cipher {
    pub fn new<T>(value: T) -> Self
        where T: ToString
    {
        Self::CipherManager(CipherManager::new(value))
    }

    pub fn payload() -> Payload {
        Payload::default()
    }

    pub fn encrypt_master(&self) -> Result<Self> {
        match self.set_to_cipher() {
            Self::CipherManager(value) => {
                let value = value.encrypt(MASTER_KEY)?;
                Ok(Self::CipherManager(value))
            },
            _ => Err(anyhow::anyhow!("Unable to encrypt with master key"))
        }
    }

    pub fn encrypt_web(&self) -> Result<Self> {
        match self.set_to_cipher() {
            Self::CipherManager(value) => {
                let value = value.encrypt(WEB_KEY)?;
                Ok(Self::CipherManager(value))
            },
            _ => Err(anyhow::anyhow!("Unable to encrypt with web key"))
        }
    }

    pub fn decrypt_master(&self) -> Result<Self> {
        match self.set_to_cipher() {
            Self::CipherManager(value) => {
                let value = value.decrypt(MASTER_KEY)?;
                Ok(Self::CipherManager(value))
            },
            _ => Err(anyhow::anyhow!("Unable to decrypt with master key"))
        }
    }

    pub fn decrypt_web(&self) -> Result<Self> {
        match self.set_to_cipher() {
            Self::CipherManager(value) => {
                let value = value.decrypt(WEB_KEY)?;
                Ok(Self::CipherManager(value))
            },
            _ => Err(anyhow::anyhow!("Unable to decrypt with web key"))
        }
    }
}