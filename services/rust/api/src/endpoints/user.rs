use log::{
    info,
    debug
};

use actix_web::{
    HttpResponse,
    HttpRequest,
    Responder,
    web
};

use crate::endpoints::{
    ApiResponse,
    ApiResponseStatus
};

use serde::{
    Serialize,
    Deserialize
};


#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("login")
                .route(web::post().to(login_post))
        )
    ;
}


pub async fn login_post(
    request: HttpRequest,
    params: web::Json<LoginRequest>
) -> impl Responder {
    info!("login_post()");

    debug!("user: {:?}", params);

    return HttpResponse::Ok()
        .json(ApiResponse {
            status: ApiResponseStatus::Success,
            message: String::from("login success"),
            data: None
        });
}