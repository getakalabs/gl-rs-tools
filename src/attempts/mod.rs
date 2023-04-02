use arraygen::Arraygen;
use mongodb::bson::{Bson, Document};
use serde::{Serialize, Deserialize};

use crate::traits::prelude::*;
use crate::MongoDateTime;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_mongo_date_time: &mut Option<MongoDateTime>)]
pub struct Attempt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_mongo_date_time)]
    pub attempt_time: Option<MongoDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_mongo_date_time)]
    pub success_time: Option<MongoDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl IsEmpty for Attempt {
    fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }
}

impl ToBson for Attempt {
    fn to_bson(&self) -> Option<Self>{
        let mut data = self.clone();

        for mongo_date_time in data.get_mongo_date_time() {
            *mongo_date_time = mongo_date_time.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => Some(d.set_to_bson_date_time())
            });
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

        for mongo_date_time in data.get_mongo_date_time() {
            *mongo_date_time = mongo_date_time.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => Some(d.set_to_string_date_time())
            });
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl From<Attempt> for Bson {
    fn from(value: Attempt) -> Self {
        Bson::Document(value.into())
    }
}

impl From<Attempt> for Document {
    fn from(value: Attempt) -> Document {
        let value = value.to_bson();

        match value {
            Some(value) => {
               match value.is_empty() {
                   true => Document::default(),
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
            },
            None => Document::default()
        }
    }
}

impl Attempt {
    pub fn new() -> Self {
        Self{
            id: Some(crate::generators::numbers::get_rand(6)),
            ..Default::default()
        }
    }
}
