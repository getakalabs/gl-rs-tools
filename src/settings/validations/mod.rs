use crate::Settings;

impl Settings {
    pub fn validate_base(&self) -> Self {
        Self {
            api_url: crate::validate_string(&self.api_url, "Please enter your API URL", "Please enter a valid API URL", Some(5)),
            web_url: crate::validate_string(&self.web_url, "Please enter your web URL", "Please enter a valid web URL", Some(5)),
            admin_url: crate::validate_string(&self.admin_url, "Please enter your admin URL", "Please enter a valid admin URL", Some(5)),
            ..Default::default()
        }
    }

    pub fn validate_mailer(&self) -> Self {
        Self {
            sender: crate::validate_string(&self.sender, "Please enter the sender of your mailer", "Please enter a valid sender", Some(5)),
            username: crate::validate_string(&self.username, "Please enter the mailer username", "Please enter a valid username", Some(5)),
            password: crate::validate_string(&self.password, "Please enter the mailer password", "Please enter a valid password", Some(5)),
            smtp_host: crate::validate_string(&self.smtp_host, "Please enter the mailer SMTP host", "Please enter a valid SMTP host", Some(5)),
            service: crate::validate_string(&self.smtp_host, "Please enter the mailer service", "Please enter a valid mailer services", Some(5)),
            email: crate::validate_email(&self.email, "Please enter your email", "Please enter a valid email"),
            ..Default::default()
        }
    }

    pub fn validate_paseto(&self) -> Self {
        let options = vec![
            String::from("seconds"),
            String::from("minutes"),
            String::from("hours"),
            String::from("days")
        ];

        Self {
            access_token_key_unit: crate::validate_primitive_i32(&self.access_token_key_unit, "Please enter an access token key unit", "Please enter a valid access token key unit", Some(1)),
            access_token_key_time: crate::validate_string_options(&self.access_token_key_time, "Please enter your access token key time", "Please enter a valid access token key time", &options),
            access_token_key_signing: crate::validate_string_base64(&self.access_token_key_signing, "Please enter your access token key signing", "Please enter a valid access token key signing", 32),
            refresh_token_key_unit: crate::validate_primitive_i32(&self.refresh_token_key_unit, "Please enter an refresh token key unit", "Please enter a valid refresh token key unit", Some(1)),
            refresh_token_key_time: crate::validate_string_options(&self.refresh_token_key_time, "Please enter your refresh token key time", "Please enter a valid refresh token key time", &options),
            refresh_token_key_signing: crate::validate_string_base64(&self.refresh_token_key_signing, "Please enter your refresh token key signing", "Please enter a valid refresh token key signing", 32),
            ..Default::default()
        }
    }

    pub fn validate_s3(&self) -> Self {
        Self {
            access_key_id: crate::validate_string(&self.access_key_id, "Please enter an access key id", "Please enter a valid access key id", Some(5)),
            secret_access_key: crate::validate_string(&self.secret_access_key, "Please enter a secret access key", "Please enter a valid secret access key", Some(5)),
            bucket: crate::validate_string(&self.bucket, "Please enter a bucket name", "Please enter a valid bucket name", Some(5)),
            region: crate::validate_string(&self.region, "Please enter a region", "Please enter a valid region", Some(5)),
            ..Default::default()
        }
    }
}