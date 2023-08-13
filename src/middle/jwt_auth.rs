use std::collections::{HashMap, HashSet};
use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, error, http::header::HeaderValue, web::Query,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};

use crate::api::AccessToken;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware 's call method gets called with normal request.
#[derive(Clone)]
pub struct JwtAuth(HashSet<String>, DecodingKey, Validation);

impl Default for JwtAuth {
    fn default() -> Self {
        let lst = ["/api/auth", "/api/refresh", ];
        let whitelist = lst.iter().map(|i| i.to_string()).collect::<HashSet<String>>();
        let decoding_key: DecodingKey = DecodingKey::from_secret("my_secret_access_token".as_ref());
        let validation = Validation::new(Algorithm::HS512);

        Self(whitelist, decoding_key, validation)
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for JwtAuth
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service,
            whitelist: self.0.clone(),
            decoding_key: self.1.clone(),
            validation: self.2.clone()
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
    whitelist: HashSet<String>,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path();
        if self.whitelist.contains(&path as &str) {
            let fut = self.service.call(req);
            return Box::pin(async move { Ok(fut.await?) });
        }

        let mut token = req.headers().get("Authorization")
            .unwrap_or(&HeaderValue::from_str("").unwrap())
            .to_str().unwrap().replace("Bearer ", "");
        if token.is_empty() {
            token = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap()
                .get("access_token").unwrap_or(&"".to_string()).to_string();
        }
        // println!("token: {}", &token);
        let rst = jsonwebtoken::decode::<AccessToken>(&token, &self.decoding_key, &self.validation);
        if rst.is_err() {
            let err_msg = rst.err().unwrap().to_string();
            return Box::pin(async move { Err(error::ErrorUnauthorized(format!("{{\"code\": 401, \"success\": false, \"msg\": \"{}\"}}", &err_msg))) });
        }

        let fut = self.service.call(req);
        Box::pin(async move { Ok(fut.await?) })
    }
}
