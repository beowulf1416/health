use log::{
    error
};

use std::env;
use std::fs;

use actix_web::{ web };


pub fn configure(cfg: &mut web::ServiceConfig) {
    if let Ok(url_db_file) = env::var("URL_DB_FILE") {
        if let Ok(url_db) = fs::read_to_string(&url_db_file) {
            if url_db == "" {
                error!("database connection configuration not set in {}", url_db_file);
            } else {
                let db = postgres::Db::new(url_db.clone());
                cfg.app_data(web::Data::new(db.clone()));
            }
        } else {
            error!("unable to read database connection configuration from {}", url_db_file);
        }
    } else {
        error!("environment variable URL_DB_FILE not set");
    }
}