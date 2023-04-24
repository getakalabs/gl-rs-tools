use actix_web::error::{InternalError, JsonPayloadError, PayloadError};
use actix_web::HttpResponse;
use actix_web::web::JsonConfig;

use crate::Payload;

// Create staging for json config
pub fn stage(json_limit: usize) -> JsonConfig {
    JsonConfig::default()
        .limit(json_limit)
        .error_handler(|err, _req| {
            // Create new json response
            let mut response = Payload {
                code: Some(400),
                ..Default::default()
            };

            // Match error
            match err {
                JsonPayloadError::ContentType => response.error = Some(String::from("Invalid Content-Type header")),
                JsonPayloadError::Deserialize(error) => response.error = Some(format!("Json deserialize error: {error}")),
                JsonPayloadError::Payload(error) => {
                    match error {
                        PayloadError::Incomplete(error) => response.error = Some(format!("A payload reached EOF, but is not complete. With error: {}", error.unwrap())),
                        PayloadError::EncodingCorrupted => response.error = Some(String::from("Can not decode content-encoding")),
                        PayloadError::Overflow => response.error = Some(String::from("Json payload size is bigger than allowed")),
                        PayloadError::UnknownLength => response.error = Some(String::from("A payload length is unknown")),
                        PayloadError::Http2Payload(error) => response.error = Some(error.to_string()),
                        PayloadError::Io(error) => response.error = Some(error.to_string()),
                        _ => response.error = Some(String::from("An error occurred while processing your request")),
                    }
                },
                _ => response.error = Some(String::from("An error occurred while processing your request")),
            }

            InternalError::from_response(
                JsonPayloadError::ContentType,
                HttpResponse::BadRequest().json(response)
            ).into()
        })
}