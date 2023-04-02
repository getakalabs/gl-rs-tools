use actix_web::Error;
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_utils::future::{ok, Ready};

use crate::middlewares::GuardMiddleware;

#[warn(clippy::module_inception)]
pub struct Guard<T: 'static, R: ToString + Clone + PartialEq> {
    pub roles: Option<Vec<R>>,
    pub callback: super::GuardParams<R, T>,
    pub has_database: Option<bool>,
    pub json_response: bool,
    pub is_optional: bool,
    pub is_refresh_token: bool,
    pub is_web_token: bool,
}

impl<T, R: ToString + Clone + PartialEq> Default for Guard<T, R> {
    fn default() -> Self {
        Self {
            roles: None,
            callback: None,
            has_database: None,
            json_response: false,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: false,
        }
    }
}

impl<T, R: ToString + Clone + PartialEq> Guard<T, R> {
    pub fn database() -> Self {
        Self {
            roles: None,
            callback: None,
            has_database: Some(true),
            json_response: true,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: false,
        }
    }

    pub fn roles(roles:Vec<R>, callback: super::GuardParams<R, T>) -> Self {
        let roles = Some(roles);
        let has_database = Some(true);
        let json_response = true;
        let is_optional = false;
        let is_refresh_token = false;
        let is_web_token = false;

        Self { roles, callback, has_database, json_response, is_optional, is_refresh_token, is_web_token }
    }

    pub fn refresh(roles:Vec<R>, callback: super::GuardParams<R, T>) -> Self {
        Self {
            roles: Some(roles),
            callback,
            has_database: Some(true),
            json_response: true,
            is_optional: false,
            is_refresh_token: true,
            is_web_token: false,
        }
    }

    pub fn web(roles:Vec<R>, callback: super::GuardParams<R, T>) -> Self {
        Self {
            roles: Some(roles),
            callback,
            has_database: Some(true),
            json_response: true,
            is_optional: false,
            is_refresh_token: false,
            is_web_token: true,
        }
    }

    pub fn optional(roles:Vec<R>, callback: super::GuardParams<R, T>) -> Self {
        Self {
            roles: Some(roles),
            callback,
            has_database: Some(true),
            json_response: true,
            is_optional: true,
            is_refresh_token: false,
            is_web_token: false,
        }
    }

    /// Set guard as json response
    pub fn set_json_response(&mut self) -> &mut Self {
        self.json_response = true;
        self
    }
}

impl<S, B, T, R> Transform<S, ServiceRequest> for Guard<T, R>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: MessageBody,
        T: 'static,
        R: ToString + Clone + PartialEq,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = GuardMiddleware<S, T, R>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let roles = self.roles.clone();
        let callback = self.callback;
        let has_database = self.has_database;
        let json_response = self.json_response;
        let is_optional = self.is_optional;
        let is_refresh_token = self.is_refresh_token;
        let is_web_token = self.is_web_token;

        ok(GuardMiddleware {
            service,
            roles,
            callback,
            has_database,
            json_response,
            is_optional,
            is_refresh_token,
            is_web_token,
        })
    }
}
