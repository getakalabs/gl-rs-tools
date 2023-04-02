use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use mongodb::bson::Bson;

use crate::traits::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MongoArray {
    ObjectId(Vec<Option<ObjectId>>),
    String(Vec<Option<String>>),
    None
}

impl Default for MongoArray {
    fn default() -> Self {
        Self::None
    }
}

impl From<String> for MongoArray {
    fn from(value: String) -> Self {
        Self::src(value)
    }
}

impl From<Option<String>> for MongoArray {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(value) => Self::src(value),
            None => Self::None
        }
    }
}

impl From<&Option<String>> for MongoArray {
    fn from(value: &Option<String>) -> Self {
        match value.clone() {
            Some(value) => Self::src(value),
            None => Self::None
        }
    }
}

impl From<&String> for MongoArray {
    fn from(value: &String) -> Self {
        Self::src(value.to_string())
    }
}

impl From<&str> for MongoArray {
    fn from(value: &str) -> Self {
        Self::src(value.to_string())
    }
}

impl From<Vec<String>> for MongoArray {
    fn from(value: Vec<String>) -> Self {
        let mut array = Vec::new();
        for item in value {
            if !item.is_empty() {
                array.push(Some(item));
            }
        }

        Self::String(array).set_to_object_id()
    }
}

impl From<Option<Vec<String>>> for MongoArray {
    fn from(value: Option<Vec<String>>) -> Self {
        let mut array = Vec::new();
        for item in value.into_iter().flatten() {
            if !item.is_empty() {
                array.push(Some(item));
            }
        }

        Self::String(array).set_to_object_id()
    }
}

impl From<&Option<Vec<String>>> for MongoArray {
    fn from(value: &Option<Vec<String>>) -> Self {
        let mut array = Vec::new();
        for item in value.clone().into_iter().flatten() {
            if !item.is_empty() {
                array.push(Some(item));
            }
        }

        Self::String(array).set_to_object_id()
    }
}

impl From<Vec<Option<String>>> for MongoArray {
    fn from(value: Vec<Option<String>>) -> Self {
        let mut array = Vec::new();
        for item in value.into_iter().flatten() {
            if !item.is_empty() {
                array.push(Some(item));
            }
        }

        Self::String(array).set_to_object_id()
    }
}

impl From<&Vec<Option<String>>> for MongoArray {
    fn from(value: &Vec<Option<String>>) -> Self {
        let mut array = Vec::new();
        for item in value.clone().into_iter().flatten() {
            if !item.is_empty() {
                array.push(Some(item));
            }
        }

        Self::String(array).set_to_object_id()
    }
}

impl From<Option<Vec<Option<String>>>> for MongoArray {
    fn from(value: Option<Vec<Option<String>>>) -> Self {
        let mut array:Vec<Option<String>> = Vec::new();

        if let Some(value) = value {
            for item in value.into_iter().flatten() {
                if !item.is_empty() {
                    array.push(Some(item));
                }
            }
        }


        Self::String(array).set_to_object_id()
    }
}

impl From<&Option<Vec<Option<String>>>> for MongoArray {
    fn from(value: &Option<Vec<Option<String>>>) -> Self {
        let mut array = Vec::new();

        if let Some(value) = value.clone() {
            for item in value.clone().into_iter().flatten() {
                if !value.is_empty() {
                    array.push(Some(item));
                }
            }
        }


        Self::String(array).set_to_object_id()
    }
}

impl From<ObjectId> for MongoArray {
    fn from(value: ObjectId) -> Self {
        Self::src(value)
    }
}

impl From<MongoArray> for Bson {
    fn from(value: MongoArray) -> Self {
        let value = value.set_to_object_id();
        match value.is_empty() {
            true => Bson::Null,
            false => match value.get_object_ids() {
                Some(value) => {
                    let mut array = Vec::new();
                    for item in value {
                        array.push(Bson::ObjectId(item));
                    }

                    Bson::Array(array)
                },
                None => Bson::Null
            }
        }
    }
}

impl<T: ToString> Src<T> for MongoArray {
    fn src(value: T) -> Self {
        match ObjectId::from_str(&value.to_string()) {
            Ok(value) => Self::ObjectId(vec![Some(value)]),
            Err(_) => Self::String(vec![Some(value.to_string())])
        }
    }
}

impl GetArrayString for MongoArray {
    fn get_array_string(&self) -> Option<Vec<String>> {
        match self.clone().set_to_string() {
            Self::String(value) => {
                let mut array:Vec<String> = Vec::new();

                for item in value.into_iter().flatten() {
                    array.push(item)
                }

                match array.is_empty() {
                    true => None,
                    false => Some(array)
                }
            },
            _ => None
        }
    }
}

impl GetObjectIds for MongoArray {
    fn get_object_ids(&self) -> Option<Vec<ObjectId>> {
        match self.clone().set_to_object_id() {
            Self::ObjectId(value) => {
                let mut array = Vec::new();

                for item in value.into_iter().flatten() {
                    array.push(item)
                }

                match array.is_empty() {
                    true => None,
                    false => Some(array)
                }
            },
            _ => None
        }
    }
}

impl GetStringIds for MongoArray {
    fn get_string_ids(&self) -> Option<Vec<String>> {
        match self.clone().set_to_string() {
            Self::String(value) => {
                let mut array:Vec<String> = Vec::new();

                for item in value.into_iter().flatten() {
                    if !item.is_empty() {
                        array.push(item.clone());
                    }
                }

                match array.is_empty() {
                    true => None,
                    false => Some(array)
                }
            },
            _ => None
        }
    }
}

impl GetMongoArray for MongoArray {
    fn get_mongo_array(&self) -> Option<MongoArray> {
        Self::get_self(self.clone())
    }
}

impl<T: IsEmpty> GetSelf<T> for MongoArray{}

impl IsEmpty for MongoArray {
    fn is_empty(&self) -> bool {
        match self.clone() == Self::default() {
            true => true,
            false => match self.clone() {
                Self::String(value) => match value.len() > 0 {
                    true => true,
                    false => false
                }
                Self::None => true,
                _ => false
            }
        }
    }
}

impl ToBson for MongoArray {
    fn to_bson(&self) -> Option<Self> {
        match self.clone().set_to_object_id() {
            Self::ObjectId(_) => Some(self.clone()),
            _ => None
        }
    }
}

impl SetToObjectId for MongoArray {
    fn set_to_object_id(&self) -> Self {
        match self {
            Self::ObjectId(value) => Self::ObjectId(value.clone()),
            Self::String(value) => {
                let mut array = Vec::new();
                for item in value {
                    match item {
                        Some(value) => if let Ok(value) = ObjectId::from_str(value) {
                            array.push(Some(value))
                        },
                        None => array.push(None)
                    }
                }

                match array.is_empty() {
                    true => Self::None,
                    false => Self::ObjectId(array)
                }
            },
            Self::None => Self::None
        }
    }
}

impl SetToString for MongoArray {
    fn set_to_string(&self) -> Self {
        match self {
            Self::ObjectId(value) => {
                let mut array = Vec::new();
                for item in value {
                    match item {
                        Some(value) => array.push(Some(value.to_string())),
                        None => {}
                    }
                }

                match array.is_empty() {
                    true => Self::None,
                    false => Self::String(array)
                }
            },
            Self::String(value) => {
                let mut array = Vec::new();
                for item in value {
                    match item {
                        Some(value) => match value.is_empty() {
                            true => {},
                            false => array.push(Some(value.clone()))
                        },
                        None => {}
                    }
                }

                match array.is_empty() {
                    true => Self::None,
                    false => Self::String(array)
                }
            },
            Self::None => Self::None
        }
    }
}

impl ToJson for MongoArray {
    fn to_json(&self) -> Option<Self> {
        match self.clone().set_to_string() {
            Self::String(_) => Some(self.clone()),
            _ => None
        }
    }
}


