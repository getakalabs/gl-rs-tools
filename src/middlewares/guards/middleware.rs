use actix_web::{Error, HttpMessage, HttpResponse};
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::http::Method;
use actix_web::web::Data;
use actix_utils::future::{Either, ok, Ready};
use handlebars::Handlebars;
use std::task::{Context, Poll};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

use crate::catchers;
use crate::middlewares::guards::authentication::AuthenticationFuture;
use crate::middlewares::guards::options::GuardOptions;

use crate::DBClient;
use crate::Paseto;
use crate::Payload;

pub struct GuardMiddleware<S, T: 'static, R: ToString + Clone + PartialEq> {
    pub service: S,
    pub roles: Option<Vec<R>>,
    pub callback: super::GuardParams<R, T>,
    pub has_database: Option<bool>,
    pub json_response: bool,
    pub is_optional: bool,
    pub is_refresh_token: bool,
    pub is_web_token: bool,
}

impl<S, B, T, R> Service<ServiceRequest> for GuardMiddleware<S, T, R>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: MessageBody,
        T: 'static,
        R: ToString + Clone + PartialEq,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Either<AuthenticationFuture<S, B>, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut payload = Payload::authentication_expired();

        if Method::OPTIONS == *req.method() {
            return Either::left(AuthenticationFuture {
                fut: self.service.call(req),
                _phantom: PhantomData,
            });
        }

        let hbs = req.app_data::<Data<Handlebars<'_>>>();

        // Retrieve pg pool to validate token in database
        let has_database = self.has_database;
        let json_response = self.json_response;
        let mut db = DBClient::Null;
        let database = req.app_data::<Data<DBClient>>();
        match () {
            _ if has_database.is_some() && has_database.unwrap() && database.is_some() => {
                let database = database.unwrap().clone();
                match database.get_db().is_none() {
                    true => {
                        // Check response type
                        match json_response || hbs.is_none() {
                            true => payload = Payload::database(),
                            false => payload = catchers::not_found_middleware(hbs.unwrap().clone()),
                        }

                        // Return response
                        return Either::right(ok(req
                            .into_response(payload)
                            .map_into_boxed_body()
                            .map_into_right_body()));
                    },
                    false => {
                        db = database.get_client();
                    }
                }
            },
            _ if has_database.is_some() && has_database.unwrap() && database.is_none() => {
                // Check response type
                match json_response || hbs.is_none() {
                    true => payload = Payload::database(),
                    false => payload = catchers::not_found_middleware(hbs.unwrap().clone()),
                }

                // Return response
                return Either::right(ok(req
                    .into_response(payload)
                    .map_into_boxed_body()
                    .map_into_right_body()));
            }
            _ => {}
        }

        // Check if other options does not exist
        if has_database.is_some() && has_database.unwrap() &&
            self.callback.is_none() && self.roles.is_none() &&
            !self.is_refresh_token && !self.is_web_token {

            // Allow access
            return Either::left(AuthenticationFuture {
                fut: self.service.call(req),
                _phantom: PhantomData,
            });
        }

        // Retrieve allowed roles
        let roles = self.roles.clone();
        let has_allowed_roles = match roles.is_some() {
            true => !roles.clone().unwrap().is_empty(),
            false => false
        };

        // Check if roles exists and database does not exists
        if has_allowed_roles && (has_database.is_none() || !has_database.unwrap()) {
            match json_response || hbs.is_none() {
                true => payload = Payload::database(),
                false => payload = catchers::not_found_middleware(hbs.unwrap().clone()),
            }

            // Return response
            return Either::right(ok(req
                .into_response( payload)
                .map_into_boxed_body()
                .map_into_right_body()));
        }

        // Retrieve authorization
        let authorization = req
            .headers()
            .get("Authorization")
            .map(|h| h.to_str().unwrap_or(""))
            .unwrap_or("")
            .trim();

        // Retrieve token
        let token = crate::strings::get_token(authorization)
            .unwrap_or(String::new());

        // Create Options
        let guard_options = GuardOptions {
            token,
            roles,
            json_response: self.json_response,
            is_optional: self.is_optional,
            is_refresh_token: self.is_refresh_token,
            is_web_token: self.is_web_token
        };

        // Retrieve callback
        let callback = self.callback;

        // Retrieve paseto
        let paseto = req.app_data::<Data<Arc<RwLock<Paseto>>>>();
        if callback.is_some() && paseto.is_some() && paseto.unwrap().clone().read().is_ok() {

            if let Some(paseto) = paseto {
                let paseto = paseto.read().unwrap().clone();

                return match callback {
                    None => {
                        Either::right(ok(req
                            .into_response(payload)
                            .map_into_boxed_body()
                            .map_into_right_body()))
                    }
                    Some(callback) => {
                        return match (callback)(db.get_db().unwrap(), guard_options, paseto) {
                            Ok(claims) => {
                                req.extensions_mut().insert(claims);

                                Either::left(AuthenticationFuture {
                                    fut: self.service.call(req),
                                    _phantom: PhantomData,
                                })
                            },
                            Err(error) => {
                                let payload = match error.to_string().contains("expired") {
                                    true => {
                                        let payload = Payload{
                                            code: Some(401),
                                            error: error.to_string(),
                                            ..Default::default()
                                        };

                                        HttpResponse::Unauthorized()
                                            .content_type("application/json")
                                            .body(serde_json::to_string(&payload).unwrap())
                                    }
                                    false => {
                                        let payload = Payload{
                                            code: Some(400),
                                            error: error.to_string(),
                                            ..Default::default()
                                        };

                                        HttpResponse::BadRequest()
                                            .content_type("application/json")
                                            .body(serde_json::to_string(&payload).unwrap())
                                    }
                                };

                                // Disable access
                                Either::right(ok(req
                                    .into_response(payload)
                                    .map_into_boxed_body()
                                    .map_into_right_body()))
                            }
                        }
                    }
                }
            }
        }

        // Disable access
        Either::right(ok(req
            .into_response(payload)
            .map_into_boxed_body()
            .map_into_right_body()))
    }
}
