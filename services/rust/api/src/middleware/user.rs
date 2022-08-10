use log::{
    debug
};

use std::task::{ Context, Poll };
use std::future::{ ready, Ready };
use futures::future::LocalBoxFuture;

use actix_web::{
    error::Error,
    dev::{
        Service, 
        Transform, 
        ServiceRequest, 
        ServiceResponse
    }
};

use http::header::{
    AUTHORIZATION
};


pub struct User {}

pub struct UserMiddleware<S> {
    service: S
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
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = UserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ready(Ok(UserMiddleware {
            service
        }));
    }
}


impl <S, B> Service<ServiceRequest> for UserMiddleware<S> 
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        return self.service.poll_ready(context);
    }

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let fut = self.service.call(request);

        return Box::pin(async move {
            debug!("here 1");

            

            let mut res = fut.await?;
            return Ok(res);
        })
    }
}