use actix_web::{
    Error,
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, errors::ErrorKind};
use std::rc::Rc;

use crate::{
    models::{auth::Claims, message::ErrorMessage},
    utils::responder::ApiResponder,
};

pub struct AuthMiddleware {
    secret: Rc<String>,
}

impl AuthMiddleware {
    pub fn new(secret: String) -> Self {
        AuthMiddleware {
            secret: Rc::new(secret),
        }
    }
}

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service,
            secret: Rc::clone(&self.secret),
        })
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: S,
    secret: Rc<String>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = Rc::clone(&self.secret);
        let auth_header = req.headers().get("Authorization").cloned();

        if let Some(auth_value) = auth_header {
            if let Ok(auth_str) = auth_value.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    let validation = Validation::new(Algorithm::HS256);
                    let result = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret.as_bytes()),
                        &validation,
                    );

                    match result {
                        Ok(_) => {
                            let fut = self.service.call(req);
                            return Box::pin(async move { fut.await });
                        }
                        Err(e) => match e.kind() {
                            ErrorKind::ExpiredSignature => {
                                let expired_response = ApiResponder::unauthorized(
                                    "Token expired".to_string(),
                                    None::<()>,
                                );
                                return Box::pin(
                                    async move { Ok(req.into_response(expired_response)) },
                                );
                            }
                            ErrorKind::InvalidToken => {
                                let invalid_response = ApiResponder::unauthorized(
                                    "Invalid token".to_string(),
                                    None::<()>,
                                );
                                return Box::pin(
                                    async move { Ok(req.into_response(invalid_response)) },
                                );
                            }
                            _ => {
                                let generic_error_response = ApiResponder::unauthorized(
                                    "Token validation error".to_string(),
                                    None::<()>,
                                );
                                return Box::pin(async move {
                                    Ok(req.into_response(generic_error_response))
                                });
                            }
                        },
                    }
                }
            }
        }

        let unauthorized_response =
            ApiResponder::unauthorized(ErrorMessage::TokenInvalid.to_string(), None::<()>);
        Box::pin(async move { Ok(req.into_response(unauthorized_response)) })
    }
}
