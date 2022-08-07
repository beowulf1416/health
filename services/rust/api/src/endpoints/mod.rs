pub mod status;
pub mod domain;
pub mod user;

use log::{
    info
};

use actix_web::{
    HttpResponse, 
    Responder 
};

use serde::{
    Serialize,
    Deserialize
};

use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Value>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FilterRequest {
    pub filter: String,
    pub items: i32,
    pub page: i32
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetObjectRequest {
    pub id: uuid::Uuid
}


pub async fn api_options() -> impl Responder {
    return HttpResponse::Ok();
}

