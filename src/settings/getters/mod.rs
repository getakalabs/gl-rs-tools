use crate::MongoObjectId;
use crate::Primitive;
use crate::Settings;

use crate::traits::*;

impl Settings {
    pub fn get_id(&self) -> Option<String> {
        self.id.clone().unwrap_or(MongoObjectId::None).get_string_id()
    }

    pub fn get_api_url(&self) -> String {
        self.api_url.clone().unwrap_or(String::default())
    }

    pub fn get_web_url(&self) -> String {
        self.web_url.clone().unwrap_or(String::default())
    }

    pub fn get_admin_url(&self) -> String {
        self.admin_url.clone().unwrap_or(String::default())
    }

    pub fn get_access_token_key_unit(&self) -> Primitive {
        self.access_token_key_unit.clone().unwrap_or(Primitive::None)
    }

    pub fn get_access_token_key_time(&self) -> String {
        match self.access_token_key_time.clone() {
            None => String::default(),
            Some(data) => data
        }
    }

    pub fn get_access_token_key_signing(&self) -> String {
        self.access_token_key_signing.clone().unwrap_or(String::default())
    }

    pub fn get_refresh_token_key_unit(&self) -> Primitive {
        self.refresh_token_key_unit.clone().unwrap_or(Primitive::None)
    }

    pub fn get_refresh_token_key_time(&self) -> String {
        self.refresh_token_key_time.clone().unwrap_or(String::default())
    }

    pub fn get_refresh_token_key_signing(&self) -> String {
        self.refresh_token_key_signing.clone().unwrap_or(String::default())
    }

    pub fn get_access_key_id(&self) -> String {
        self.access_key_id.clone().unwrap_or(String::default())
    }

    pub fn get_secret_access_key(&self) -> String {
        self.secret_access_key.clone().unwrap_or(String::default())
    }

    pub fn get_bucket(&self) -> String {
        self.bucket.clone().unwrap_or(String::default())
    }

    pub fn get_path(&self) -> String {
        self.path.clone().unwrap_or(String::default())
    }

    pub fn get_region(&self) -> String {
        self.region.clone().unwrap_or(String::default())
    }

    pub fn get_image_small_size(&self) -> Option<i32> {
        self
            .image_small_size
            .clone()
            .unwrap_or(Primitive::I32(72))
            .get_i32()
    }

    pub fn get_image_medium_size(&self) -> Option<i32> {
        self
            .image_medium_size
            .clone()
            .unwrap_or(Primitive::I32(192))
            .get_i32()
    }

    pub fn get_image_large_size(&self) -> Option<i32> {
        self
            .image_large_size
            .clone()
            .unwrap_or(Primitive::I32(512))
            .get_i32()
    }

    pub fn get_image_xl_size(&self) -> Option<i32> {
        self
            .image_xl_size
            .clone()
            .unwrap_or(Primitive::I32(1024))
            .get_i32()
    }

    pub fn get_sender(&self) -> String {
        self.sender.clone().unwrap_or(String::default())
    }

    pub fn get_username(&self) -> String {
        self.username.clone().unwrap_or(String::default())
    }

    pub fn get_password(&self) -> String {
        self.password.clone().unwrap_or(String::default())
    }

    pub fn get_smtp_host(&self) -> String {
        self.smtp_host.clone().unwrap_or(String::default())
    }

    pub fn get_service(&self) -> String {
        self.service.clone().unwrap_or(String::default())
    }
}