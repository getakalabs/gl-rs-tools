use chrono::{DateTime as ChronoDateTime, Utc, NaiveDateTime};
use mongodb::bson::{Bson, DateTime as BsonDateTime};
use serde::{Serialize, Deserialize};
use std::borrow::Cow;

use crate::traits::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MongoDateTime {
    BsonDateTime(BsonDateTime),
    ChronoDateTime(ChronoDateTime<Utc>),
    String(String),
    None
}

impl Default for MongoDateTime {
    fn default() -> Self {
        Self::None
    }
}

impl IsEmpty for MongoDateTime {
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

impl From<String> for MongoDateTime {
    fn from(value: String) -> Self {
        Self::src(value)
    }
}

impl From<&String> for MongoDateTime {
    fn from(value: &String) -> Self {
        Self::src(value.clone())
    }
}

impl From<Option<String>> for MongoDateTime {
    fn from(value: Option<String>) -> Self {
        Self::src(value.unwrap_or(String::default()))
    }
}

impl From<&Option<String>> for MongoDateTime {
    fn from(value: &Option<String>) -> Self {
        Self::src(value.clone().unwrap_or(String::default()))
    }
}

impl From<&str> for MongoDateTime {
    fn from(value: &str) -> Self {
        Self::src(value.to_string())
    }
}

impl From<Cow<'_, str>> for MongoDateTime {
    fn from(value: Cow<'_, str>) -> Self {
        Self::src(value)
    }
}

impl From<BsonDateTime> for MongoDateTime {
    fn from(value: BsonDateTime) -> Self {
        Self::BsonDateTime(value)
    }
}

impl From<ChronoDateTime<Utc>> for MongoDateTime {
    fn from(value: ChronoDateTime<Utc>) -> Self {
        Self::ChronoDateTime(value).set_to_bson_date_time()
    }
}

impl From<&ChronoDateTime<Utc>> for MongoDateTime {
    fn from(value: &ChronoDateTime<Utc>) -> Self {
        Self::ChronoDateTime(*value).set_to_bson_date_time()
    }
}


impl From<NaiveDateTime> for MongoDateTime {
    fn from(value: NaiveDateTime) -> Self {
        let value = ChronoDateTime::<Utc>::from_utc(value, Utc);
        Self::ChronoDateTime(value).set_to_bson_date_time()
    }
}

impl From<MongoDateTime> for Bson {
    fn from(value: MongoDateTime) -> Self {
        match value.get_bson_date_time() {
            None => Bson::Null,
            Some(data) => Bson::DateTime(data)
        }
    }
}

impl GetBsonDateTime for MongoDateTime {
    fn get_bson_date_time(&self) -> Option<BsonDateTime> {
        match self.set_to_bson_date_time() {
            Self::BsonDateTime(value) => Some(value),
            _ => None
        }
    }
}

impl GetChronoDateTime for MongoDateTime {
    fn get_chrono_date_time(&self) -> Option<ChronoDateTime<Utc>> {
        match self.set_to_chrono_date_time() {
            Self::ChronoDateTime(value) => Some(value),
            _ => None
        }
    }
}

impl GetString for MongoDateTime {
    fn get_string(&self) -> Option<Self> {
        match self.clone().set_to_string() {
            Self::String(value) => Some(Self::String(value)),
            _ => None
        }
    }
}

impl GetStringDateTime for MongoDateTime {
    fn get_string_date_time(&self) -> Option<String> {
        match self.set_to_string_date_time() {
            Self::String(value) => Some(value),
            _ => None
        }
    }
}

impl GetMongoDateTime for MongoDateTime {
    fn get_mongo_date_time(&self) -> Option<MongoDateTime> {
        match self.clone().set_to_bson_date_time() {
            Self::BsonDateTime(value) => Some(Self::BsonDateTime(value)),
            _ => None
        }
    }
}

impl<T: IsEmpty> GetSelf<T> for MongoDateTime{}

impl ToString for MongoDateTime {
    fn to_string(&self) -> String {
        match self {
            Self::BsonDateTime(value) => Self::ChronoDateTime(value.to_chrono()).to_string(),
            Self::ChronoDateTime(value) => value.to_rfc3339(),
            Self::String(value) => {
                match crate::parsers::naive_date_time(value) {
                    None => String::from("None"),
                    Some(value) => {
                        let value = ChronoDateTime::<Utc>::from_utc(value, Utc);
                        value.to_rfc3339()
                    }
                }
            }
            Self::None => String::from("None")
        }
    }
}

impl<T: ToString> Src<T> for MongoDateTime {
    fn src(value: T) -> Self {
        match crate::parsers::naive_date_time(value) {
            None => Self::None,
            Some(value) => {
                let value = ChronoDateTime::<Utc>::from_utc(value, Utc);
                let value:BsonDateTime = value.into();
                Self::BsonDateTime(value)
            }
        }
    }
}

impl SetToBsonDateTime for MongoDateTime {
    fn set_to_bson_date_time(&self) -> Self {
        match self.clone() {
            Self::BsonDateTime(value) => Self::BsonDateTime(value),
            Self::ChronoDateTime(value) => {
                let value:BsonDateTime = value.into();
                Self::BsonDateTime(value)
            }
            Self::String(value) => {
                match crate::parsers::naive_date_time(value) {
                    None => Self::None,
                    Some(value) => {
                        let value = ChronoDateTime::<Utc>::from_utc(value, Utc);
                        let value:BsonDateTime = value.into();
                        Self::BsonDateTime(value)
                    }
                }
            }
            Self::None => Self::None
        }
    }
}

impl SetToChronoDateTime for MongoDateTime {
    fn set_to_chrono_date_time(&self) -> Self {
        match self.clone() {
            Self::BsonDateTime(value) => Self::ChronoDateTime(value.to_chrono()),
            Self::ChronoDateTime(value) => Self::ChronoDateTime(value),
            Self::String(value) => {
                match crate::parsers::naive_date_time(value) {
                    None => Self::None,
                    Some(value) => {
                        let value = ChronoDateTime::<Utc>::from_utc(value, Utc);
                        Self::ChronoDateTime(value)
                    }
                }
            }
            Self::None => Self::None
        }
    }
}

impl SetToString for MongoDateTime {
    fn set_to_string(&self) -> Self {
        match self {
            Self::BsonDateTime(value) => Self::ChronoDateTime(value.to_chrono()).set_to_string(),
            Self::ChronoDateTime(value) => Self::String(value.to_rfc3339()),
            Self::String(value) => Self::String(value.to_string()),
            Self::None => Self::None
        }
    }
}

impl SetToStringDateTime for MongoDateTime {
    fn set_to_string_date_time(&self) -> Self {
        self.clone().set_to_string()
    }
}

impl ToBson for MongoDateTime {
    fn to_bson(&self) -> Option<Self> {
        match self.clone().set_to_bson_date_time() {
            Self::BsonDateTime(_) => Some(self.clone()),
            _ => None
        }
    }
}

impl ToBsonDateTime for MongoDateTime {
    fn to_bson_date_time(&self) -> Option<BsonDateTime> {
        match self.clone() {
            Self::BsonDateTime(value) => Some(value),
            Self::ChronoDateTime(value) => {
                let value:BsonDateTime = value.into();
                Some(value)
            }
            Self::String(value) => {
                match crate::parsers::naive_date_time(value) {
                    None => None,
                    Some(value) => {
                        let value = ChronoDateTime::<Utc>::from_utc(value, Utc);
                        let value:BsonDateTime = value.into();
                        Some(value)
                    }
                }
            }
            Self::None => None
        }
    }
}

impl ToChronoDateTime for MongoDateTime {
    fn to_chrono_date_time(&self) -> Option<ChronoDateTime<Utc>> {
        match self.clone() {
            Self::BsonDateTime(value) =>  Some(value.to_chrono()),
            Self::ChronoDateTime(value) => Some(value),
            Self::String(value) => {
                match crate::parsers::naive_date_time(value) {
                    None => None,
                    Some(value) => {
                        let value = ChronoDateTime::<Utc>::from_utc(value, Utc);
                        Some(value)
                    }
                }
            }
            Self::None => None
        }
    }
}

impl ToOptString for MongoDateTime {
    fn to_opt_string(&self) -> Option<String> {
        match self {
            Self::BsonDateTime(value) => Some(value.to_chrono().to_rfc3339()),
            Self::ChronoDateTime(value) => Some(value.to_rfc3339()),
            Self::String(value) => match crate::parsers::naive_date_time(value) {
                None => None,
                Some(value) => {
                    let value = ChronoDateTime::<Utc>::from_utc(value, Utc);
                    Some(value.to_rfc3339())
                }
            },
            Self::None => None
        }
    }
}

impl ToJson for MongoDateTime {
    fn to_json(&self) -> Option<Self> {
        match self.clone().set_to_string() {
            Self::String(_) => Some(self.clone()),
            _ => None
        }
    }
}

impl MongoDateTime {
    pub fn new() -> Self {
        Self::BsonDateTime(BsonDateTime::now())
    }
}