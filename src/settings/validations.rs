use actix_web::Result;

use crate::Payload;
use crate::Primitive;
use crate::Settings;
use crate::traits::IsEmpty;

impl Settings {
    pub fn validate_base(&self) -> Result<Payload> {
        let api_url = crate::validate_string(&self.api_url, Some(5))
            .and_then(|error| match error {
                "empty" => Some("API URL cannot be empty".to_string()),
                "invalid" => Some("API URL is invalid".to_string()),
                _=> None
            });

        let web_url = crate::validate_string(&self.web_url, Some(5))
            .and_then(|error| match error {
                "empty" => Some("Web URL cannot be empty".to_string()),
                "invalid" => Some("Web URL is invalid".to_string()),
                _=> None
            });

        let admin_url = crate::validate_string(&self.web_url, Some(5))
            .and_then(|error| match error {
                "empty" => Some("Admin URL cannot be empty".to_string()),
                "invalid" => Some("Admin URL is invalid".to_string()),
                _=> None
            });

        let error = Self {
            api_url,
            web_url,
            admin_url,
            ..Default::default()
        };

        match error.is_empty() {
            true => Ok(Payload::default()),
            false => Err(Payload::errors(error))
        }
    }

    pub fn validate_paseto(&self) -> Result<Payload> {
        let options = vec![
            "seconds",
            "minutes",
            "hours",
            "days"
        ];

        let access_token_key_unit = crate::validate_primitive_i32(&self.access_token_key_unit, Some(1))
            .and_then(|error| match error {
                "empty" => Some(Primitive::from("Please enter an access token key unit")),
                "invalid" => Some(Primitive::from("Please enter a valid access token key unit")),
                _=> None
            });

        let access_token_key_time = crate::validate_string_options(&self.access_token_key_time, &options)
            .and_then(|error| match error {
                "empty" => Some("Please enter your access token key time".to_string()),
                "invalid" => Some("Please enter a valid access token key time".to_string()),
                _=> None
            });

        let access_token_key_signing = crate::validate_string_base64(&self.access_token_key_signing, Some(32))
            .and_then(|error| match error {
                "empty" => Some("Please enter your access token key signing".to_string()),
                "invalid" => Some("Please enter a valid access token key signing".to_string()),
                _=> None
            });

        let refresh_token_key_unit = crate::validate_primitive_i32(&self.refresh_token_key_unit, Some(1))
            .and_then(|error| match error {
                "empty" => Some(Primitive::from("Please enter an refresh token key unit")),
                "invalid" => Some(Primitive::from("Please enter a valid refresh token key unit")),
                _=> None
            });


        let refresh_token_key_time = crate::validate_string_options(&self.refresh_token_key_time, &options)
            .and_then(|error| match error {
                "empty" => Some("Please enter your refresh token key time".to_string()),
                "invalid" => Some("Please enter a valid refresh token key time".to_string()),
                _=> None
            });

        let refresh_token_key_signing = crate::validate_string_base64(&self.refresh_token_key_signing, Some(32))
            .and_then(|error| match error {
                "empty" => Some("Please enter your refresh token key signing".to_string()),
                "invalid" => Some("Please enter a valid refresh token key signing".to_string()),
                _=> None
            });

        let error = Self {
            access_token_key_unit,
            access_token_key_time,
            access_token_key_signing,
            refresh_token_key_unit,
            refresh_token_key_time,
            refresh_token_key_signing,
            ..Default::default()
        };

        match error.is_empty() {
            true => Ok(Payload::default()),
            false => Err(Payload::errors(error))
        }
    }

    pub fn validate_mailer(&self) -> Result<Payload> {
        let sender = crate::validate_string(&self.sender, Some(5))
            .and_then(|error| match error {
                "empty" => Some("Please enter the sender of your mailer".to_string()),
                "invalid" => Some("Please enter a valid sender".to_string()),
                _=> None
            });

        let username = crate::validate_string(&self.username, Some(5))
            .and_then(|error| match error {
                "empty" => Some("Please enter the mailer username".to_string()),
                "invalid" => Some("Please enter a valid username".to_string()),
                _=> None
            });

        let password = crate::validate_string(&self.password, Some(5))
            .and_then(|error| match error {
                "empty" => Some("Please enter the mailer password".to_string()),
                "invalid" => Some("Please enter a valid password".to_string()),
                _=> None
            });

        let smtp_host = crate::validate_string(&self.smtp_host, Some(5))
            .and_then(|error| match error {
                "empty" => Some("Please enter the mailer SMTP host".to_string()),
                "invalid" => Some("Please enter a valid SMTP host".to_string()),
                _=> None
            });

        let service = crate::validate_string(&self.service, Some(5))
            .and_then(|error| match error {
                "empty" => Some("Please enter the mailer service".to_string()),
                "invalid" => Some("Please enter a valid mailer services".to_string()),
                _=> None
            });

        let email = crate::validate_email(&self.email)
            .and_then(|error| match error {
                "empty" => Some("Please enter the mailer service".to_string()),
                "invalid" => Some("Please enter a valid mailer services".to_string()),
                _=> None
            });

        let error = Self {
            sender,
            username,
            password,
            smtp_host,
            service,
            email,
            ..Default::default()
        };

        match error.is_empty() {
            true => Ok(Payload::default()),
            false => Err(Payload::errors(error))
        }
    }

    pub fn validate_s3(&self) -> Result<Payload> {
        let access_key_id = crate::validate_string(&self.access_key_id, Some(5))
            .and_then(|error| match error {
                "empty" => Some("Please enter an access key id".to_string()),
                "invalid" => Some("Please enter a valid access key id".to_string()),
                _=> None
            });

        let secret_access_key = crate::validate_string(&self.secret_access_key, Some(5))
            .and_then(|error| match error {
                "empty" => Some("Please enter a secret access key".to_string()),
                "invalid" => Some("Please enter a valid secret access key".to_string()),
                _=> None
            });

        let bucket = crate::validate_string(&self.bucket, Some(2))
            .and_then(|error| match error {
                "empty" => Some("Please enter a bucket name".to_string()),
                "invalid" => Some("Please enter a valid bucket name".to_string()),
                _=> None
            });

        let region = crate::validate_string(&self.region, Some(2))
            .and_then(|error| match error {
                "empty" => Some("Please enter a region".to_string()),
                "invalid" => Some("Please enter a valid region".to_string()),
                _=> None
            });

        let error = Self {
            access_key_id,
            secret_access_key,
            bucket,
            region,
            ..Default::default()
        };

        match error.is_empty() {
            true => Ok(Payload::default()),
            false => Err(Payload::errors(error))
        }
    }
}