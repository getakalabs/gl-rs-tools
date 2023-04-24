pub mod queries;
pub mod setters;
pub mod validations;

use arraygen::Arraygen;
use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};

pub static TABLE_SETTINGS: &str = "settings";

use crate::traits::prelude::*;
use crate::Base;
use crate::Mailer;
use crate::MongoObjectId;
use crate::MongoDateTime;
use crate::Paseto;
use crate::Payload;
use crate::Primitive;
use crate::S3;

#[derive(Debug, Default, Clone, PartialEq, Sanitize, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_array_id: &mut Option<MongoObjectId>)]
#[gen_array(fn get_array_date: &mut Option<MongoDateTime>)]
#[gen_array(fn get_array_base: &mut Option<Base>)]
#[gen_array(fn get_array_mailer: &mut Option<Mailer>)]
#[gen_array(fn get_array_paseto: &mut Option<Paseto>)]
#[gen_array(fn get_array_s3: &mut Option<S3>)]
pub struct Settings {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_id)]
    pub id: Option<MongoObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_base)]
    pub base: Option<Base>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_mailer)]
    pub mailer: Option<Mailer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_paseto)]
    pub paseto: Option<Paseto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_s3)]
    pub s3: Option<S3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_date)]
    pub created_at: Option<MongoDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_array_date)]
    pub updated_at: Option<MongoDateTime>,
    #[sanitize(trim, lower_case)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
    #[sanitize(trim, lower_case)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[sanitize(trim, lower_case)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_url: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_key_unit: Option<Primitive>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_key_time: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_key_signing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_key_unit: Option<Primitive>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_key_time: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_key_signing: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[sanitize(trim, lower_case)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_host: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[sanitize(trim, lower_case)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[sanitize(trim)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_small_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_medium_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_large_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_thumbnail_xl_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_small_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_small_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_medium_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_medium_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_large_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_large_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_xl_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_xl_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_xxl_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_xxl_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_width_xxxl_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_landscape_height_xxxl_size: Option<Primitive>,
}

impl Decrypt for Settings {
    fn decrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for base in data.get_array_base() {
            *base = base.clone().and_then(|d| d.decrypt());
        }

        for mailer in data.get_array_mailer() {
            *mailer = mailer.clone().and_then(|d| d.decrypt());
        }

        for paseto in data.get_array_paseto() {
            *paseto = paseto.clone().and_then(|d| d.decrypt());
        }

        for s3 in data.get_array_s3() {
            *s3 = s3.clone().and_then(|d| d.decrypt());
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl Encrypt for Settings {
    fn encrypt(&self) -> Option<Self> {
        let mut data = self.clone();

        for base in data.get_array_base() {
            *base = base.clone().and_then(|d| d.encrypt());
        }

        for mailer in data.get_array_mailer() {
            *mailer = mailer.clone().and_then(|d| d.encrypt());
        }

        for paseto in data.get_array_paseto() {
            *paseto = paseto.clone().and_then(|d| d.encrypt());
        }

        for s3 in data.get_array_s3() {
            *s3 = s3.clone().and_then(|d| d.encrypt());
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl From<Base> for Settings {
    fn from(base: Base) -> Self {
        let module = Some("Base".to_string());
        let base = match base.is_empty() {
            true => None,
            false => Some(base)
        };

        Self {
            module,
            base,
            ..Default::default()
        }.set_insertable()
    }
}

impl From<&Base> for Settings {
    fn from(base: &Base) -> Self {
        Self::from(base.clone())
    }
}

impl From<Mailer> for Settings {
    fn from(mailer: Mailer) -> Self {
        let module = Some("Mailer".to_string());
        let mailer = match mailer.is_empty() {
            true => None,
            false => Some(mailer)
        };

        Self {
            module,
            mailer,
            ..Default::default()
        }.set_insertable()
    }
}

impl From<&Mailer> for Settings {
    fn from(mailer: &Mailer) -> Self {
        Self::from(mailer.clone())
    }
}

impl From<Paseto> for Settings {
    fn from(paseto: Paseto) -> Self {
        let module = Some("Paseto".to_string());
        let paseto = match paseto.is_empty() {
            true => None,
            false => Some(paseto)
        };

        Self {
            module,
            paseto,
            ..Default::default()
        }.set_insertable()
    }
}

impl From<&Paseto> for Settings {
    fn from(paseto: &Paseto) -> Self {
        Self::from(paseto.clone())
    }
}

impl From<S3> for Settings {
    fn from(s3: S3) -> Self {
        let module = Some("S3".to_string());
        let s3 = match s3.is_empty() {
            true => None,
            false => Some(s3)
        };

        Self {
            module,
            s3,
            ..Default::default()
        }.set_insertable()
    }
}

impl From<&S3> for Settings {
    fn from(s3: &S3) -> Self {
        Self::from(s3.clone())
    }
}

impl IsEmpty for Settings {
    fn is_empty(&self) -> bool {
        Self::default() == *self
    }
}

impl Normalize for Settings {
    fn normalize(&self) -> Self {
        let mut data = self.clone();

        data.sanitize();

        data
    }
}

impl ToBson for Settings {
    fn to_bson(&self) -> Option<Self> {
        let mut data = self.clone();

        for id in data.get_array_id() {
            *id = id.clone().and_then(|d| d.to_bson());
        }

        for date in data.get_array_date() {
            *date = date.clone().and_then(|d| d.to_bson());
        }

        for base in data.get_array_base() {
            *base = base.clone().and_then(|d| d.to_bson());
        }

        for mailer in data.get_array_mailer() {
            *mailer = mailer.clone().and_then(|d| d.to_bson());
        }

        for paseto in data.get_array_paseto() {
            *paseto = paseto.clone().and_then(|d| d.to_bson());
        }

        for s3 in data.get_array_s3() {
            *s3 = s3.clone().and_then(|d| d.to_bson());
        }

        Some(data)
    }
}

impl ToJson for Settings {
    fn to_json(&self) -> Option<Self> {
        let mut data = self.clone();

        for id in data.get_array_id() {
            *id = id.clone().and_then(|d| d.to_json());
        }

        for date in data.get_array_date() {
            *date = date.clone().and_then(|d| d.to_json());
        }

        for base in data.get_array_base() {
            *base = base.clone().and_then(|d| d.to_json());
        }

        for mailer in data.get_array_mailer() {
            *mailer = mailer.clone().and_then(|d| d.to_json());
        }

        for paseto in data.get_array_paseto() {
            *paseto = paseto.clone().and_then(|d| d.to_json());
        }

        for s3 in data.get_array_s3() {
            *s3 = s3.clone().and_then(|d| d.to_json());
        }

        Some(data)
    }
}

impl ToPayload for Settings {
    fn to_payload(&self, code:usize) -> Payload {
        let mut payload = Payload{
            code: Some(200),
            ..Default::default()
        };

        match code {
            200 => payload.data = Some(serde_json::to_value(self.clone()).unwrap()),
            _ => payload.errors = Some(serde_json::to_value(self.clone()).unwrap())
        }

        payload
    }
}
