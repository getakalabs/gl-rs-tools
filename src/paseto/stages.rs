use actix_web::Result;
use mongodb::{bson::doc, options::FindOneOptions, Database};
use std::sync::{Arc, RwLock};

use crate::traits::Decrypt;
use crate::settings::TABLE_SETTINGS;
use crate::settings::Settings;

use crate::Paseto;
use crate::Payload;

impl Paseto {
    pub async fn setup<T>(database: &Database, app_name: T) -> Result<Arc<RwLock<Self>>>
        where T: ToString
    {
        let paseto = Paseto::from(app_name.to_string());
        let settings = Settings::from(paseto);

        if let Ok(value) = settings.create(database).await {
            return Ok(Arc::new(RwLock::new(value.paseto.unwrap())));
        }

        Ok(Arc::new(RwLock::new(Self::default())))
    }

    pub async fn stage<T>(database: &Database, app_name: T) -> Result<Arc<RwLock<Self>>>
        where T: ToString
    {
        let collection = database.collection::<Settings>(TABLE_SETTINGS);
        let filter = doc! { "module": "Paseto" };
        let options = FindOneOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();

        match collection.find_one(filter, options).await {
            Ok(value) => if let Some(value) = value.and_then(|value| value.paseto.and_then(|value| value.decrypt())) {
                return Ok(Arc::new(RwLock::new(value)));
            },
            Err(error) => return Err(Payload::error(error))
        };

        match Self::setup(database, app_name).await {
            Ok(value) => Ok(value),
            Err(_) => Ok(Arc::new(RwLock::new(Self::default())))
        }
    }
}