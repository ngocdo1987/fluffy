use actix_service::{Service, Transform};
use actix_web::{Error, HttpResponse, 
    dev::{ServiceRequest, ServiceResponse},
};
use futures::{ Poll, future::{ok, Either, FutureResult}};
use crate::constants;
use crate::{jwt, response};

macro_rules! error_internal { 
    ($req: expr, $message: expr) => {
        Either::B(ok($req.into_response(
            HttpResponse::InternalServerError()
                .json(response::JsonError{code: 500, message: $message})
                .into_body()
        )))
    }
}

/// Intermediate component verification
pub struct Authentication;

impl<S, B> Transform<S> for Authentication
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = FutureResult<Self::Transform, Self::InitError>;
    
    /// Generate a mandarin
    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

/// Used to process verification
pub struct AuthenticationMiddleware<S> {
    service: S,
}
impl<S, B> Service for AuthenticationMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;
    
    /// Asynchronous
    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    /// Call
    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let auth_header = if let Some(v) = req.headers().get(constants::AUTHORIZATION) { v } else { 
            return error_internal!(req, "Lack of Authorization logo");
        };
        let auth_str = if let Ok(v) = auth_header.to_str() { v } else {
            return error_internal!(req, "Obtaining Authorization failed");
        };
        if !auth_str.starts_with("Bearer") { // authorization的前缀是 'Bearer'
            return error_internal!(req, "Inferior Authorization character");
        }

        let token = auth_str[6..auth_str.len()].trim();
        match jwt::decode(token) { 
            Ok(_) => { 
                return Either::A(self.service.call(req));
            },
            Err(err) => { 
                let err_msg = format!("Analysis of Authorization failed({})", err);
                return error_internal!(req, err_msg);
            }
        };
        //let auth_data = match jwt::decode(token) {
        //    Ok(v) => { v },
        //    Err(err) => { 
        //        println!("error = {}", err);
        //        return error_internal!(req, "解析Authorization失败");
        //    }
        //};
        //if jwt::verify(&auth_data).is_ok() {
        //    return Either::A(self.service.call(req));
        //}
        //let result = response::JsonError {
        //    code: 401,
        //    message: "获取Token授权失败"
        //};
        //
        //Either::B(ok(req.into_response(
        //    HttpResponse::Unauthorized()
        //        .json(result)
        //        .into_body()
        //)))
    }
}
