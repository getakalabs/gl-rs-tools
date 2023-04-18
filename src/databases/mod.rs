pub mod array;
pub mod datetime;
pub mod objectid;

pub use actix_web::Result;
pub use array::MongoArray;
pub use datetime::MongoDateTime;
pub use objectid::MongoObjectId;

use mongodb::{Client, options::ClientOptions, Database};
use std::env;

use crate::Payload;
use crate::traits::IsEmpty;

#[derive(Debug, Clone)]
pub enum MongoDBManager {
    MongoDB(MongoDB),
    None
}

impl IsEmpty for MongoDBManager {
    fn is_empty(&self) -> bool {
        match self {
            MongoDBManager::MongoDB(_) => false,
            MongoDBManager::None => true
        }
    }
}

#[derive(Debug, Clone)]
pub struct MongoDB {
    pub client: Client,
    pub database: Database
}

impl From<(Client, String)> for MongoDB {
    fn from((client, database): (Client, String)) -> Self {
        let database = client.database(&database);

        Self { client, database }
    }
}

impl MongoDBManager {
    pub async fn new<T, U>(url: T, name: U) -> Result<Self>
        where T: ToString,
              U: ToString
    {
        let url = match env::var(url.to_string()) {
            Ok(value) => value,
            Err(_) => return Err(Payload::error("Unable to retrieve database url"))
        };

        let name = match env::var(name.to_string()) {
            Ok(value) => value,
            Err(_) => return Err(Payload::error("Unable to retrieve database name"))
        };

        let options = match ClientOptions::parse(&url).await {
            Ok(value) => value,
            Err(error) => return Err(Payload::error(error))
        };

        let client = match Client::with_options(options) {
            Ok(value) => value,
            Err(error) => return Err(Payload::error(error))
        };

        Ok(Self::MongoDB(MongoDB::from((client, name))))
    }

    pub fn get(&self) -> Result<Database> {
        match self {
            Self::MongoDB(value) => Ok(value.database.clone()),
            Self::None => Err(Payload::error("Unable to retrieve database"))
        }
    }

    pub fn get_client(&self) -> Result<Client> {
        match self {
            Self::MongoDB(value) => Ok(value.client.clone()),
            Self::None => Err(Payload::error("Unable to retrieve database client"))
        }
    }
}