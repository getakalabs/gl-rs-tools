use chrono::{DateTime as ChronoDateTime, Utc, NaiveDateTime};
use mongodb::bson::{Bson, DateTime as BsonDateTime};
use serde::{Serialize, Deserialize};
use crate::traits::{GetDateTimeBson, GetDateTimeChrono, IsEmpty, SetToDateTimeBson, SetToDateTimeChrono, ToBson, ToJson};

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

impl From<&ChronoDateTime<Utc>> for MongoDateTime {
    fn from(value: &ChronoDateTime<Utc>) -> Self {
        Self::ChronoDateTime(*value).set_to_date_time_bson()
    }
}

impl From<MongoDateTime> for Bson {
    fn from(value: MongoDateTime) -> Self {
        match value.set_to_date_time_bson() {
            MongoDateTime::BsonDateTime(value) => Bson::DateTime(value),
            _ => Bson::Null
        }
    }
}

impl From<Option<String>> for MongoDateTime {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(value) => Self::from(value),
            None => Self::None
        }
    }
}

impl From<String> for MongoDateTime {
    fn from(value: String) -> Self {
        match crate::parsers::dates::naive_date_time(&value) {
            Some(value) => Self::from(ChronoDateTime::<Utc>::from_utc(value, Utc)),
            None => match value.trim().is_empty() {
                true => Self::None,
                false => Self::String(value.trim().to_string())
            }
        }
    }
}

impl From<&String> for MongoDateTime {
    fn from(value: &String) -> Self {
        Self::from(value.to_string())
    }
}

impl From<&str> for MongoDateTime {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<BsonDateTime> for MongoDateTime {
    fn from(value: BsonDateTime) -> Self {
        Self::BsonDateTime(value)
    }
}

impl From<ChronoDateTime<Utc>> for MongoDateTime {
    fn from(value: ChronoDateTime<Utc>) -> Self {
        Self::ChronoDateTime(value).set_to_date_time_bson()
    }
}

impl From<NaiveDateTime> for MongoDateTime {
    fn from(value: NaiveDateTime) -> Self {
        Self::ChronoDateTime(ChronoDateTime::from_utc(value, Utc))
            .set_to_date_time_bson()
    }
}

impl GetDateTimeBson for MongoDateTime {
    fn get_date_time_bson(&self) -> Option<BsonDateTime> {
        match self.set_to_date_time_bson() {
            Self::BsonDateTime(value) => Some(value),
            _ => None
        }
    }
}

impl GetDateTimeChrono for MongoDateTime {
    fn get_date_time_chrono(&self) -> Option<ChronoDateTime<Utc>> {
        match self.set_to_date_time_chrono() {
            Self::ChronoDateTime(value) => Some(value),
            _ => None
        }
    }
}

impl IsEmpty for MongoDateTime {
    fn is_empty(&self) -> bool {
        match self {
            Self::None => true,
            Self::String(value) => value.trim().is_empty(),
            _ => false
        }
    }
}

impl SetToDateTimeBson for MongoDateTime {
    fn set_to_date_time_bson(&self) -> Self {
        match self.clone() {
            Self::BsonDateTime(value) => Self::BsonDateTime(value),
            Self::ChronoDateTime(value) => Self::BsonDateTime(BsonDateTime::from(value)),
            Self::String(value) => match Self::from(value) {
                Self::BsonDateTime(value) => Self::BsonDateTime(value),
                Self::ChronoDateTime(value) => Self::BsonDateTime(BsonDateTime::from(value)),
                _ => Self::None
            },
            Self::None => Self::None
        }
    }
}

impl SetToDateTimeChrono for MongoDateTime {
    fn set_to_date_time_chrono(&self) -> Self {
        match self.clone() {
            Self::BsonDateTime(value) => Self::ChronoDateTime(value.to_chrono()),
            Self::ChronoDateTime(value) => Self::ChronoDateTime(value),
            Self::String(value) => match Self::from(value) {
                Self::BsonDateTime(value) => Self::ChronoDateTime(value.to_chrono()),
                Self::ChronoDateTime(value) => Self::ChronoDateTime(value),
                _ => Self::None
            },
            Self::None => Self::None
        }
    }
}

impl ToBson for MongoDateTime {
    fn to_bson(&self) -> Option<Self> {
        match self.set_to_date_time_bson() {
            Self::BsonDateTime(value) => Some(Self::BsonDateTime(value)),
            _ => None
        }
    }
}

impl ToJson for MongoDateTime {
    fn to_json(&self) -> Option<Self> {
        match self.set_to_date_time_chrono() {
            Self::ChronoDateTime(value) => Some(Self::String(value.to_rfc3339())),
            _ => None
        }
    }
}

impl MongoDateTime {
    pub fn new() -> Self {
        Self::BsonDateTime(BsonDateTime::now())
    }
}