use serde::{Serialize, Deserialize};
use std::str::FromStr;

use mongodb::bson::oid::ObjectId;

use crate::traits::prelude::*;
use crate::MongoObjectId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Swap<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> {
    MongoObjectId(MongoObjectId),
    Value(Box<T>),
    String(String),
    None
}

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> Default for Swap<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> IsEmpty for Swap<T> {
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

impl<T: Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> GetObjectId for Swap<T> {
    fn get_object_id(&self) -> Option<ObjectId> {
        match self.clone() {
            Swap::MongoObjectId(data) => data.get_object_id(),
            Swap::Value(data) => {
                match data.get_mongo_object_id() {
                    Some(data) => data.get_object_id(),
                    None => None
                }
            },
            Swap::String(data) => {
                match data.is_empty() {
                    true => None,
                    false => {
                        match ObjectId::from_str(&data) {
                            Ok(data) => Some(MongoObjectId::from(data).get_object_id().unwrap()),
                            Err(_) => None
                        }
                    }
                }
            },
            Swap::None => None
        }
    }
}

impl<T: Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> GetSwapValue<T> for Swap<T> {
    fn get_swap_value(&self) -> Option<T> {
        match self.clone().is_empty() {
            true => None,
            false => match self.clone() {
                Self::Value(value) => Some(*value),
                _ => None
            }
        }
    }
}

impl <T: Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> ToString for Swap<T> {
    fn to_string(&self) -> String {
        match self.clone() {
            Swap::MongoObjectId(data) => data.to_string(),
            Swap::Value(data) => {
                match data.get_mongo_object_id() {
                    Some(data) => data.to_string(),
                    None => String::new()
                }
            },
            Swap::String(data) => data,
            Swap::None => String::new()
        }
    }
}

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> ToBson for Swap<T> {
    fn to_bson(&self) -> Option<Self> {
        match self.clone() {
            Swap::MongoObjectId(data) => data.to_bson().map(Swap::MongoObjectId),
            Swap::Value(data) => {
                match data.to_bson() {
                    Some(data) => match data.get_mongo_object_id() {
                        Some(data) => Some(Swap::MongoObjectId(data)),
                        None => Some(Swap::Value(Box::new(data)))
                    },
                    None => None
                }
            },
            Swap::String(data) => {
                match data.is_empty() {
                    true => None,
                    false => {
                        match ObjectId::from_str(&data) {
                            Ok(data) => Some(Swap::MongoObjectId(MongoObjectId::from(data))),
                            Err(_) => Some(Swap::String(data))
                        }
                    }
                }
            },
            Swap::None => None
        }
    }
}

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> ToJson for Swap<T> {
    fn to_json(&self) -> Option<Self> {
        match self.clone() {
            Swap::MongoObjectId(data) => data.to_json().map(Swap::MongoObjectId),
            Swap::Value(data) => data.to_json().map(|data| Swap::Value(Box::new(data))),
            Swap::String(data) => {
                match data.is_empty() {
                    true => None,
                    false => Some(Swap::String(data))
                }
            },
            Swap::None => None
        }
    }
}

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> Swap<T> {
    pub fn new(value: T) -> Self {
        Swap::Value(Box::new(value))
    }
}

