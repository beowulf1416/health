use log::{
    error
};

use std::env;
use std::fs;

use actix_web::{ web };


pub fn configure(cfg: &mut web::ServiceConfig) {
    if let Ok(secret_file) = env::var("JWT_SECRET_FILE") {
        if let Ok(secret) = fs::read_to_string(secret_file) {
            let jwt = token::JWT::new(String::from(secret.trim()));
            cfg.app_data(web::Data::new(jwt.clone())); 
        } else {
            error!("unable to read JWT secret");
        }
    } else {
        error!("environment variable JWT_SECRET_FILE not set");
    }
}