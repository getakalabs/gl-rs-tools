use serde::{Serialize, Deserialize};

use crate::traits::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Switch<T:Clone + PartialEq> {
    Value(Box<T>),
    String(String),
    None
}

impl<T:Clone + PartialEq> Default for Switch<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<T:Clone + PartialEq> IsEmpty for Switch<T> {
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

impl<T: Clone + PartialEq> GetSwitchValue<T> for Switch<T> {
    fn get_switch_value(&self) -> Option<T> {
        match self.clone().is_empty() {
            true => None,
            false => match self.clone() {
                Self::Value(value) => Some(*value),
                _ => None
            }
        }
    }
}

impl<T:Clone + PartialEq> Switch<T> {
    pub fn new(value: T) -> Self {
        Switch::Value(Box::new(value))
    }
}

