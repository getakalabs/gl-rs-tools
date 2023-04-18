use actix_web::Result;
use chrono::{DateTime, Duration, Utc};
use paseto_lib::tokens::{validate_local_token, PasetoBuilder, TimeBackend};
use serde::Serialize;

use crate::traits::GetI32;
use crate::traits::IsEmpty;
use crate::Paseto;
use crate::Payload;
use crate::Token;

impl Paseto {
    pub fn generate_tokens<I, C>(&self, id:I, claims: &C) -> Result<Token>
        where I: ToString,
              C: Serialize + Clone
    {
        let c = serde_json::to_value(&(*claims).clone()).unwrap();

        // Set access token duration
        let access_token_duration = match self.access_token_key_time.clone().map_or(String::default(), |d| d.to_string()).as_str() {
            "Minutes" => Duration::minutes(i64::from(self.access_token_key_unit.clone().map_or(5, |d| d.get_i32().unwrap_or(5)))),
            "Hours" => Duration::hours(i64::from(self.access_token_key_unit.clone().map_or(5, |d| d.get_i32().unwrap_or(5)))),
            "Days" => Duration::days(i64::from(self.access_token_key_unit.clone().map_or(5, |d| d.get_i32().unwrap_or(5)))),
            _ =>  Duration::seconds(i64::from(self.access_token_key_unit.clone().map_or(5, |d| d.get_i32().unwrap_or(5))))
        };

        // Set access token expiry
        let access_token_expiry = Utc::now().checked_add_signed(access_token_duration).unwrap();

        // Set aid
        let aid = id.to_string();

        // Decrypt access token signing
        let access_token_signing = base64_url::decode(&self.access_token_key_signing.clone().map_or(String::default(), |d| d.to_string()));
        if access_token_signing.is_err() {
            return Err(Payload::error("Unable to generate access token"));
        }

        // Set access token signing
        let access_token_signing = access_token_signing.unwrap();

        // Set access token
        let access_token = PasetoBuilder::new()
            .set_encryption_key(&access_token_signing[..])
            .set_expiration(&access_token_expiry)
            .set_subject(&aid)
            .set_footer(format!("key-id:{}", &self.app_name.clone().map_or(String::default(), |d| d.to_string())).as_str())
            .set_claim("data", c.clone())
            .build();

        if access_token.is_err() {
            return Err(Payload::error("Unable to generate access token"));
        }

        // Set refresh token duration
        let refresh_token_duration = match self.refresh_token_key_time.clone().map_or(String::default(), |d| d.to_string()).as_str() {
            "Minutes" => Duration::minutes(i64::from(self.refresh_token_key_unit.clone().map_or(30, |d| d.get_i32().unwrap_or(30)))),
            "Hours" => Duration::hours(i64::from(self.refresh_token_key_unit.clone().map_or(30, |d| d.get_i32().unwrap_or(30)))),
            "Days" => Duration::days(i64::from(self.refresh_token_key_unit.clone().map_or(30, |d| d.get_i32().unwrap_or(30)))),
            _ =>  Duration::seconds(i64::from(self.refresh_token_key_unit.clone().map_or(30, |d| d.get_i32().unwrap_or(30))))
        };

        // Set refresh token expiry
        let refresh_token_expiry = Utc::now().checked_add_signed(refresh_token_duration).unwrap();

        // Decrypt refresh token signing
        let refresh_token_signing = base64_url::decode(&self.refresh_token_key_signing.clone().map_or(String::default(), |d| d.to_string()));
        if refresh_token_signing.is_err() {
            return Err(Payload::error("Unable to generate access token"));
        }

        // Set access token signing
        let refresh_token_signing = refresh_token_signing.unwrap();

        // Set refresh token
        let refresh_token = PasetoBuilder::new()
            .set_encryption_key(&refresh_token_signing[..])
            .set_expiration(&refresh_token_expiry)
            .set_subject(&aid)
            .set_footer(format!("key-id:{}", &self.app_name.clone().map_or(String::default(), |d| d.to_string())).as_str())
            .set_claim("data", c.clone())
            .build();

        if refresh_token.is_err() {
            return Err(Payload::error("Unable to generate refresh token"));
        }

        // Create encrypted web token
        let encrypted = crate::ciphers::encrypt(c.to_string().trim(), "WEB_KEY");
        if encrypted.is_err() {
            return Err(Payload::error("Encryption failed"));
        }

        // Create mutable token
        let mut tokens = Token::new();
        tokens.access = Some(access_token.unwrap());
        tokens.refresh = Some(refresh_token.unwrap());
        tokens.web = Some(encrypted.unwrap());

        // Return tokens
        Ok(tokens)
    }

    pub fn validate_access_token<T, C>(&self, token: T, _: C) -> Result<C>
        where T: ToString,
              C: serde::de::DeserializeOwned + Default
    {

        // Retrieve access token key signing
        let access_token_key_signing = match self.access_token_key_signing {
            Some(ref value) => value.to_string(),
            None => return Err(Payload::error("Invalid authentication token"))
        };

        // Decrypt access token signing
        let access_token_signing = match base64_url::decode(&access_token_key_signing) {
            Ok(value) => value,
            Err(error) => return Err(Payload::error(error))
        };

        // Retrieve app name
        let app_name = match self.app_name {
            Some(ref value) => value.to_string(),
            None => return Err(Payload::error("Invalid authentication token"))
        };

        // Verify token
        let result = match validate_local_token(
            &token.to_string(),
            Some(&format!("key-id:{app_name}")),
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
                    true => Err(Payload::error("Your authentication token has expired")),
                    false => Err(Payload::error("Invalid authentication token"))
                }
            }
        };

        // Retrieve values from paseto
        let result = match result.get("data") {
            Some(value) => value.to_owned(),
            None => return Err(Payload::error("Invalid authentication token"))
        };

        // Return value to custom struct
        let claims = serde_json::from_value::<C>(result)?;

        // Return claims
        Ok(claims)
    }

    pub fn validate_refresh_token<T, C>(&self, token: T, _: C) -> Result<C>
        where T: ToString,
              C: serde::de::DeserializeOwned + Default
    {
        // Decrypt refresh token signing
        let refresh_token_signing = base64_url::decode(&self.refresh_token_key_signing.clone().map_or(String::default(), |d| d.to_string()));
        if refresh_token_signing.is_err() {
            return Err(Payload::error("Invalid refresh token"));
        }

        // Set access token signing
        let refresh_token_signing = refresh_token_signing.unwrap();

        // Verify token
        let result = match validate_local_token(
            &token.to_string(),
            Some(format!("key-id:{}", &self.app_name.clone().map_or(String::default(), |d| d.to_string())).as_str()),
            &refresh_token_signing[..],
            &TimeBackend::Chrono
        ) {
            Ok(result) => match result.get("data") {
                Some(_) => result,
                None => return Err(Payload::error("Invalid refresh token"))
            },
            Err(error) => {
                let is_expired = error
                    .to_string()
                    .to_lowercase()
                    .as_str() == "this token is expired (exp claim).";

                return match is_expired {
                    true => Err(Payload::error("Your refresh token has expired")),
                    false => Err(Payload::error("Invalid refresh token"))
                }
            }
        };


        // Retrieve values from paseto
        let result = result.get("data");
        if result.is_none() {
            return Err(Payload::error("Invalid refresh token"));
        }

        // Return value to custom struct
        let result:Result<C, _> = serde_json::from_value(result.unwrap().clone());
        if result.is_err() {
            return Err(Payload::error("Invalid refresh token"));
        }

        // Return claims
        Ok(result.unwrap())
    }

    pub fn validate_web_token<T, C>(&self, token: T, _: C) -> Result<C>
        where T: ToString,
              C: serde::de::DeserializeOwned + Default
    {
        // Create decrypt web token
        let result = crate::ciphers::encrypt(token.to_string(), "WEB_KEY");
        if result.is_err() {
            return Err(Payload::error("Decryption failed"));
        }

        // Return value to custom struct
        let result:Result<C, _> = serde_json::from_str(&result.unwrap());
        if result.is_err() {
            return Err(Payload::error("Invalid authentication token"));
        }

        // Return claims
        Ok(result.unwrap())
    }

    pub fn get_access_token_expiry(&self) -> DateTime<Utc> {
        // Create default expiry
        let expiry = Utc::now()
            .checked_add_signed(Duration::minutes(
                i64::from(self.refresh_token_key_unit.clone().map_or(30, |d| d.get_i32().unwrap_or(30)))
            ))
            .unwrap();

        self
            .access_token_key_time
            .clone()
            .unwrap_or_default()
            .is_empty()
            .then(|| String::from("Minutes"))
            .map_or(expiry, |item| {
                let duration = match item.as_ref() {
                    "Minutes" => Duration::minutes(i64::from(self.access_token_key_unit.clone().map_or(5, |d| d.get_i32().unwrap_or(5)))),
                    "Hours" => Duration::hours(i64::from(self.access_token_key_unit.clone().map_or(5, |d| d.get_i32().unwrap_or(5)))),
                    "Days" => Duration::days(i64::from(self.access_token_key_unit.clone().map_or(5, |d| d.get_i32().unwrap_or(5)))),
                    _ =>  Duration::seconds(i64::from(self.access_token_key_unit.clone().map_or(5, |d| d.get_i32().unwrap_or(5))))
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
                i64::from(self.refresh_token_key_unit.clone().map_or(30, |d| d.get_i32().unwrap_or(30)))
            ))
            .unwrap();

        self
            .refresh_token_key_time
            .clone()
            .unwrap_or_default()
            .is_empty()
            .then(|| String::from("Minutes"))
            .map_or(expiry, |item| {
                let duration = match item.as_ref() {
                    "Minutes" => Duration::minutes(i64::from(self.refresh_token_key_unit.clone().map_or(30, |d| d.get_i32().unwrap_or(30)))),
                    "Hours" => Duration::hours(i64::from(self.refresh_token_key_unit.clone().map_or(30, |d| d.get_i32().unwrap_or(30)))),
                    "Days" => Duration::days(i64::from(self.refresh_token_key_unit.clone().map_or(30, |d| d.get_i32().unwrap_or(30)))),
                    _ =>  Duration::seconds(i64::from(self.refresh_token_key_unit.clone().map_or(30, |d| d.get_i32().unwrap_or(30))))
                };

                Utc::now()
                    .checked_add_signed(duration)
                    .unwrap()
            })
    }
}