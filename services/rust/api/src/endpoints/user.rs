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
    // ApiResponseStatus,
    api_options
};


#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserAddRequest {
    pub email: String,
    pub password: String,
    pub given_name: String,
    pub family_name: String,
    pub prefix: String,
    pub suffix: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("authenticate")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(authenticate_get))
                .route(web::post().to(authenticate_post))
        )
    ;
}


async fn authenticate_get() -> impl Responder {
    info!("authenticate_get()");
    return HttpResponse::Ok().body("use POST /authenticate instead");
}


async fn authenticate_post(
    request: HttpRequest,
    jwt: web::Data<JWT>,
    db: web::Data<Db>,
    params: web::Json<LoginRequest>
) -> impl Responder {
    info!("authenticate_post()");

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
                        success: true,
                        message: String::from("login success"),
                        data: None
                    });
            } else {
                return HttpResponse::Ok()
                    .json(ApiResponse {
                        success: false,
                        message: String::from("login failed"),
                        data: None
                    });
            }
        } else {
            return HttpResponse::Ok()
                .json(ApiResponse {
                    success: false,
                    message: String::from("login failed"),
                    data: None
                });
        }
    } else {
        return HttpResponse::InternalServerError()
            .json(ApiResponse {
                success: false,
                message: String::from("login error"),
                data: None
            });
    }
}


async fn user_add_get() -> impl Responder {
    info!("user_add_get()");
    return HttpResponse::Ok().body("use POST /add instead");
}

async fn user_add_post(
    request: HttpRequest,
    jwt: web::Data<JWT>,
    db: web::Data<Db>,
    params: web::Json<UserAddRequest>
) -> impl Responder {
    info!("user_add_post()");

    let email = params.email.clone();
    let pw = params.password.clone();
    let given_name = params.given_name.clone();
    let family_name = params.family_name.clone();
    let prefix = params.prefix.clone();
    let suffix = params.suffix.clone();
    
    return HttpResponse::Ok().body("// TODO user_add_post()");
}

