use std::str::FromStr;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::traits::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Swap<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> {
    ObjectId(ObjectId),
    Swap(Box<T>),
    String(String),
    None
}

impl <T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> Default for Swap<T> {
    fn default() -> Self {
        Self::None
    }
}

impl <T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> GetSwap<T> for Swap<T> {
    fn get_swap(&self) -> Option<T> {
        match self {
            Self::Swap(value) => Some(*value.clone()),
            _ => None
        }
    }
}

impl <T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> GetObjectId for Swap<T> {
    fn get_object_id(&self) -> Option<ObjectId> {
        match self {
            Self::ObjectId(value) => Some(*value),
            Self::Swap(value) => value.get_object_id(),
            Self::String(value) => match ObjectId::from_str(value) {
                Ok(value) => Some(value),
                Err(_) => None
            },
            _ => None
        }
    }
}

impl <T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> IsEmpty for Swap<T> {
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

impl <T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> ToJson for Swap<T> {
    fn to_json(&self) -> Option<Self> {
        match self {
            Self::ObjectId(value) => Some(Self::String(value.to_string())),
            Self::Swap(value) => match value.to_json() {
                Some(value) => match value.is_empty() {
                    true => None,
                    false => Some(Self::Swap(Box::new(value.to_json().unwrap_or_default())))
                },
                None => None
            },
            Self::String(value) => Some(Self::String(value.clone())),
            Self::None => Some(Self::None)
        }
    }
}

impl <T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> ToBson for Swap<T> {
    fn to_bson(&self) -> Option<Self> {
        match self {
            Self::ObjectId(value) => Some(Self::ObjectId(*value)),
            Self::Swap(value) => match value.get_object_id() {
                Some(value) => Some(Self::ObjectId(value)),
                None => Some(Self::Swap(value.clone()))
            },
            Self::String(value) => match ObjectId::from_str(value) {
                Ok(value) => Some(Self::ObjectId(value)),
                Err(_) => Some(Self::String(value.clone()))
            },
            Self::None => Some(Self::None)
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> ToString for Swap<T> {
    fn to_string(&self) -> String {
        match self {
            Self::ObjectId(value) => value.to_string(),
            Self::Swap(value) => value.get_object_id().unwrap_or_default().to_string(),
            Self::String(value) => value.clone(),
            Self::None => String::new()
        }
    }
}

impl<T:Clone + GetObjectId + ToJson + ToBson + IsEmpty + PartialEq + Default> Swap<T> {
    pub fn new(value: T) -> Self {
        Self::Swap(Box::new(value))
    }
}
