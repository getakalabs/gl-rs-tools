use serde::{Serialize, Deserialize};

use crate::traits::prelude::*;
use crate::Swap;
use crate::traits::SetToMongoObjectId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Array<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> {
    SwapArray(Vec<Option<Swap<T>>>),
    String(String),
    None
}

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> Default for Array<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> GetArrayString for Array<T> {
    fn get_array_string(&self) -> Option<Vec<String>> {
        match self.clone() {
            Self::SwapArray(value) => {
                let mut array:Vec<String> = Vec::new();

                for item in value.into_iter().flatten() {
                    if !item.to_string().is_empty() {
                        array.push(item.to_string());
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

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> GetArrayValue<T> for Array<T> {
    fn get_array_value(&self) -> Option<Vec<T>> where T: Sized {
        match self.clone() {
            Self::SwapArray(value) => {
                let mut array:Vec<T> = Vec::new();

                for item in value.into_iter().flatten() {
                    if let Some(value) = item.get_swap_value() {
                        array.push(value);
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

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> IsEmpty for Array<T> {
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

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> From<Vec<String>> for Array<T> {
    fn from(value: Vec<String>) -> Self {
        match value.is_empty() {
            true => Self::None,
            false => {
                let mut array:Vec<Option<Swap<T>>> = Vec::new();

                for item in value.into_iter() {
                    array.push(Some(Swap::String(item)));
                }

                Self::SwapArray(array)
            }
        }
    }
}

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> From<Vec<T>> for Array<T> {
    fn from(value: Vec<T>) -> Self {
        match value.is_empty() {
            true => Self::None,
            false => {
                let mut array:Vec<Option<Swap<T>>> = Vec::new();

                for item in value.into_iter() {
                    array.push(Some(Swap::new(item)));
                }

                Self::SwapArray(array)
            }
        }
    }
}

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> SetToMongoObjectId for Array<T> {
    fn set_to_mongo_object_id(&self) -> Self {
        match self.clone() {
            Self::SwapArray(value) => {
                let mut array:Vec<Option<Swap<T>>> = Vec::new();

                for item in value.into_iter().flatten() {
                    if item.to_bson().is_some() {
                        array.push(item.to_bson());
                    }
                }

               Self::SwapArray(array)
            },
            _ => Self::None
        }
    }
}

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq> ToBson for Array<T> {
    fn to_bson(&self) -> Option<Self> {
        match self.set_to_mongo_object_id() {
            Self::SwapArray(value) => Some(Self::SwapArray(value)),
            _ => None
        }
    }
}

impl<T:Clone + GetMongoObjectId + ToJson + ToBson + PartialEq>  ToJson for Array<T> {
    fn to_json(&self) -> Option<Self<>> {
        match self.clone() {
            Self::SwapArray(value) => {
                let mut array = Vec::new();

                for item in value.into_iter().flatten() {
                    if let Some(value) = item.to_json() {
                        array.push(Some(value));
                    }
                }

                match array.is_empty() {
                    true => None,
                    false => Some(Self::SwapArray(array))
                }
            },
            _ => None
        }
    }
}
