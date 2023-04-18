use actix_web::Result;
use mongodb::Database;

use crate::settings::TABLE_SETTINGS;
use crate::settings::Settings;

use crate::traits::{Decrypt, Encrypt};
use crate::traits::ToBson;

use crate::Payload;

impl Settings {
    pub async fn create(&self, database: &Database) -> Result<Self> {
        if let Some(value) = self.to_bson().and_then(|value| value.set_insertable().encrypt()) {
            let collection = database.collection::<Settings>(TABLE_SETTINGS);

            return match collection.insert_one(value, None).await {
                Ok(_) => Ok(self.decrypt().unwrap_or(self.clone())),
                Err(errors) => Err(Payload::error(errors))
            };
        }

        Err(Payload::error("An error occurred while trying to save a new settings entry"))
    }
}