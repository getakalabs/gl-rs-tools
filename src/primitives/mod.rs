use mongodb::bson::Bson;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::traits::prelude::*;
use crate::MongoArray;
use crate::traits::{SetToBool, SetToI64, SetToMongoArray};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Primitive {
    I32(i32),
    I64(i64),
    Bool(bool),
    MongoArray(MongoArray),
    String(String),
    None
}

impl Default for Primitive {
    fn default() -> Self {
        Self::None
    }
}

impl ToString for Primitive {
    fn to_string(&self) -> String {
        match self {
            Primitive::I32(value) => value.to_string(),
            Primitive::I64(value) => value.to_string(),
            Primitive::Bool(value) => match value {
                true => String::from("true"),
                false => String::from("false")
            },
            Primitive::MongoArray(value) => {
                match serde_json::to_string(value) {
                    Ok(value) => value,
                    Err(_) => format!("{value:?}")
                }
            }
            Primitive::String(value) => value.clone(),
            Primitive::None => String::from("None")
        }
    }
}

impl ToOptString for Primitive {
    fn to_opt_string(&self) -> Option<String> {
        match self {
            Primitive::I32(value) => Some(value.to_string()),
            Primitive::I64(value) => Some(value.to_string()),
            Primitive::Bool(value) => match value {
                true => Some(String::from("true")),
                false => Some(String::from("false"))
            },
            Primitive::MongoArray(value) => {
                match serde_json::to_string(value) {
                    Ok(value) => Some(value),
                    Err(_) => None
                }
            }
            Primitive::String(value) => Some(value.clone()),
            Primitive::None => None
        }
    }
}

impl<T: IsEmpty> GetSelf<T> for Primitive {}

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
            Primitive::I32(value) => Bson::Int32(value),
            Primitive::I64(value) => Bson::Int64(value),
            Primitive::Bool(value) => Bson::Boolean(value),
            Primitive::MongoArray(value) => Bson::from(value),
            Primitive::String(value) => Bson::String(value),
            Primitive::None => Bson::Null
        }
    }
}

impl From<String> for Primitive {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<i32> for Primitive {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl GetArrayString for Primitive {
    fn get_array_string(&self) -> Option<Vec<String>> {
        match self.set_to_mongo_array() {
            Primitive::MongoArray(value) => value.get_array_string(),
            _ => None
        }
    }
}

impl GetI32 for Primitive {
    fn get_i32(&self) -> Option<i32> {
        match self.set_to_i32() {
            Primitive::I32(value) => Some(value),
            _ => None
        }
    }
}

impl GetI64 for Primitive {
    fn get_i64(&self) -> Option<i64> {
        match self.set_to_i64() {
            Primitive::I64(value) => Some(value),
            _ => None
        }
    }
}

impl GetBool for Primitive {
    fn get_bool(&self) -> Option<bool> {
        match self.set_to_bool() {
            Primitive::Bool(value) => Some(value),
            _ => None
        }
    }
}

impl GetMongoArray for Primitive {
    fn get_mongo_array(&self) -> Option<MongoArray> {
        match self.set_to_mongo_array() {
            Primitive::MongoArray(value) => Some(value),
            _ => None
        }
    }
}

impl GetObjectIds for Primitive {
    fn get_object_ids(&self) -> Option<Vec<ObjectId>> {
        match self.set_to_mongo_array() {
            Primitive::MongoArray(value) => value.get_object_ids(),
            _ => None
        }
    }
}

impl SetToI32 for Primitive {
    fn set_to_i32(&self) -> Self {
        match self.clone() {
            Self::I32(value) => Self::I32(value),
            Self::I64(value) => Self::I32(value as i32),
            Self::String(value) => match value.parse::<i32>() {
                Ok(value) => Self::I32(value),
                Err(_) => Self::None
            },
            _ => Self::I32(0)
        }
    }
}

impl SetToI64 for Primitive {
    fn set_to_i64(&self) -> Self {
        match self.clone() {
            Self::I32(value) => Self::I64(value as i64),
            Self::I64(value) => Self::I64(value),
            Self::String(value) => match value.parse::<i64>() {
                Ok(value) => Self::I64(value),
                Err(_) => Self::None
            },
            _ => Self::None
        }
    }
}

impl SetToBool for Primitive {
    fn set_to_bool(&self) -> Self {
        match self.clone() {
            Self::Bool(value) => Self::Bool(value),
            Self::String(value) => match value.to_lowercase().as_str() {
                "true" => Self::Bool(true),
                "false" => Self::Bool(false),
                _ => Self::None
            },
            _ => Self::None
        }
    }
}

impl SetToMongoArray for Primitive {
    fn set_to_mongo_array(&self) -> Self {
        match self.clone() {
            Self::MongoArray(value) => Self::MongoArray(value),
            Self::String(value) => Self::MongoArray(MongoArray::from(value)),
            _ => Self::None
        }
    }
}

impl SetToString for Primitive {
    fn set_to_string(&self) -> Self {
        match self.clone() {
            Self::I32(value) => Self::String(value.to_string()),
            Self::I64(value) => Self::String(value.to_string()),
            Self::Bool(value) => match value {
                true => Self::String(String::from("true")),
                false => Self::String(String::from("false"))
            },
            Self::String(value) => Self::String(value),
            _ => Self::None
        }
    }
}