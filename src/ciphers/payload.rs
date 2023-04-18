use serde::{Serialize, Deserialize};

use crate::traits::IsEmpty;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uppercase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowercase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_character: Option<String>,
}

impl IsEmpty for Payload {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}