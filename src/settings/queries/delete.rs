use actix_web::Result;
use mongodb::{bson::doc, Database};

use crate::traits::GetObjectId;
use crate::settings::TABLE_SETTINGS;
use crate::Payload;
use crate::Settings;


impl Settings {
    pub async fn delete<T>(&self, database: &Database, field: T) -> Result<Self>
        where T: ToString
    {
        let collection =  database.collection::<Self>(TABLE_SETTINGS);

        let filter = match field.to_string().trim().to_lowercase().as_str() {
            "_id" => match self.id {
                Some(ref id) => match id.get_object_id() {
                    Some(id) => doc! { "_id": id },
                    None => return Err(Payload::error("Invalid object id"))
                },
                None => return Err(Payload::error("Invalid object id"))
            },
            "module" => match self.module {
                Some(ref module) => doc! { "module": module },
                None => return Err(Payload::error("Invalid module name"))
            },
            _ => return Err(Payload::error("Invalid field name"))
        };

        match collection.delete_one(filter, None).await {
            Ok(_) => Ok(self.clone()),
            Err(error) => Err(Payload::error(error))
        }
    }
}