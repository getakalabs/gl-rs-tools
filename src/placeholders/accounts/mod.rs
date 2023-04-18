use mongodb::bson::Bson;
use serde::{Serialize, Serializer, Deserialize, Deserializer};


use crate::traits::{IsEmpty, ToOption};

#[derive(Debug, Clone, PartialEq)]
pub enum Account {
    Manual,
    Facebook,
    Google,
    Twitter,
    Linkedin,
    Apple,
    Instagram,
    String(String)
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self.clone() {
            Self::Manual => serializer.serialize_str("Manual"),
            Self::Facebook => serializer.serialize_str("Facebook"),
            Self::Google => serializer.serialize_str("Google"),
            Self::Twitter => serializer.serialize_str("Twitter"),
            Self::Linkedin => serializer.serialize_str("Linkedin"),
            Self::Apple => serializer.serialize_str("Apple"),
            Self::Instagram => serializer.serialize_str("Instagram"),
            Self::String(value) => serializer.serialize_str(&value)
        }
    }
}

impl<'de> Deserialize<'de> for Account {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.clone().to_lowercase().as_str() {
            "manual" => Ok(Self::Manual),
            "facebook" => Ok(Self::Facebook),
            "google" => Ok(Self::Google),
            "twitter" => Ok(Self::Twitter),
            "linkedin" => Ok(Self::Linkedin),
            "apple" => Ok(Self::Apple),
            "instagram" => Ok(Self::Instagram),
            _ => Ok(Self::String(value)),
        }
    }
}

impl IsEmpty for Account {
    fn is_empty(&self) -> bool {
        match self {
            Self::String(value) => value.is_empty(),
            _ => false
        }
    }
}

impl ToOption for Account {
    fn to_option(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => Some(self.clone())
        }
    }
}

impl ToString for Account {
    fn to_string(&self) -> String {
        match self.clone() {
            Self::Manual => String::from("Manual"),
            Self::Facebook => String::from("Facebook"),
            Self::Google => String::from("Google"),
            Self::Twitter => String::from("Twitter"),
            Self::Linkedin => String::from("Linkedin"),
            Self::Apple => String::from("Apple"),
            Self::Instagram => String::from("Instagram"),
            Self::String(value) => value
        }
    }
}

impl From<Account> for Bson {
    fn from(value: Account) -> Self {
        Bson::String(value.to_string())
    }
}

impl Account {
    pub fn new<T>(value: T) -> Self
        where T: ToString
    {
        let value = value.to_string();
        match value.to_lowercase().as_str() {
            "manual" => Self::Manual,
            "facebook" => Self::Facebook,
            "google" => Self::Google,
            "twitter" => Self::Twitter,
            "linkedin" => Self::Linkedin,
            "apple" => Self::Apple,
            "instagram" => Self::Instagram,
            _ => Self::String(value),
        }
    }
}