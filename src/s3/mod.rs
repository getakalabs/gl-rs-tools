pub mod impls;
pub mod mutations;
pub mod stages;

use arraygen::Arraygen;
use mongodb::bson::{Bson, Document};
use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};
use std::default::Default;

use crate::traits::prelude::*;
use crate::Cipher;
use crate::Settings;

#[derive(Debug, Default, Clone, PartialEq, Sanitize, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_array_ciphers: &mut Option<Cipher>)]
pub struct S3 {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub access_key_id: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub secret_access_key: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub bucket: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub path: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_ciphers)]
    pub region: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_small_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_medium_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_large_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_xl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_small_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_small_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_medium_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_medium_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_large_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_large_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_xl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_xl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_xxl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_xxl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_xxxl_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_xxxl_size: Option<i32>,
}

impl Decrypt for S3 {
    fn decrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_array_ciphers() {
            *cipher = cipher.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => match d.decrypt_master() {
                    Ok(d) => Some(d.set_to_string()),
                    Err(_) => Some(d.set_to_string())
                }
            });
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl Encrypt for S3 {
    fn encrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_array_ciphers() {
            *cipher = cipher.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => match d.encrypt_master() {
                    Ok(d) => Some(d),
                    Err(_) => Some(d)
                }
            });
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl From<S3> for Bson {
    fn from(value: S3) -> Self {
        Bson::Document(value.into())
    }
}

impl From<S3> for Document {
    fn from(value: S3) -> Document {
        match value.is_empty() {
            true => Document::new(),
            false => {
                let mut doc = Document::new();

                doc.insert("access_key_id", Bson::from(value.access_key_id));
                doc.insert("secret_access_key", Bson::from(value.secret_access_key));
                doc.insert("bucket", Bson::from(value.bucket));
                doc.insert("path", Bson::from(value.path));
                doc.insert("region", Bson::from(value.region));
                doc.insert("image_thumbnail_small_size", Bson::from(value.image_thumbnail_small_size));
                doc.insert("image_thumbnail_medium_size", Bson::from(value.image_thumbnail_medium_size));
                doc.insert("image_thumbnail_large_size", Bson::from(value.image_thumbnail_large_size));
                doc.insert("image_thumbnail_xl_size", Bson::from(value.image_thumbnail_xl_size));
                doc.insert("image_landscape_width_small_size", Bson::from(value.image_landscape_width_small_size));
                doc.insert("image_landscape_height_small_size", Bson::from(value.image_landscape_height_small_size));
                doc.insert("image_landscape_width_medium_size", Bson::from(value.image_landscape_width_medium_size));
                doc.insert("image_landscape_height_medium_size", Bson::from(value.image_landscape_height_medium_size));
                doc.insert("image_landscape_width_large_size", Bson::from(value.image_landscape_width_large_size));
                doc.insert("image_landscape_height_large_size", Bson::from(value.image_landscape_height_large_size));
                doc.insert("image_landscape_width_xl_size", Bson::from(value.image_landscape_width_xl_size));
                doc.insert("image_landscape_height_xl_size", Bson::from(value.image_landscape_height_xl_size));
                doc.insert("image_landscape_width_xxl_size", Bson::from(value.image_landscape_width_xxl_size));
                doc.insert("image_landscape_height_xxl_size", Bson::from(value.image_landscape_height_xxl_size));
                doc.insert("image_landscape_width_xxxl_size", Bson::from(value.image_landscape_width_xxxl_size));
                doc.insert("image_landscape_height_xxxl_size", Bson::from(value.image_landscape_height_xxxl_size));

                doc
            }
        }
    }
}

impl From<Settings> for S3 {
    fn from(value: Settings) -> Self {
        let access_key_id = Some(Cipher::from(value.access_key_id.map_or(String::default(), |d| d)));
        let secret_access_key = Some(Cipher::from(value.secret_access_key.map_or(String::default(), |d| d)));
        let bucket = Some(Cipher::from(value.bucket.map_or(String::default(), |d| d)));
        let path = Some(Cipher::from(value.path.map_or(String::default(), |d| d)));
        let region = Some(Cipher::from(value.region.map_or(String::default(), |d| d)));
        let image_thumbnail_small_size = Some(value.image_thumbnail_small_size.map_or(72, |d| d.get_i32().unwrap_or(72)));
        let image_thumbnail_medium_size = Some(value.image_thumbnail_medium_size.map_or(192, |d| d.get_i32().unwrap_or(192)));
        let image_thumbnail_large_size = Some(value.image_thumbnail_large_size.map_or(514, |d| d.get_i32().unwrap_or(514)));
        let image_thumbnail_xl_size = Some(value.image_thumbnail_xl_size.map_or(1024, |d| d.get_i32().unwrap_or(1024)));
        let image_landscape_width_small_size = Some(value.image_landscape_width_small_size.map_or(640, |d| d.get_i32().unwrap_or(640)));
        let image_landscape_height_small_size = Some(value.image_landscape_height_small_size.map_or(360, |d| d.get_i32().unwrap_or(360)));
        let image_landscape_width_medium_size = Some(value.image_landscape_width_medium_size.map_or(854, |d| d.get_i32().unwrap_or(854)));
        let image_landscape_height_medium_size = Some(value.image_landscape_height_medium_size.map_or(480, |d| d.get_i32().unwrap_or(480)));
        let image_landscape_width_large_size = Some(value.image_landscape_width_large_size.map_or(960, |d| d.get_i32().unwrap_or(960)));
        let image_landscape_height_large_size = Some(value.image_landscape_height_large_size.map_or(540, |d| d.get_i32().unwrap_or(540)));
        let image_landscape_width_xl_size = Some(value.image_landscape_width_xl_size.map_or(1136, |d| d.get_i32().unwrap_or(1136)));
        let image_landscape_height_xl_size = Some(value.image_landscape_height_xl_size.map_or(640, |d| d.get_i32().unwrap_or(640)));
        let image_landscape_width_xxl_size = Some(value.image_landscape_width_xxl_size.map_or(1280, |d| d.get_i32().unwrap_or(1280)));
        let image_landscape_height_xxl_size = Some(value.image_landscape_height_xxl_size.map_or(720, |d| d.get_i32().unwrap_or(720)));
        let image_landscape_width_xxxl_size = Some(value.image_landscape_width_xxxl_size.map_or(1920, |d| d.get_i32().unwrap_or(1920)));
        let image_landscape_height_xxxl_size = Some(value.image_landscape_height_xxxl_size.map_or(1080, |d| d.get_i32().unwrap_or(1080)));

        Self {
            access_key_id,
            secret_access_key,
            bucket,
            path,
            region,
            image_thumbnail_small_size,
            image_thumbnail_medium_size,
            image_thumbnail_large_size,
            image_thumbnail_xl_size,
            image_landscape_width_small_size,
            image_landscape_height_small_size,
            image_landscape_width_medium_size,
            image_landscape_height_medium_size,
            image_landscape_width_large_size,
            image_landscape_height_large_size,
            image_landscape_width_xl_size,
            image_landscape_height_xl_size,
            image_landscape_width_xxl_size,
            image_landscape_height_xxl_size,
            image_landscape_width_xxxl_size,
            image_landscape_height_xxxl_size,
        }
    }
}

impl From<&Settings> for S3 {
    fn from(value: &Settings) -> Self {
        Self::from(value.clone())
    }
}

impl IsEmpty for S3 {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

impl ToBson for S3 {
    fn to_bson(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => self.encrypt()
        }
    }
}

impl ToJson for S3 {
    fn to_json(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => self.decrypt()
        }
    }
}

impl ToOption for S3 {
    fn to_option(&self) -> Option<Self> {
        match self.is_empty() {
            true => None,
            false => Some(self.clone())
        }
    }
}