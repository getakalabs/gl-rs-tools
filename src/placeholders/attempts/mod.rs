use arraygen::Arraygen;
use mongodb::bson::{Bson, Document};
use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::default::Default;

use crate::traits::IsEmpty;
use crate::traits::ToBson;
use crate::traits::ToJson;
use crate::traits::ToOption;
use crate::MongoDateTime;

#[derive(Debug, Default, Clone, PartialEq, Sanitize, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_array_date: &mut Option<MongoDateTime>)]
pub struct Attempt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_date)]
    pub attempt_time: Option<MongoDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_date)]
    pub success_time: Option<MongoDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl From<Attempt> for Bson {
    fn from(value: Attempt) -> Self {
        Bson::Document(value.into())
    }
}

impl From<Attempt> for Document {
    fn from(value: Attempt) -> Document {
        match value.is_empty() {
            true => Document::new(),
            false => {
                let mut doc = Document::new();
                doc.insert("attempt_count", value.attempt_count);
                doc.insert("attempt_time", value.attempt_time);
                doc.insert("success_time", value.success_time);
                doc.insert("id", value.id);
                doc.insert("ip", value.ip);
                doc
            }
        }
    }
}

impl IsEmpty for Attempt {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

impl ToBson for Attempt {
    fn to_bson(&self) -> Option<Self> {
        let mut data = self.clone();

        for date in data.get_array_date() {
            *date = date.clone().and_then(|d| d.to_bson());
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl ToJson for Attempt {
    fn to_json(&self) -> Option<Self> {
        let mut data = self.clone();

        for date in data.get_array_date() {
            *date = date.clone().and_then(|d| d.to_json());
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl ToOption for Attempt {
    fn to_option(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => Some(self.clone())
        }
    }
}
