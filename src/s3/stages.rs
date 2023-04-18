use actix_web::Result;
use mongodb::{bson::doc, options::FindOneOptions, Database};
use std::sync::{Arc, RwLock};

use crate::traits::Decrypt;
use crate::settings::TABLE_SETTINGS;
use crate::settings::Settings;

use crate::Payload;
use crate::S3;

impl S3 {
    pub async fn stage(database: &Database) -> Result<Arc<RwLock<Self>>> {
        let collection = database.collection::<Settings>(TABLE_SETTINGS);
        let filter = doc! { "module": "S3" };
        let options = FindOneOptions::builder()
            .sort(doc! { "created_at": -1 })
            .build();

        match collection.find_one(filter, options).await {
            Ok(value) => if let Some(value) = value.and_then(|value| value.s3.and_then(|value| value.decrypt())) {
                return Ok(Arc::new(RwLock::new(value)));
            },
            Err(error) => return Err(Payload::error(error))
        }

        Ok(Arc::new(RwLock::new(Self::default())))
    }
}