pub mod getters;
pub mod queries;
pub mod setters;
pub mod validations;

use arraygen::Arraygen;
use sanitizer::prelude::*;
use serde::{Serialize, Deserialize};

pub use crate::Base;
pub use crate::Mailer;
pub use crate::Module;
pub use crate::MongoDateTime;
pub use crate::MongoObjectId;
pub use crate::Paseto;
pub use crate::Payload;
pub use crate::Primitive;
pub use crate::S3;

pub use crate::traits::*;

pub static TABLE_SETTINGS: &str = "settings";

#[derive(Debug, Default, Clone, PartialEq, Sanitize, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_base: &mut Option<Base>)]
#[gen_array(fn get_mailer: &mut Option<Mailer>)]
#[gen_array(fn get_paseto: &mut Option<Paseto>)]
#[gen_array(fn get_s3: &mut Option<S3>)]
#[gen_array(fn get_mongo_object_id: &mut Option<MongoObjectId>)]
#[gen_array(fn get_mongo_date_time: &mut Option<MongoDateTime>)]
pub struct Settings {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[in_array(get_mongo_object_id)]
    pub id: Option<MongoObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_base)]
    pub base: Option<Base>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_mailer)]
    pub mailer: Option<Mailer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_paseto)]
    pub paseto: Option<Paseto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_s3)]
    pub s3: Option<S3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<Module>,
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
    pub image_small_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_medium_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_large_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_xl_size: Option<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_mongo_date_time)]
    pub created_at: Option<MongoDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_mongo_date_time)]
    pub updated_at: Option<MongoDateTime>,
}

impl Decrypt for Settings {
    fn decrypt(&self) -> Self {
        let mut data = self.clone();

        for base in data.get_base() {
            *base = base.clone().and_then(|d| match d.decrypt().is_empty() {
                true => None,
                false => Some(d.decrypt())
            });
        }

        for mailer in data.get_mailer() {
            *mailer = mailer.clone().and_then(|d| match d.decrypt().is_empty() {
                true => None,
                false => Some(d.decrypt())
            });
        }

        for paseto in data.get_paseto() {
            *paseto = paseto.clone().and_then(|d| match d.decrypt().is_empty() {
                true => None,
                false => Some(d.decrypt())
            });
        }

        for s3 in data.get_s3() {
            *s3 = s3.clone().and_then(|d| match d.decrypt().is_empty() {
                true => None,
                false => Some(d.decrypt())
            });
        }

        data
    }
}

impl Encrypt for Settings {
    fn encrypt(&self) -> Self {
        let mut data = self.clone();

        for base in data.get_base() {
            *base = base.clone().and_then(|d| match d.encrypt().is_empty() {
                true => None,
                false => Some(d.encrypt())
            });
        }

        for mailer in data.get_mailer() {
            *mailer = mailer.clone().and_then(|d| match d.encrypt().is_empty() {
                true => None,
                false => Some(d.encrypt())
            });
        }

        for paseto in data.get_paseto() {
            *paseto = paseto.clone().and_then(|d| match d.encrypt().is_empty() {
                true => None,
                false => Some(d.encrypt())
            });
        }

        for s3 in data.get_s3() {
            *s3 = s3.clone().and_then(|d| match d.encrypt().is_empty() {
                true => None,
                false => Some(d.encrypt())
            });
        }

        data
    }
}

impl IsEmpty for Settings {
    fn is_empty(&self) -> bool {
        self.clone() == Self::default()
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
        let mut data = self.clone().encrypt();

        for mongo_object_id in data.get_mongo_object_id() {
            *mongo_object_id = match mongo_object_id {
                None => None,
                Some(value) =>  value.get_mongo_object_id()
            };
        }

        for mongo_date_time in data.get_mongo_date_time() {
            *mongo_date_time = match mongo_date_time.clone() {
                None => None,
                Some(value) =>  value.get_mongo_date_time()
            };
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl ToJson for Settings {
    fn to_json(&self) -> Option<Self> {
        let mut data = self.clone().decrypt();

        for base in data.get_base() {
            *base = base.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => match d.is_empty() {
                    true => None,
                    false => d.to_json()
                }
            });
        }

        for mailer in data.get_mailer() {
            *mailer = mailer.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => match d.is_empty() {
                    true => None,
                    false => d.to_json()
                }
            });
        }

        for paseto in data.get_paseto() {
            *paseto = paseto.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => match d.is_empty() {
                    true => None,
                    false => d.to_json()
                }
            });
        }

        for s3 in data.get_s3() {
            *s3 = s3.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => match d.is_empty() {
                    true => None,
                    false => d.to_json()
                }
            });
        }

        for mongo_object_id in data.get_mongo_object_id() {
            *mongo_object_id = mongo_object_id.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => Some(d.set_to_string())
            });
        }

        for mongo_date_time in data.get_mongo_date_time() {
            *mongo_date_time = mongo_date_time.clone().and_then(|d| match d.is_empty() {
                true => None,
                false => Some(d.set_to_string())
            });

        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl ToPayload for Settings {
    fn to_payload(&self, code:u16) -> Payload {
        let mut payload = Payload{
            code: Some(200),
            ..Default::default()
        };

        match code {
            200 => payload.data = serde_json::to_value(self.clone()).unwrap(),
            _ => payload.errors = serde_json::to_value(self.clone()).unwrap()
        }

        payload
    }
}

impl From<&Base> for Settings {
    fn from(value: &Base) -> Self {
        Self::default()
            .set_base(value)
            .set_insertable()
    }
}

impl From<&Mailer> for Settings {
    fn from(value: &Mailer) -> Self {
        Self::default()
            .set_mailer(value)
            .set_insertable()
    }
}

impl From<&Module> for Settings {
    fn from(value: &Module) -> Self {
        Self::default()
            .set_module(value)
    }
}

impl From<&Paseto> for Settings {
    fn from(value: &Paseto) -> Self {
        Self::default()
            .set_paseto(value)
            .set_insertable()
    }
}

impl From<&S3> for Settings {
    fn from(value: &S3) -> Self {
        Self::default()
            .set_s3(value)
            .set_insertable()
    }
}