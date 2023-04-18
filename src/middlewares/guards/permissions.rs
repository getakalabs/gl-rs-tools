use actix_web::{Error, HttpMessage, Result};
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Method;
use actix_web::web::Data;
use actix_utils::future::{Either, ok, Ready};
use mongodb::Database;
use std::task::{Context, Poll};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

use crate::middlewares::guards::AuthenticationFuture;
use crate::MongoDBManager;
use crate::Paseto;
use crate::Payload;

/// Create permission guard params
pub type PermissionGuardParams<T, R> = Option<fn(Database, PermissionGuardOptions<R>, Paseto) -> Result<T>>;

// Create permission guard option
#[derive(Debug, Clone, PartialEq)]
pub struct PermissionGuardOptions<R: ToString + Clone + PartialEq> {
    pub token: String,
    pub roles: Option<Vec<R>>,
    pub is_optional: bool,
    pub is_refresh_token: bool,
    pub is_web_token: bool,
}

/// Permission guard checks for roles and permission for each endpoints
#[warn(clippy::module_inception)]
#[derive(Default)]
pub struct PermissionGuard<T: 'static, R: ToString + Clone + PartialEq> {
    pub roles: Option<Vec<R>>,
    pub callback: PermissionGuardParams<T, R>,
    pub is_optional: bool,
    pub is_refresh_token: bool,
    pub is_web_token: bool,
}

impl<T, R: ToString + Clone + PartialEq> PermissionGuard<T, R> {
    pub fn roles(roles:Vec<R>, callback: PermissionGuardParams<T, R>) -> Self {
        Self {
            roles: Some(roles),
            callback,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: false,
        }
    }

    pub fn refresh(roles:Vec<R>, callback: PermissionGuardParams<T, R>) -> Self {
        Self {
            roles: Some(roles),
            callback,
            is_optional: false,
            is_refresh_token: true,
            is_web_token: false,
        }
    }

    pub fn web(roles:Vec<R>, callback: PermissionGuardParams<T, R>) -> Self {
        Self {
            roles: Some(roles),
            callback,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: true,
        }
    }

    pub fn optional(roles:Vec<R>, callback: PermissionGuardParams<T, R>) -> Self {
        Self {
            roles: Some(roles),
            callback,
            is_optional: true,
            is_refresh_token: false,
            is_web_token: false,
        }
    }
}

impl<S, B, T, R> Transform<S, ServiceRequest> for PermissionGuard<T, R>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: MessageBody,
        T: 'static,
        R: ToString + Clone + PartialEq,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = PermissionGuardMiddleware<S, T, R>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let roles = self.roles.clone();
        let callback = self.callback;
        let is_optional = self.is_optional;
        let is_refresh_token = self.is_refresh_token;
        let is_web_token = self.is_web_token;

        ok(PermissionGuardMiddleware {
            service ,
            roles,
            callback,
            is_optional,
            is_refresh_token,
            is_web_token,
        })
    }
}

pub struct PermissionGuardMiddleware<S, T: 'static, R: ToString + Clone + PartialEq> {
    pub service: S,
    pub roles: Option<Vec<R>>,
    pub callback: PermissionGuardParams<T, R>,
    pub is_optional: bool,
    pub is_refresh_token: bool,
    pub is_web_token: bool,
}

impl<S, B, T, R> Service<ServiceRequest> for PermissionGuardMiddleware<S, T, R>
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
        if Method::OPTIONS == *req.method() {
            return Either::left(AuthenticationFuture {
                fut: self.service.call(req),
                _phantom: PhantomData,
            });
        }

        // Check database access
        let payload = Payload::database();
        let database = match req.app_data::<Data<MongoDBManager>>() {
            None => return Either::right(ok(req
                .into_response(payload)
                .map_into_boxed_body()
                .map_into_right_body())),
            Some(database) => {
                match database.get() {
                    Ok(database) => database,
                    Err(_) => return Either::right(ok(req
                        .into_response(payload)
                        .map_into_boxed_body()
                        .map_into_right_body()))
                }
            }
        };

        // Check paseto
        let paseto = match req.app_data::<Data<Arc<RwLock<Paseto>>>>() {
            None => return Either::right(ok(req
                .into_response(payload)
                .map_into_boxed_body()
                .map_into_right_body())),
            Some(paseto) => paseto.read().unwrap().clone(),
        };

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
        let options = PermissionGuardOptions {
            token,
            roles: self.roles.clone(),
            is_optional: self.is_optional,
            is_refresh_token: self.is_refresh_token,
            is_web_token: self.is_web_token
        };

        // Check callback
        if let Some(callback) = self.callback {
            return match (callback)(database, options, paseto) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);

                    Either::left(AuthenticationFuture {
                        fut: self.service.call(req),
                        _phantom: PhantomData,
                    })
                },
                Err(error) => Either::right(ok(req
                    .into_response(error)
                    .map_into_boxed_body()
                    .map_into_right_body()))
            }
        }

        // Return error
        Either::right(ok(req
            .into_response(payload)
            .map_into_boxed_body()
            .map_into_right_body()))
    }
}
