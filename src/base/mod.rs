pub mod mutations;
pub mod getters;

use arraygen::Arraygen;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

use crate::Cipher;
use crate::DBClient;
use crate::Module;
use crate::Settings;

use crate::traits::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_ciphers: &mut Option<Cipher>)]
pub struct Base {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub api_url: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub web_url: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub admin_url: Option<Cipher>,
}

impl IsEmpty for Base {
    fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }
}

impl Decrypt for Base {
    fn decrypt(&self) -> Self {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = cipher.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => d.decrypt_master()
            });
        }

        data
    }
}

impl Encrypt for Base {
    fn encrypt(&self) -> Self {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = cipher.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => d.encrypt_master()
            });
        }

        data
    }
}

impl ToBson for Base {
    fn to_bson(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = match cipher {
                None => None,
                Some(data) => {
                    match data.is_empty() {
                        true => None,
                        false => data.encrypt_master()
                    }
                }
            };
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl ToJson for Base {
    fn to_json(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = match cipher {
                None => None,
                Some(data) => {
                    match data.is_empty() {
                        true => None,
                        false => {
                            let data = data.set_to_string();
                            match data.is_empty() {
                                true => None,
                                false => Some(data)
                            }
                        }
                    }
                }
            };
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl Base {
    pub fn new(form: &Settings) -> Self {
        Self{
            api_url: Cipher::new(form.get_api_url()),
            web_url: Cipher::new(form.get_web_url()),
            admin_url: Cipher::new(form.get_admin_url())
        }
    }

    pub async fn stage(client: &DBClient) -> Arc<RwLock<Base>> {
        let db = match client.get_db() {
            None => return Arc::new(RwLock::new(Base::default())),
            Some(client) => client
        };

        let settings = match Settings::read_from_module(&db, &Module::Base).await {
            Ok(settings) => settings,
            Err(_) => return Arc::new(RwLock::new(Base::default()))
        };

        let data = settings
            .base
            .map_or(Base::default(), |d| d.decrypt());

        Arc::new(RwLock::new(data))
    }
}

