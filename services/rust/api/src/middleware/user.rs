use log::{
    debug
};

use std::task::{ Context, Poll };
use std::future::{ ready, Ready };
use std::rc::Rc;
use futures::future::LocalBoxFuture;

use actix_web::{
    HttpMessage,
    error::Error,
    dev::{
        Service, 
        Transform, 
        ServiceRequest, 
        ServiceResponse
    },
    web
};

use http::header::{
    AUTHORIZATION
};

use jwt::JWT;


pub struct User {}

pub struct UserMiddleware<S> {
    service: Rc<S>
}

impl User {
    pub fn new() -> Self {
        return Self {};
    }
}


impl <S, B> Transform<S, ServiceRequest> for User
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    S: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = UserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ready(Ok(UserMiddleware {
            service: Rc::new(service)
        }));
    }
}


impl <S, B> Service<ServiceRequest> for UserMiddleware<S> 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    S: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        return self.service.poll_ready(context);
    }

    fn call(&self, request: ServiceRequest) -> Self::Future {
        debug!("here 1");

        let service = self.service.clone();

        return Box::pin(async move {
            debug!("here 2");

            if let Some(header_value) = request.headers().get(AUTHORIZATION) {
                let token = header_value.to_str().unwrap().replace("Bearer", "").trim().to_owned();
                
                let jwt = request.app_data::<web::Data<JWT>>().unwrap().clone();
                if jwt.validate(&token) {
                    debug!("token valid");
                    request.extensions_mut().insert(crate::models::user::User::new(
                        None,
                        String::from("testvalid@test.com")
                    ));
                } else {
                    debug!("token invalid");
                    request.extensions_mut().insert(crate::models::user::User::new(
                        None,
                        String::from("testinvalid@test.com")
                    ));
                }
            } else {
                debug!("no authorization header found");
            }

            let fut = service.call(request);
            let mut res = fut.await?;
            return Ok(res);
        })
    }
}