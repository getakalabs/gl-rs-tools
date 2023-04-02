use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};

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

// Token implementation
impl Token {
    // Create new Token instance
    pub fn new() -> Self {
        Self::default()
    }

    // Convert custom struct type to Token
    #[allow(dead_code)]
    pub fn from<T>(input: T) -> Self
        where T: Serialize
    {
        serde_json::from_str(
            &serde_json::to_string(&input)
                .unwrap_or(String::default())
        ).unwrap_or(Token::default())
    }

    // Convert custom struct type to Token
    #[allow(dead_code)]
    pub fn from_string<T: ToString>(input: T) -> Self {
        serde_json::from_str(&input.to_string()).unwrap_or(Token::default())
    }

    // Convert custom struct type from Token to T
    #[allow(dead_code)]
    pub fn to<T>(&self) -> T
        where T: serde::de::DeserializeOwned + Default
    {
        serde_json::from_str::<T>(
            &serde_json::to_string(&self.clone())
                .unwrap_or(String::default())
        ).unwrap_or(T::default())
    }

    // Check if tokens has no value
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }

    // Normalize form
    #[allow(dead_code)]
    pub fn normalize(&self) -> Self {
        // Set data
        let mut data = self.clone();

        // Sanitize form
        data.sanitize();

        // Return data
        data
    }
}
