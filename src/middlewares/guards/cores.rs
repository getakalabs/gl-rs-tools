use actix_web::Error;
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Method;
use actix_web::web::Data;
use actix_utils::future::{Either, ok, Ready};
use handlebars::Handlebars;
use std::task::{Context, Poll};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

use crate::middlewares::guards::AuthenticationFuture;
use crate::MongoDBManager;
use crate::Paseto;
use crate::Payload;

/// Core guard checks for all core settings and configurations
/// such as database connection, handlebars and paseto.
/// This should always sit on the very top of wrap declaration in actix
#[warn(clippy::module_inception)]
#[derive(Default)]
pub struct CoreGuard {}

impl<S, B> Transform<S, ServiceRequest> for CoreGuard
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = CoreGuardMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CoreGuardMiddleware { service })
    }
}

pub struct CoreGuardMiddleware<S> {
    pub service: S
}

impl<S, B> Service<ServiceRequest> for CoreGuardMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: MessageBody,
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
        match req.app_data::<Data<MongoDBManager>>() {
            None => return Either::right(ok(req
                .into_response(payload)
                .map_into_boxed_body()
                .map_into_right_body())),
            Some(database) => {
                match database.get() {
                    Ok(_) => {},
                    Err(_) => return Either::right(ok(req
                        .into_response(payload)
                        .map_into_boxed_body()
                        .map_into_right_body()))
                }
            }
        }

        // Check for handlebars
        let payload = Payload::middleware();
        match req.app_data::<Data<Handlebars<'_>>>() {
            None => return Either::right(ok(req
                .into_response(payload)
                .map_into_boxed_body()
                .map_into_right_body())),
            Some(_) => {}
        }

        // Check paseto
        match req.app_data::<Data<Arc<RwLock<Paseto>>>>() {
            None => return Either::right(ok(req
                .into_response(payload)
                .map_into_boxed_body()
                .map_into_right_body())),
            Some(_) => {}
        }

        // Return success
        Either::left(AuthenticationFuture {
            fut: self.service.call(req),
            _phantom: PhantomData
        })
    }
}
