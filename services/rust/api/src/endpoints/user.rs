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

use http::header::AUTHORIZATION;

use serde::{
    Serialize,
    Deserialize
};

use jwt::JWT;

use postgres::{ 
    Db,
    users::Users
};

use crate::endpoints::{
    ApiResponse,
    ApiResponseStatus
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
    jwt: web::Data<JWT>,
    db: web::Data<Db>,
    params: web::Json<LoginRequest>
) -> impl Responder {
    info!("login_post()");

    let email = params.email.clone();
    let pw = params.password.clone();

    if let Ok(client) = db.pool().get().await {
        let users = Users::new(client);
        let authentic = users.authenticate(
            &email,
            &pw
        ).await;
        if authentic {
            if let Ok(token) = jwt.generate(
                &email
            ) {
                // return authorization header
                return HttpResponse::Ok()
                    .append_header((AUTHORIZATION, format!("Bearer {}", token)))
                    .json(ApiResponse {
                        status: ApiResponseStatus::Success,
                        message: String::from("login success"),
                        data: None
                    });
            } else {
                return HttpResponse::Ok()
                    .json(ApiResponse {
                        status: ApiResponseStatus::Fail,
                        message: String::from("login failed"),
                        data: None
                    });
            }
        } else {
            return HttpResponse::Ok()
                .json(ApiResponse {
                    status: ApiResponseStatus::Fail,
                    message: String::from("login failed"),
                    data: None
                });
        }
    } else {
        return HttpResponse::InternalServerError()
            .json(ApiResponse {
                status: ApiResponseStatus::Error,
                message: String::from("login error"),
                data: None
            });
    }
}