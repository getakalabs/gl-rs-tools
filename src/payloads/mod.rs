use actix_web::{error, HttpRequest, HttpResponse, Responder};
use actix_web::{body::BoxBody, http::StatusCode};
use display_json::DisplayAsJsonPretty;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, PartialEq, DisplayAsJsonPretty, Serialize, Deserialize)]
pub struct Payload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>
}

impl Responder for Payload {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let payload = serde_json::to_string(&self).unwrap();

        let mut code = 400;
        match () {
            _ if self.code.as_ref().is_some() => code = *self.code.as_ref().unwrap() as i32,
            _ if self.code.as_ref().is_none() && !self.challenge.unwrap_or_default().is_empty() => code = 200,
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

impl error::ResponseError for Payload {
    fn status_code(&self) -> StatusCode {
        match self.code {
            Some(200) => StatusCode::OK,
            Some(400) => StatusCode::BAD_REQUEST,
            Some(401) => StatusCode::UNAUTHORIZED,
            Some(403) => StatusCode::FORBIDDEN,
            Some(404) => StatusCode::NOT_FOUND,
            Some(405) => StatusCode::METHOD_NOT_ALLOWED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }

    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let payload = serde_json::to_string(&self).unwrap();

        let mut code = 400;
        match () {
            _ if self.code.as_ref().is_some() => code = *self.code.as_ref().unwrap() as i32,
            _ if self.code.as_ref().is_none() && !self.challenge.clone().unwrap_or_default().is_empty() => code = 200,
            _ => ()
        }

        match code {
            200 => HttpResponse::Ok(),
            401 => HttpResponse::Unauthorized(),
            403 => HttpResponse::Forbidden(),
            404 => HttpResponse::NotFound(),
            405 => HttpResponse::MethodNotAllowed(),
            500 => HttpResponse::InternalServerError(),
            _ => HttpResponse::BadRequest()
        }.content_type("application/json")
            .body(payload)
    }
}

impl Payload {
    pub fn data<T>(code:usize, data:T) -> Self
        where T: Serialize
    {
        Self {
            code: Some(code as u16),
            data: Some(serde_json::to_value(data)
                .unwrap_or(serde_json::Value::Null)),
            ..Default::default()
        }
    }

    pub fn error<T: ToString>(error: T) -> error::Error {
        let error = error.to_string();

        let code = match error.to_lowercase().as_str() == "your authentication token has expired" {
            true => Some(401),
            false => Some(400)
        };

        Self {
            code,
            error: Some(error),
            ..Default::default()
        }.into()
    }

    pub fn errors<T>(error:T) -> error::Error
        where T: Serialize
    {
        Self {
            code: Some(400),
            errors: Some(serde_json::to_value(error)
                .unwrap_or(serde_json::Value::Null)),
            ..Default::default()
        }.into()
    }

    pub fn success<T: ToString>(message: T) -> Self {
        Self {
            code: Some(200),
            message: Some(message.to_string()),
            ..Default::default()
        }
    }

    pub fn authentication_expired() -> HttpResponse {
        let payload = Self {
            code: Some(401),
            error: Some(String::from("Authentication token expired")),
            ..Default::default()
        };

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    pub fn database() -> HttpResponse {
        let payload = Self {
            code: Some(400),
            error: Some(String::from("Invalid database configuration")),
            ..Default::default()
        };

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }

    pub fn middleware() -> HttpResponse {
        let payload = Self {
            code: Some(400),
            error: Some(String::from("Missing middleware. Please configure your server properly")),
            ..Default::default()
        };

        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(serde_json::to_string(&payload).unwrap())
    }
}