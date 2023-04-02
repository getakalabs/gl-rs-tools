use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use serde::{Serialize, Deserialize};
use std::fmt::Display;

use crate::constants::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub challenge: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub data: serde_json::Value,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error: String,
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub errors: serde_json::Value
}

impl Default for Payload {
    fn default() -> Self {
        Self {
            code: None,
            challenge: String::default(),
            message: String::default(),
            data: serde_json::Value::Null,
            error: String::default(),
            errors: serde_json::Value::Null,
        }
    }
}

impl Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let item = self.clone();
        write!(f, "{item}",)
    }
}

impl Responder for Payload {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let payload = serde_json::to_string(&self).unwrap();

        let mut code = 400;
        match () {
            _ if self.code.as_ref().is_some() => code = *self.code.as_ref().unwrap() as i32,
            _ if self.code.as_ref().is_none() && !self.challenge.is_empty() => code = 200,
            _ => ()
        }

        match code {
            200 => HttpResponse::Ok(),
            401 => HttpResponse::Unauthorized(),
            404 => HttpResponse::NotFound(),
            500 => HttpResponse::InternalServerError(),
            _ => HttpResponse::BadRequest()
        }.content_type("application/json")
            .body(payload)
    }
}

impl Payload {
    pub fn new(code: u16) -> Self {
        Self {
            code: Some(code),
            ..Default::default()
        }
    }

    pub fn authentication_expired() -> HttpResponse {
        let payload = Self {
            code: Some(401),
            error: String::from(AUTHENTICATION_TOKEN_EXPIRED),
            ..Default::default()
        };

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    pub fn challenge<T: ToString>(challenge: T) -> Self {
        Self {
            challenge: challenge.to_string(),
            ..Default::default()
        }
    }

    pub fn database() -> HttpResponse {
        let payload = Self {
            code: Some(400),
            error: String::from(INVALID_DATABASE_CONFIGURATION),
            ..Default::default()
        };

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    pub fn data<T>(code:u16, data:T) -> Self
        where T: Serialize
    {
        Self {
            code: Some(code),
            data: serde_json::to_value(data)
                .unwrap_or(serde_json::Value::Null),
            ..Default::default()
        }
    }

    pub fn error<T: ToString>(error: T) -> Self {
        Self {
            code: Some(400),
            error: error.to_string(),
            ..Default::default()
        }
    }

    pub fn page_not_found() -> Self {
        Self {
            code: Some(404),
            error: String::from(PAGE_NOT_FOUND),
            ..Default::default()
        }
    }

    pub fn permission() -> HttpResponse {
        let payload = Self {
            code: Some(403),
            error: String::from(PAGE_PERMISSION),
            ..Default::default()
        };

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    pub fn success<T: ToString>(message: T) -> Self {
        Self {
            code: Some(200),
            message: message.to_string(),
            ..Default::default()
        }
    }

    pub fn token_refresh_expired() -> HttpResponse {
        let payload = Self {
            code: Some(401),
            error: String::from(TOKEN_REFRESH_EXPIRED),
            ..Default::default()
        };

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    pub fn token_web_expired() -> HttpResponse {
        let payload = Self {
            code: Some(401),
            error: String::from(TOKEN_WEB_EXPIRED),
            ..Default::default()
        };

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }
}