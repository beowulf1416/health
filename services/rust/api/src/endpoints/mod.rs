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


// #[derive(Debug, Serialize, Deserialize)]
// pub enum ApiResponseStatus {
//     #[serde(rename="success")]
//     Success,
//     #[serde(rename="fail")]
//     Fail,
//     #[serde(rename="error")]
//     Error
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Value>
}


pub async fn api_options() -> impl Responder {
    return HttpResponse::Ok();
}

