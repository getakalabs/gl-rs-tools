use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};

use crate::traits::Normalize;

// Struct container for token
#[derive(Debug, Default, Clone, PartialEq, Sanitize, Serialize, Deserialize)]
pub struct Token {
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>
}

impl Normalize for Token {
    fn normalize(&self) -> Self {
        let mut data = self.to_owned();
        data.sanitize();
        data
    }
}

impl Token {
    pub fn new() -> Self {
        Self::default()
    }
}
