use serde::{Serialize, Deserialize};

use crate::traits::prelude::*;
use crate::Swap;

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