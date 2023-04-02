use rand::Rng;

use crate::Paseto;

impl Paseto {
    pub fn get_app_name(&self) -> String {
        self
            .app_name
            .clone()
            .map_or(String::from("QSweep"), |d| {
                d.to_string().unwrap_or(String::default())
            })
    }

    pub fn get_access_token_key_unit(&self) -> i32 {
        self
            .access_token_key_unit
            .clone()
            .map_or(5, |d| {
                d.to_i32().unwrap_or(5)
            })
    }

    pub fn get_access_token_key_time(&self) -> String {
        self
            .access_token_key_time
            .clone()
            .map_or(String::from("Minutes"), |d| {
                d.to_string().unwrap_or(String::default())
            })
    }

    pub fn get_access_token_key_signing(&self) -> String {
        let default = base64_url::encode(&rand::thread_rng().gen::<[u8; 32]>().to_vec());

        self
            .access_token_key_signing
            .clone()
            .map_or(default.clone(), |d| {
                d.to_string().unwrap_or(default.clone())
            })
    }

    pub fn get_refresh_token_key_unit(&self) -> i32 {
        self
            .refresh_token_key_unit
            .clone()
            .map_or(30, |d| {
                d.to_i32().unwrap_or(30)
            })
    }

    pub fn get_refresh_token_key_time(&self) -> String {
        self
            .refresh_token_key_time
            .clone()
            .map_or(String::from("Minutes"), |d| {
                d.to_string().unwrap_or(String::default())
            })
    }

    pub fn get_refresh_token_key_signing(&self) -> String {
        let default = base64_url::encode(&rand::thread_rng().gen::<[u8; 32]>().to_vec());

        self
            .refresh_token_key_signing
            .clone()
            .map_or(default.clone(), |d| {
                d.to_string().unwrap_or(default.clone())
            })
    }
}