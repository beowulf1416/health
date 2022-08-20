use log::{
    info,
    debug,
    error
};

use actix_web::{
    HttpResponse,
    HttpRequest,
    Responder,
    web
};

use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

use postgres::{ 
    Db,
    patients::Patients
};

use crate::endpoints::{
    ApiResponse,
    GetObjectRequest,
    GetObjectBySlugRequest,
    api_options
};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(patient_add_get))
                .route(web::post().to(patient_add_post))
        )
    ;
}