use log::{
    info,
    // debug,
    error
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
use serde_json::json;

use jwt::JWT;

use postgres::{ 
    Db,
    users::Users
};

use crate::endpoints::{
    ApiResponse,
    FilterRequest,
    GetObjectRequest,
    api_options
};


#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserAddRequest {
    pub id: uuid::Uuid,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub prefix: String,
    pub suffix: String
}


#[derive(Debug, Serialize, Deserialize)]
struct UserSetActiveRequest {
    pub id: uuid::Uuid,
    pub active: bool
}


#[derive(Debug, Serialize, Deserialize)]
struct UserSetPasswordRequest {
    pub id: uuid::Uuid,
    pub password: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("authenticate")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(authenticate_get))
                .route(web::post().to(authenticate_post))
        )
        .service(
            web::resource("current")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(current_get))
                .route(web::post().to(current_post))
        )
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(user_add_get))
                .route(web::post().to(user_add_post))
        )
        .service(
            web::resource("set/password")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(user_set_password_get))
                .route(web::post().to(user_set_password_post))
        )
        .service(
            web::resource("set/active")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(user_set_active_get))
                .route(web::post().to(user_set_active_post))
        )
        .service(
            web::resource("list")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(user_list_get))
                .route(web::post().to(user_list_post))
        )
        .service(
            web::resource("get")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(user_get_get))
                .route(web::post().to(user_get_post))
        )
    ;
}


async fn authenticate_get() -> impl Responder {
    info!("authenticate_get()");
    return HttpResponse::Ok().body("use POST /authenticate instead");
}


async fn authenticate_post(
    _request: HttpRequest,
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


async fn current_get() -> impl Responder {
    info!("current_get()");
    return HttpResponse::Ok().body("use POST /current instead");
}


async fn current_post(
    _request: HttpRequest,
    jwt: web::Data<JWT>,
    db: web::Data<Db>
) -> impl Responder {
    info!("current_post()");

    return HttpResponse::Ok()
        .json(ApiResponse {
            success: true,
            message: String::from("here"),
            data: None
        });
}


async fn user_add_get() -> impl Responder {
    info!("user_add_get()");
    return HttpResponse::Ok().body("use POST /user/add instead");
}


async fn user_add_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<UserAddRequest>
) -> impl Responder {
    info!("user_add_post()");
    
    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client connection: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO user_add_post error"),
                    data: None
                });
        }
        Ok(client) => {
            let id = params.id.clone();
            let email = params.email.clone();
            // let pw = params.password.clone();
            let given_name = params.given_name.clone();
            let family_name = params.family_name.clone();
            let prefix = params.prefix.clone();
            let suffix = params.suffix.clone();

            let users = Users::new(client);
            match users.add(
                &id,
                &email,
                &given_name,
                &family_name,
                &prefix,
                &suffix
            ).await {
                Err(e) => {
                    error!("unable to add user: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO user_add_post error"),
                            data: None
                        });
                }
                Ok(_) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("// TODO user_add_post success"),
                            data: None
                        });
                }
            }
        }
    }
}


async fn user_set_password_get() -> impl Responder {
    info!("user_set_password_get()");
    return HttpResponse::Ok().body("use POST /set/password instead");
}


async fn user_set_password_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<UserSetPasswordRequest>
) -> impl Responder {
    info!("user_set_password_post()");
    
    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client connection: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO user_set_password_post error"),
                    data: None
                });
        }
        Ok(client) => {
            let id = params.id.clone();
            let pw = params.password.clone();

            let users = Users::new(client);
            match users.set_password(
                &id,
                &pw
            ).await {
                Err(e) => {
                    error!("unable to set user password: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO user_set_password_post error"),
                            data: None
                        });
                }
                Ok(_) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("// TODO user_set_password_post success"),
                            data: None
                        });
                }
            }
        }
    }
}


async fn user_set_active_get() -> impl Responder {
    info!("user_set_active_get()");
    return HttpResponse::Ok().body("use POST /set/active instead");
}


async fn user_set_active_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<UserSetActiveRequest>
) -> impl Responder {
    info!("user_set_active_post()");
    
    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client connection: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO user_set_active_post error"),
                    data: None
                });
        }
        Ok(client) => {
            let id = params.id.clone();
            let active = params.active.clone();

            let users = Users::new(client);
            match users.set_active(
                &id,
                &active
            ).await {
                Err(e) => {
                    error!("unable to set user password: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO user_set_active_post error"),
                            data: None
                        });
                }
                Ok(_) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("// TODO user_set_active_post success"),
                            data: None
                        });
                }
            }
        }
    }
}


async fn user_list_get() -> impl Responder {
    info!("user_list_get()");
    return HttpResponse::Ok().body("use POST /list instead");
}


async fn user_list_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<FilterRequest>
) -> impl Responder {
    info!("user_list_post()");
    
    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client connection: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO user_list_post error"),
                    data: None
                });
        }
        Ok(client) => {
            let filter = params.filter.clone();
            let items = params.items.clone();
            let page = params.page.clone();

            let users = Users::new(client);
            match users.list(
                &filter,
                &items,
                &page
            ).await {
                Err(e) => {
                    error!("unable to list users: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO user_list_post error"),
                            data: None
                        });
                }
                Ok(users) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("// TODO user_list_post success"),
                            data: Some(json!({
                                "users": users
                            }))
                        });
                }
            }
        }
    }
}



async fn user_get_get() -> impl Responder {
    info!("user_get_get()");
    return HttpResponse::Ok().body("use POST /get instead");
}


async fn user_get_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<GetObjectRequest>
) -> impl Responder {
    info!("user_get_post()");
    
    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client connection: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO user_get_post error"),
                    data: None
                });
        }
        Ok(client) => {
            let id = params.id.clone();

            let users = Users::new(client);
            match users.get(
                &id
            ).await {
                Err(e) => {
                    error!("unable to user: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO user_get_post error"),
                            data: None
                        });
                }
                Ok(user) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("// TODO user_get_post success"),
                            data: Some(json!({
                                "user": user
                            }))
                        });
                }
            }
        }
    }
}