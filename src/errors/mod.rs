use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Errors {
    Message(String)
}

impl ToString for Errors {
    fn to_string(&self) -> String {
        match self {
            Errors::Message(data) => data.to_string()
        }
    }
}

impl Errors {
    pub fn new<T: ToString>(str: T) -> Self {
        Self::Message(str.to_string())
    }
}