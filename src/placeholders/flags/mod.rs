use mongodb::bson::{Bson, Document};
use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::default::Default;

use crate::traits::{IsEmpty, ToOption};

#[derive(Debug, Default, Clone, PartialEq, Sanitize, Serialize, Deserialize)]
pub struct Flag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_update_password_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_update_profile_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_onboarding_completed: Option<bool>
}

impl From<Flag> for Bson {
    fn from(value: Flag) -> Self {
        Bson::Document(value.into())
    }
}

impl From<Flag> for Document {
    fn from(value: Flag) -> Document {
        match value.is_empty() {
            true => Document::new(),
            false => {
                let mut doc = Document::new();
                doc.insert("is_update_password_required", value.is_update_password_required);
                doc.insert("is_update_profile_required", value.is_update_profile_required);
                doc.insert("is_verified", value.is_verified);
                doc.insert("is_onboarding_completed", value.is_onboarding_completed);
                doc
            }
        }
    }
}

impl IsEmpty for Flag {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

impl ToOption for Flag {
    fn to_option(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => Some(self.clone())
        }
    }
}