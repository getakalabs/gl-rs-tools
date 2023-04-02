use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use std::borrow::Cow;
use std::str::FromStr;
use mongodb::bson::Bson;

use crate::traits::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MongoObjectId {
    ObjectId(ObjectId),
    String(String),
    None
}

impl Default for MongoObjectId {
    fn default() -> Self {
        Self::None
    }
}

impl From<String> for MongoObjectId {
    fn from(value: String) -> Self {
        Self::src(value)
    }
}

impl From<&String> for MongoObjectId {
    fn from(value: &String) -> Self {
        Self::src(value.clone())
    }
}

impl From<Option<String>> for MongoObjectId {
    fn from(value: Option<String>) -> Self {
        Self::src(value.unwrap_or(String::default()))
    }
}

impl From<&Option<String>> for MongoObjectId {
    fn from(value: &Option<String>) -> Self {
        Self::src(value.clone().unwrap_or(String::default()))
    }
}

impl From<&str> for MongoObjectId {
    fn from(value: &str) -> Self {
        Self::src(value)
    }
}

impl From<Cow<'_, str>> for MongoObjectId {
    fn from(value: Cow<'_, str>) -> Self {
        Self::src(value)
    }
}

impl From<ObjectId> for MongoObjectId {
    fn from(value: ObjectId) -> Self {
        Self::ObjectId(value)
    }
}

impl From<MongoObjectId> for Bson {
    fn from(value: MongoObjectId) -> Self {
        match value.get_object_id() {
            None => Bson::Null,
            Some(data) => Bson::ObjectId(data)
        }
    }
}

impl<T: ToString> Src<T> for MongoObjectId {
    fn src(value: T) -> Self {
        match ObjectId::from_str(&value.to_string()) {
            Ok(value) => Self::ObjectId(value),
            Err(_) => Self::None
        }
    }
}

impl ToString for MongoObjectId {
    fn to_string(&self) -> String {
        match self {
            Self::ObjectId(value) => value.to_string(),
            Self::String(value) => value.to_string(),
            Self::None => String::from("None")
        }
    }
}

impl GetObjectId for MongoObjectId {
    fn get_object_id(&self) -> Option<ObjectId> {
        match self.clone().set_to_object_id() {
            Self::ObjectId(value) => Some(value),
            _ => None
        }
    }
}

impl GetString for MongoObjectId {
    fn get_string(&self) -> Option<Self> {
        match self.clone().set_to_string() {
            Self::String(value) => Some(Self::String(value)),
            _ => None
        }
    }
}

impl GetStringId for MongoObjectId {
    fn get_string_id(&self) -> Option<String> {
        match self.clone().set_to_string() {
            Self::String(value) => Some(value),
            _ => None
        }
    }
}

impl GetMongoObjectId for MongoObjectId {
    fn get_mongo_object_id(&self) -> Option<MongoObjectId> {
        match self.clone().set_to_object_id() {
            Self::ObjectId(value) => Some(Self::ObjectId(value)),
            _ => None
        }
    }
}

impl<T: IsEmpty> GetSelf<T> for MongoObjectId {}

impl IsEmpty for MongoObjectId {
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

impl ToOptString for MongoObjectId {
    fn to_opt_string(&self) -> Option<String> {
        match self {
            Self::ObjectId(value) => Some(value.to_string()),
            Self::String(value) => Some(value.to_string()),
            Self::None => None
        }
    }
}

impl ToBson for MongoObjectId {
    fn to_bson(&self) -> Option<Self> {
        match self.clone().set_to_object_id() {
            Self::ObjectId(_) => Some(self.clone()),
            _ => None
        }
    }
}

impl SetToObjectId for MongoObjectId {
    fn set_to_object_id(&self) -> Self {
        match self {
            Self::ObjectId(value) => Self::ObjectId(*value),
            Self::String(value) => match ObjectId::from_str(value) {
                Ok(value) => Self::ObjectId(value),
                Err(_) => Self::None
            },
            Self::None => Self::None
        }
    }
}

impl SetToString for MongoObjectId {
    fn set_to_string(&self) -> Self {
        match self {
            Self::ObjectId(value) => Self::String(value.to_string()),
            Self::String(value) => Self::String(value.to_string()),
            Self::None => Self::None
        }
    }
}

impl ToJson for MongoObjectId {
    fn to_json(&self) -> Option<Self> {
        match self.clone().set_to_string() {
            Self::String(_) => Some(self.clone()),
            _ => None
        }
    }
}

impl MongoObjectId {
    pub fn new() -> Self {
        Self::ObjectId(ObjectId::new())
    }
}


