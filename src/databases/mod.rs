use mongodb::{Client, options::ClientOptions, Database};
use std::env;

// DBClient enum which will allows the actix web server to run with or without database connection
#[derive(Debug, Clone)]
pub enum DBClient {
    MongoDB(Database),
    Null
}

// DBClient core implementations
impl DBClient {
    // Create new database instance
    pub async fn new<DBU, DBN>(database_url: DBU, database_name: DBN) -> Self
        where DBU: ToString,
              DBN: ToString
    {
        match Self::get_config(database_url, database_name) {
            None => {}
            Some((database_url, database_name)) => {
                match ClientOptions::parse(&database_url).await {
                    Ok(client_options) => {
                        match Client::with_options(client_options) {
                            Ok(client) => return Self::MongoDB(client.database(&database_name)),
                            Err(error) => println!("{error}")
                        }
                    }
                    Err(error) => println!("{error}")
                }
            }
        }

        Self::Null
    }

    // Retrieve mongodb environment variables
    pub fn get_config<DBU, DBN>(database_url: DBU, database_name: DBN) -> Option<(String, String)>
        where DBU: ToString,
              DBN: ToString
    {
        // Retrieve database url from environment
        let database_url = match env::var(database_url.to_string()) {
            Ok(value) => value,
            Err(_) => String::default()
        };

        // Retrieve database name from environment
        let database_name = match env::var(database_name.to_string()) {
            Ok(value) => value,
            Err(_) => String::default()
        };

        // Check database url & name
        match database_url.is_empty() || database_name.is_empty() {
            false => Some((String::from(&database_url), String::from(&database_name))),
            true => {
                if database_url.is_empty() {
                    println!("Unable to parse your database url from environment")
                }

                if database_name.is_empty() {
                    println!("Unable to parse your database name from environment")
                }

                None
            }
        }
    }

    // Retrieve client with initialized database
    pub fn get_db(&self) -> Option<Database> {
        match self.clone() {
            DBClient::MongoDB(database) => Some(database),
            DBClient::Null => None
        }
    }

    // Retrieve client with initialized database
    pub fn get_client(&self) -> DBClient {
        self.clone()
    }
}