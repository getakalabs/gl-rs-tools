use mongodb::{bson::doc, Database};

use crate::Errors;
use crate::Settings;

impl Settings {
    pub async fn delete_from_id(&self, db: &Database) -> Result<Self, Errors> {
        if self.get_id().is_none() {
            return Err(Errors::new("Invalid object id"));
        }

        let collection =  db.collection::<Self>("settings");
        let filter = doc! { "_id": self.get_id().unwrap() };
        match collection.delete_one(filter, None).await {
            Ok(value) => {
                match value.deleted_count == 1 {
                    true => Ok(self.clone()),
                    false => Err(Errors::new("Unable to delete your entry. Please try again"))
                }
            }
            Err(error) => Err(Errors::new(error))
        }
    }

    pub async fn delete_from_module(&self, db: &Database) -> Result<Self, Errors> {
        if self.module.clone().is_none() {
            return Err(Errors::new("Invalid module name"));
        }

        let collection =  db.collection::<Self>("settings");
        let filter = doc! { "module": self.module.clone().unwrap() };
        match collection.delete_one(filter, None).await {
            Ok(value) => {
                match value.deleted_count == 1 {
                    true => Ok(self.clone()),
                    false => Err(Errors::new("Unable to delete your entry. Please try again"))
                }
            }
            Err(error) => Err(Errors::new(error))
        }
    }
}