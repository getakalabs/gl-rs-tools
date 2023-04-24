use itertools::Itertools;
use mongodb::bson::{oid::ObjectId, Bson};
use serde::{Serialize, Deserialize};
use std::str::FromStr;

use crate::traits::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Primitive {
    I64(i64),
    F64(f64),
    Bool(bool),
    Array(Vec<Option<Primitive>>),
    String(String),
    None
}

impl Default for Primitive {
    fn default() -> Self {
        Self::None
    }
}

impl Dedup for Primitive {
    fn dedup(&self) -> Self {
        match self.clone() {
            Self::Array(array) => {
                let mut data = Vec::new();

                for item in array.into_iter().flatten() {
                    if item.to_string().trim().to_lowercase().as_str() != "none" {
                        data.push(item.to_string().trim().to_string());
                    }
                }

                let old_array:Vec<_> = data.into_iter().unique().collect();
                let mut new_array = Vec::new();

                for item in old_array {
                    new_array.push(Some(Primitive::String(item)));
                }

                Self::Array(new_array)
            },
            _ => self.clone()
        }
    }
}

impl GetArrayString for Primitive {
    fn get_array_string(&self) -> Option<Vec<String>> {
        match self.clone() {
            Self::Array(array) => {
                let mut data = Vec::new();

                for item in array.into_iter().flatten() {
                    if !item.to_string().is_empty() {
                        data.push(item.to_string());
                    }
                }

                if !data.is_empty() {
                    Some(data)
                } else {
                    None
                }
            },
            _ => None
        }
    }
}

impl GetBool for Primitive {
    fn get_bool(&self) -> Option<bool> {
        match self.clone() {
            Self::Bool(value) => Some(value),
            Self::String(value) => match value.to_lowercase().as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None
            },
            _ => None
        }
    }
}

impl GetI32 for Primitive {
    fn get_i32(&self) -> Option<i32> {
        match self.clone() {
            Self::I64(value) => Some(value as i32),
            Self::F64(value) => Some(value as i32),
            Self::String(value) => match value.parse::<i32>() {
                Ok(value) => Some(value),
                Err(_) => None
            },
            _ => None
        }
    }
}

impl GetF64 for Primitive {
    fn get_f64(&self) -> Option<f64> {
        match self.clone() {
            Self::I64(value) => Some(value as f64),
            Self::F64(value) => Some(value),
            Self::String(value) => match value.parse::<f64>() {
                Ok(value) => Some(value),
                Err(_) => None
            },
            _ => None
        }
    }
}

impl IsEmpty for Primitive {
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

impl From<Primitive> for Bson {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::I64(value) => Bson::Int64(value),
            Primitive::F64(value) => Bson::Double(value),
            Primitive::Bool(value) => Bson::Boolean(value),
            Primitive::String(value) => Bson::String(value),
            Primitive::Array(value) => Bson::Array(value.into_iter().map(|value| match value {
                Some(value) => Bson::from(value),
                None => Bson::Null
            }).collect()),
            Primitive::None => Bson::Null
        }
    }
}

impl From<Vec<String>> for Primitive {
    fn from(value: Vec<String>) -> Self {
        match value.is_empty() {
            true => Self::None,
            false => {
                let mut array = Vec::new();
                for item in value {
                    array.push(Some(Primitive::String(item)));
                }
                Self::Array(array)
            }
        }
    }
}

impl From<String> for Primitive {
    fn from(value: String) -> Self {
        match value.trim().is_empty() {
            true => Self::None,
            false => Self::String(value)
        }
    }
}

impl From<&str> for Primitive {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<i32> for Primitive {
    fn from(value: i32) -> Self {
        Self::I64(value as i64)
    }
}

impl From<f64> for Primitive {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl ToBson for Primitive {
    fn to_bson(&self) -> Option<Self> {
        match self.clone() {
            Self::I64(value) => Some(Self::I64(value)),
            Self::F64(value) => Some(Self::F64(value)),
            Self::Bool(value) => Some(Self::Bool(value)),
            Self::String(value) => match ObjectId::from_str(value.as_str()) {
                Ok(value) => Some(Self::String(value.to_hex())),
                Err(_) => Some(Self::String(value))
            },
            Self::Array(value) => {
                let mut data = Vec::new();

                for item in value.into_iter().flatten() {
                    data.push(item.to_bson());
                }

                Some(Self::Array(data))
            },
            _ => None
        }
    }
}

impl ToJson for Primitive {
    fn to_json(&self) -> Option<Self> {
        match self.clone() {
            Self::I64(value) => Some(Self::I64(value)),
            Self::F64(value) => Some(Self::F64(value)),
            Self::Bool(value) => Some(Self::Bool(value)),
            Self::String(value) => Some(Self::String(value)),
            Self::Array(value) => {
                let mut data = Vec::new();

                for item in value.into_iter().flatten() {
                    data.push(item.to_json());
                }

                Some(Self::Array(data))
            },
            _ => None
        }
    }
}

impl ToString for Primitive {
    fn to_string(&self) -> String {
        match self.clone() {
            Self::I64(value) => value.to_string(),
            Self::F64(value) => value.to_string(),
            Self::Bool(value) => value.to_string(),
            Self::String(value) => value,
            _ => "None".to_string()
        }
    }
}