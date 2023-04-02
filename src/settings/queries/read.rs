use mongodb::{bson::doc, options::FindOneOptions, Database};
use mongodb::bson::Bson;

use crate::traits::prelude::*;
use crate::Errors;
use crate::Module;
use crate::Settings;

impl Settings {
    pub async fn read_from_module(db: &Database, module: &Module) -> Result<Self, Errors> {
        let collection =  db.collection::<Self>("settings");

        let filter = doc! { "module": Bson::from(module) };
        let options = FindOneOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();

        match collection.find_one(filter, options).await {
            Ok(value) => {
                match value {
                    Some(value) => {
                        match value.decrypt().to_json() {
                            None => Err(Errors::new("Unable to convert the record to JSON. Please check your input and try again")),
                            Some(data) => Ok(data)
                        }
                    },
                    None => Err(Errors::new("No matching record was found in the database. Please check your input and try again"))
                }
            }
            Err(error) => Err(Errors::new(error))
        }
    }
}