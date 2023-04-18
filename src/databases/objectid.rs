use mongodb::bson::{Bson, oid::ObjectId};
use serde::{Serialize, Deserialize};
use std::str::FromStr;

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

impl From<MongoObjectId> for Bson {
    fn from(value: MongoObjectId) -> Self {
        match value.set_to_object_id() {
            MongoObjectId::ObjectId(value) => Bson::ObjectId(value),
            _ => Bson::Null
        }
    }
}

impl From<Option<String>> for MongoObjectId {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(value) => Self::from(value),
            None => Self::None
        }
    }
}

impl From<String> for MongoObjectId {
    fn from(value: String) -> Self {
        match ObjectId::from_str(&value) {
            Ok(value) => Self::ObjectId(value),
            Err(_) => match value.trim().is_empty() {
                true => Self::None,
                false => Self::String(value.trim().to_string())
            }
        }
    }
}

impl From<&String> for MongoObjectId {
    fn from(value: &String) -> Self {
        Self::from(value.to_string())
    }
}

impl From<&str> for MongoObjectId {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<Option<ObjectId>> for MongoObjectId {
    fn from(value: Option<ObjectId>) -> Self {
        match value {
            Some(value) => Self::ObjectId(value),
            None => Self::None
        }
    }
}

impl From<ObjectId> for MongoObjectId {
    fn from(value: ObjectId) -> Self {
        Self::ObjectId(value)
    }
}

impl GetObjectId for MongoObjectId {
    fn get_object_id(&self) -> Option<ObjectId> {
        match self.set_to_object_id() {
            Self::ObjectId(value) => Some(value),
            _ => None
        }
    }
}

impl GetString for MongoObjectId {
    fn get_string(&self) -> Option<String> {
        match self.set_to_string() {
            Self::String(value) => Some(value),
            _ => None
        }
    }
}

impl IsEmpty for MongoObjectId {
    fn is_empty(&self) -> bool {
        match self {
            Self::ObjectId(_) => false,
            Self::String(value) => value.trim().is_empty(),
            Self::None => true
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
        match self.to_string().is_empty() {
            true => Self::None,
            false => Self::String(self.to_string())
        }
    }
}

impl ToBson for MongoObjectId {
    fn to_bson(&self) -> Option<Self> {
        match self.set_to_object_id() {
            Self::ObjectId(value) => Some(Self::ObjectId(value)),
            _ => None
        }
    }
}

impl ToJson for MongoObjectId {
    fn to_json(&self) -> Option<Self> {
        match self.set_to_string() {
            Self::String(value) => Some(Self::String(value)),
            _ => None
        }
    }
}

impl ToString for MongoObjectId {
    fn to_string(&self) -> String {
        match self {
            Self::ObjectId(value) => value.to_string(),
            Self::String(value) => value.to_string(),
            Self::None => String::default()
        }
    }
}

impl MongoObjectId {
    pub fn new() -> Self {
        Self::ObjectId(ObjectId::new())
    }
}