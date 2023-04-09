use mongodb::bson::Bson;
use serde::{Serialize, Serializer, Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq)]
pub enum MediaSource {
    Youtube,
    Vimeo,
    String(String)
}

impl Serialize for MediaSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self.clone() {
            Self::Youtube => serializer.serialize_str("Youtube"),
            Self::Vimeo => serializer.serialize_str("Vimeo"),
            Self::String(value) => serializer.serialize_str(&value)
        }
    }
}

impl<'de> Deserialize<'de> for MediaSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.clone().to_lowercase().as_str() {
            "youtube" => Ok(Self::Youtube),
            "vimeo" => Ok(Self::Vimeo),
            _ => Ok(Self::String(value)),
        }
    }
}

impl ToString for MediaSource {
    fn to_string(&self) -> String {
        match self.clone() {
            Self::Youtube => String::from("Youtube"),
            Self::Vimeo => String::from("Vimeo"),
            Self::String(value) => value
        }
    }
}

impl From<MediaSource> for Bson {
    fn from(value: MediaSource) -> Self {
        Bson::String(value.to_string())
    }
}

impl MediaSource {
    pub fn new<T>(value: T) -> Self
        where T: ToString
    {
        let value = value.to_string();
        match value.to_lowercase().as_str() {
            "youtube" => Self::Youtube,
            "vimeo" => Self::Vimeo,
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