use mongodb::Database;

use crate::settings::TABLE_SETTINGS;

use crate::Errors;
use crate::Settings;

use crate::traits::Decrypt;
use crate::traits::ToBson;

impl Settings {
    pub async fn create(&self, db: &Database) -> Result<Self, Errors> {
        match self.clone().to_bson() {
            Some(value) => {
                let mut data = value.clone();
                if data.id.is_none() {
                    data = data.set_insertable();
                }

                let collection = db.collection::<Settings>(TABLE_SETTINGS);
                match collection.insert_one(data, None).await {
                    Ok(_) => Ok(self.decrypt()),
                    Err(errors) => Err(Errors::new(errors))
                }
            },
            None => Err(Errors::new("An error occurred while creating the settings."))
        }
    }
}