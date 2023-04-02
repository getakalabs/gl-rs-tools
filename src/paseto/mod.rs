pub mod getters;
pub mod mutations;

use arraygen::Arraygen;
use chrono::{DateTime, Duration, Utc};
use paseto_lib::tokens::{validate_local_token, PasetoBuilder, TimeBackend};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

pub use crate::traits::*;
use crate::Cipher;
use crate::DBClient;
use crate::Errors;
use crate::Module;
use crate::Primitive;
use crate::Settings;
use crate::Token;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_ciphers: &mut Option<Cipher>)]
pub struct Paseto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub app_name: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub access_token_key_unit: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub access_token_key_time: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub access_token_key_signing: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub refresh_token_key_unit: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub refresh_token_key_time: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub refresh_token_key_signing: Option<Cipher>,
}

impl IsEmpty for Paseto {
    fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }
}

impl Decrypt for Paseto {
    fn decrypt(&self) -> Self {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = cipher.clone().and_then(|d| {
                match d.is_empty() {
                    true => None,
                    false => d.decrypt_master()
                }
            });
        }

        data
    }
}

impl Encrypt for Paseto {
    fn encrypt(&self) -> Self {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = cipher.clone().and_then(|d| {
                match d.is_empty() {
                    true => None,
                    false => d.encrypt_master()
                }
            });
        }

        data
    }
}

impl ToBson for Paseto {
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

impl ToJson for Paseto {
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

impl Paseto {
    pub fn new<T>(app_name: T, form: &Settings) -> Self
        where T: ToString
    {
        let mut data = Self::default();

        let access_token_key_unit = match form.get_access_token_key_unit() {
            Primitive::None => 5,
            _ => form.get_access_token_key_unit().get_i32().map_or(5, |d| d)
        };

        let access_token_key_time = match form.get_access_token_key_time().is_empty() {
            true => String::from("Minutes"),
            _ => form.get_access_token_key_time()
        };

        let access_token_key_signing = match form.get_access_token_key_signing().is_empty() {
            true => base64_url::encode(&rand::thread_rng().gen::<[u8; 32]>()),
            _ => form.get_access_token_key_signing()
        };

        data.app_name = Some(Cipher::from(&app_name.to_string()));
        data.access_token_key_unit = Some(Cipher::from(&access_token_key_unit));
        data.access_token_key_time = Some(Cipher::from(&access_token_key_time));
        data.access_token_key_signing = Some(Cipher::from(&access_token_key_signing));

        let refresh_token_key_unit = match form.get_refresh_token_key_unit() {
            Primitive::None => 30,
            _ => form.get_refresh_token_key_unit().get_i32().map_or(30, |d| d)
        };

        let refresh_token_key_time = match form.get_refresh_token_key_time().is_empty() {
            true => String::from("Minutes"),
            _ => form.get_refresh_token_key_time()
        };

        let refresh_token_key_signing = match form.get_refresh_token_key_signing().is_empty() {
            true => base64_url::encode(&rand::thread_rng().gen::<[u8; 32]>()),
            _ => form.get_refresh_token_key_signing()
        };

        data.refresh_token_key_unit = Some(Cipher::from(&refresh_token_key_unit));
        data.refresh_token_key_time = Some(Cipher::from(&refresh_token_key_time));
        data.refresh_token_key_signing = Some(Cipher::from(&refresh_token_key_signing));

        data
    }


    pub async fn stage(client: &DBClient) -> Arc<RwLock<Paseto>>
    {
        let db = match client.get_db() {
            None => return Arc::new(RwLock::new(Paseto::default())),
            Some(client) => client
        };

        let settings = match Settings::read_from_module(&db, &Module::Paseto).await {
            Ok(settings) => settings,
            Err(_) => Settings::default()
        };

        Arc::new(RwLock::new(settings.paseto.unwrap_or(Paseto::default())))
    }

    pub async fn setup<T>(client: &DBClient, app_name: T) -> Arc<RwLock<Paseto>>
        where T: ToString
    {
        let db = match client.get_db() {
            None => return Arc::new(RwLock::new(Paseto::default())),
            Some(client) => client
        };

        let paseto = Paseto::new(app_name, &Settings::default());
        let settings = match Settings::from(&paseto).create(&db).await {
            Ok(settings) => settings,
            Err(_) => return Arc::new(RwLock::new(Paseto::default())),
        };

        let data = settings
            .paseto
            .map_or(Paseto::default(), |d| d);

        Arc::new(RwLock::new(data))
    }

    pub fn generate_tokens<I, C>(&self, id:I, claims: &C) -> Result<Token, Errors>
        where I: ToString,
              C: Serialize + Clone
    {
        let c = serde_json::to_value(&(*claims).clone()).unwrap();

        // Set access token duration
        let access_token_duration = match self.get_access_token_key_time().as_str() {
            "Minutes" => Duration::minutes(i64::from(self.get_access_token_key_unit())),
            "Hours" => Duration::hours(i64::from(self.get_access_token_key_unit())),
            "Days" => Duration::days(i64::from(self.get_access_token_key_unit())),
            _ =>  Duration::seconds(i64::from(self.get_access_token_key_unit()))
        };

        // Set access token expiry
        let access_token_expiry = Utc::now().checked_add_signed(access_token_duration).unwrap();

        // Set aid
        let aid = id.to_string();

        // Decrypt access token signing
        let access_token_signing = base64_url::decode(&self.get_access_token_key_signing());
        if access_token_signing.is_err() {
            return Err(Errors::new("Unable to generate access token"));
        }

        // Set access token signing
        let access_token_signing = access_token_signing.unwrap();

        // Set access token
        let access_token = PasetoBuilder::new()
            .set_encryption_key(&access_token_signing[..])
            .set_expiration(&access_token_expiry)
            .set_subject(&aid)
            .set_footer(format!("key-id:{}", &self.get_app_name()).as_str())
            .set_claim("data", c.clone())
            .build();

        if access_token.is_err() {
            return Err(Errors::new("Unable to generate access token"));
        }

        // Set refresh token duration
        let refresh_token_duration = match self.get_refresh_token_key_time().as_str() {
            "Minutes" => Duration::minutes(i64::from(self.get_refresh_token_key_unit())),
            "Hours" => Duration::hours(i64::from(self.get_refresh_token_key_unit())),
            "Days" => Duration::days(i64::from(self.get_refresh_token_key_unit())),
            _ =>  Duration::seconds(i64::from(self.get_refresh_token_key_unit()))
        };

        // Set refresh token expiry
        let refresh_token_expiry = Utc::now().checked_add_signed(refresh_token_duration).unwrap();

        // Decrypt refresh token signing
        let refresh_token_signing = base64_url::decode(&self.get_refresh_token_key_signing());
        if refresh_token_signing.is_err() {
            return Err(Errors::new("Unable to generate access token"));
        }

        // Set access token signing
        let refresh_token_signing = refresh_token_signing.unwrap();

        // Set refresh token
        let refresh_token = PasetoBuilder::new()
            .set_encryption_key(&refresh_token_signing[..])
            .set_expiration(&refresh_token_expiry)
            .set_subject(&aid)
            .set_footer(format!("key-id:{}", &self.get_app_name()).as_str())
            .set_claim("data", c.clone())
            .build();

        if refresh_token.is_err() {
            return Err(Errors::new("Unable to generate refresh token"));
        }

        // Create encrypted web token
        let encrypted = crate::ciphers::encrypt(c.to_string().trim(), "WEB_KEY");
        if encrypted.is_err() {
            return Err(Errors::new("Encryption failed"));
        }

        // Create mutable token
        let mut tokens = Token::new();
        tokens.access = Some(access_token.unwrap());
        tokens.refresh = Some(refresh_token.unwrap());
        tokens.web = Some(encrypted.unwrap());

        // Return tokens
        Ok(tokens)
    }

    pub fn validate_access_token<T, C>(&self, token: T, _: C) -> Result<C, Errors>
        where T: ToString,
              C: serde::de::DeserializeOwned + Default
    {
        // Decrypt access token signing
        let access_token_signing = base64_url::decode(&self.get_access_token_key_signing());
        if access_token_signing.is_err() {
            return Err(Errors::new("Invalid authentication token"));
        }

        // Set access token signing
        let access_token_signing = access_token_signing.unwrap();

        // Verify token
        let result = match validate_local_token(
            &token.to_string(),
            Some(format!("key-id:{}", &self.get_app_name()).as_str()),
            &access_token_signing[..],
            &TimeBackend::Chrono
        ) {
            Ok(value) => value,
            Err(error) => {
                let is_expired = error
                    .to_string()
                    .to_lowercase()
                    .as_str() == "this token is expired (exp claim).";

                return match is_expired {
                    true => Err(Errors::new("Your authentication token has expired")),
                    false => Err(Errors::new("Invalid authentication token"))
                }
            }
        };

        // Retrieve values from paseto
        let result = result.get("data");
        if result.is_none() {
            return Err(Errors::new("Invalid authentication token"));
        }

        // Return value to custom struct
        let result:Result<C, _> = serde_json::from_value(result.unwrap().clone());
        if result.is_err() {
            return Err(Errors::new("Invalid authentication token"));
        }

        // Return claims
        Ok(result.unwrap())
    }

    pub fn validate_refresh_token<T, C>(&self, token: T, _: C) -> Result<C, Errors>
        where T: ToString,
              C: serde::de::DeserializeOwned + Default
    {
        // Decrypt refresh token signing
        let refresh_token_signing = base64_url::decode(&self.get_refresh_token_key_signing());
        if refresh_token_signing.is_err() {
            return Err(Errors::new("Invalid refresh token"));
        }

        // Set access token signing
        let refresh_token_signing = refresh_token_signing.unwrap();

        // Verify token
        let result = match validate_local_token(
            &token.to_string(),
            Some(format!("key-id:{}", &self.get_app_name()).as_str()),
            &refresh_token_signing[..],
            &TimeBackend::Chrono
        ) {
            Ok(result) => match result.get("data") {
                Some(_) => result,
                None => return Err(Errors::new("Invalid refresh token"))
            },
            Err(error) => {
                let is_expired = error
                    .to_string()
                    .to_lowercase()
                    .as_str() == "this token is expired (exp claim).";

                return match is_expired {
                    true => Err(Errors::new("Your refresh token has expired")),
                    false => Err(Errors::new("Invalid refresh token"))
                }
            }
        };


        // Retrieve values from paseto
        let result = result.get("data");
        if result.is_none() {
            return Err(Errors::new("Invalid refresh token"));
        }

        // Return value to custom struct
        let result:Result<C, _> = serde_json::from_value(result.unwrap().clone());
        if result.is_err() {
            return Err(Errors::new("Invalid refresh token"));
        }

        // Return claims
        Ok(result.unwrap())
    }

    pub fn validate_web_token<T, C>(&self, token: T, _: C) -> Result<C, Errors>
        where T: ToString,
              C: serde::de::DeserializeOwned + Default
    {
        // Create decrypt web token
        let result = crate::ciphers::encrypt(token.to_string(), "WEB_KEY");
        if result.is_err() {
            return Err(Errors::new("Decryption failed"));
        }

        // Return value to custom struct
        let result:Result<C, _> = serde_json::from_str(&result.unwrap());
        if result.is_err() {
            return Err(Errors::new("Invalid authentication token"));
        }

        // Return claims
        Ok(result.unwrap())
    }

    pub fn get_access_token_expiry(&self) -> DateTime<Utc> {
        // Create default expiry
        let expiry = Utc::now()
            .checked_add_signed(Duration::minutes(
                i64::from(self.get_refresh_token_key_unit())
            ))
            .unwrap();

        self
            .get_access_token_key_time()
            .is_empty()
            .then(|| String::from("Minutes"))
            .map_or(expiry, |item| {
                let duration = match item.as_ref() {
                    "Minutes" => Duration::minutes(i64::from(self.get_access_token_key_unit())),
                    "Hours" => Duration::hours(i64::from(self.get_access_token_key_unit())),
                    "Days" => Duration::days(i64::from(self.get_access_token_key_unit())),
                    _ =>  Duration::seconds(i64::from(self.get_access_token_key_unit()))
                };

                Utc::now()
                    .checked_add_signed(duration)
                    .unwrap()
            })
    }

    pub fn get_refresh_token_expiry(&self) -> DateTime<Utc> {
        // Create default expiry
        let expiry = Utc::now()
            .checked_add_signed(Duration::minutes(
                i64::from(self.get_refresh_token_key_unit())
            ))
            .unwrap();

        self
            .get_refresh_token_key_time()
            .is_empty()
            .then(|| String::from("Minutes"))
            .map_or(expiry, |item| {
                let duration = match item.as_ref() {
                    "Minutes" => Duration::minutes(i64::from(self.get_refresh_token_key_unit())),
                    "Hours" => Duration::hours(i64::from(self.get_refresh_token_key_unit())),
                    "Days" => Duration::days(i64::from(self.get_refresh_token_key_unit())),
                    _ =>  Duration::seconds(i64::from(self.get_refresh_token_key_unit()))
                };

                Utc::now()
                    .checked_add_signed(duration)
                    .unwrap()
            })
    }
}

