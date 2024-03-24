use std:: future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, error, Error, FromRequest, HttpMessage
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey,Validation};

use crate::models::user::{User, UserClaim};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authorization;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware { service }))
    }
}
pub struct AuthorizationMiddleware<S> {
    service: S,
}
impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("authorization");
        if token.is_none(){
           
            return Box::pin(async move {
                Err(error::ErrorUnauthorized("Unauthorized"))
            });
        }
        let token_str=token.unwrap().to_str().unwrap().split(" ").collect::<Vec<&str>>()[1];
        let decode_result =decode::<UserClaim>(token_str, &DecodingKey::from_secret("123456789".as_ref()), &Validation::new(Algorithm::HS256));
        if decode_result.is_err(){
            return Box::pin(async move {
                Err(error::ErrorUnauthorized("Unauthorized Invalid jwt"))
            });
        }
        req.extensions_mut().insert(decode_result.unwrap().claims.auth_user);
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}


impl  FromRequest for  User{
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let user = req.extensions().get::<User>().unwrap().clone();
        ready(Ok(user))
    }
}