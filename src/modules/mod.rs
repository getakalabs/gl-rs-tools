use mongodb::bson::Bson;
use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq)]
pub enum Module {
    Base,
    Mailer,
    Mailgun,
    Paseto,
    S3,
    SES,
    String(String)
}

impl Serialize for Module {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self.clone() {
            Self::Base => serializer.serialize_str("Base"),
            Self::Mailer => serializer.serialize_str("Mailer"),
            Self::Mailgun => serializer.serialize_str("Mailgun"),
            Self::Paseto => serializer.serialize_str("Paseto"),
            Self::S3 => serializer.serialize_str("S3"),
            Self::SES => serializer.serialize_str("SES"),
            Self::String(value) => serializer.serialize_str(&value)
        }
    }
}

impl<'de> Deserialize<'de> for Module {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.clone().to_lowercase().as_str() {
            "base" => Ok(Self::Base),
            "mailer" => Ok(Self::Mailer),
            "mailgun" => Ok(Self::Mailgun),
            "paseto" => Ok(Self::Paseto),
            "s3" => Ok(Self::S3),
            "ses" => Ok(Self::SES),
            _ => Ok(Self::String(value)),
        }
    }
}

impl ToString for Module {
    fn to_string(&self) -> String {
        match self.clone() {
            Self::Base => String::from("Base"),
            Self::Mailer => String::from("Mailer"),
            Self::Mailgun => String::from("Mailgun"),
            Self::Paseto => String::from("Paseto"),
            Self::S3 => String::from("S3"),
            Self::SES => String::from("SES"),
            Self::String(value) => value
        }
    }
}

impl From<Module> for Bson {
    fn from(value: Module) -> Self {
        Bson::String(value.to_string())
    }
}

impl Module {
    // Convert string into Module
    pub fn new<T>(value: T) -> Self
        where T: ToString
    {
        let value = value.to_string();
        match value.to_lowercase().as_str() {
            "base" => Self::Base,
            "mailer" => Self::Mailer,
            "mailgun" => Self::Mailgun,
            "paseto" => Self::Paseto,
            "s3" => Self::S3,
            "ses" => Self::SES,
            _ => Self::String(value),
        }
    }

    pub fn new_option<T>(value: T) -> Option<Self>
        where T: ToString
    {
        Some(Self::new(value))
    }

    pub fn get_string(&self) -> String {
        let value:String = self.clone().to_string();

        value
    }
}