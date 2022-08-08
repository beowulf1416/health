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

use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

use postgres::{ 
    Db,
    roles::Roles
};

use crate::endpoints::{
    ApiResponse,
    FilterRequest,
    GetObjectRequest,
    api_options
};


#[derive(Debug, Serialize, Deserialize)]
struct RoleAddUpdateRequest {
    id: uuid::Uuid,
    domain_id: uuid::Uuid,
    name: String,
    slug: String
}


#[derive(Debug, Serialize, Deserialize)]
struct RoleSetActiveRequest {
    id: uuid::Uuid,
    active: bool
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(role_add_get))
                .route(web::post().to(role_add_post))
        )
        .service(
            web::resource("list")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(role_list_get))
                .route(web::post().to(role_list_post))
        )
        .service(
            web::resource("get")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(role_get_get))
                .route(web::post().to(role_get_post))
        )
        .service(
            web::resource("set/active")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(role_set_active_get))
                .route(web::post().to(role_set_active_post))
        )
        .service(
            web::resource("update")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(role_update_get))
                .route(web::post().to(role_update_post))
        )
    ;
}


async fn role_add_get() -> impl Responder {
    info!("role_add_get()");
    return HttpResponse::Ok().body("use POST /role/add instead");
}


async fn role_add_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<RoleAddUpdateRequest>
) -> impl Responder {
    info!("role_add_post()");
    
    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client connection: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO role_add_post error"),
                    data: None
                });
        }
        Ok(client) => {
            let id = params.id.clone();
            let domain_id = params.domain_id.clone();
            let name = params.name.clone();
            let slug = params.slug.clone();

            let roles = Roles::new(client);
            match roles.add(
                &id,
                &domain_id,
                &name,
                &slug
            ).await {
                Err(e) => {
                    error!("unable to add role: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO role_add_post error"),
                            data: None
                        });
                }
                Ok(_) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("// TODO role_add_post success"),
                            data: None
                        });
                }
            }
        }
    }
}


async fn role_list_get() -> impl Responder {
    info!("role_list_get()");
    return HttpResponse::Ok().body("use POST /role/list instead");
}


async fn role_list_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<FilterRequest>
) -> impl Responder {
    info!("role_list_post()");
    
    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client connection: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO role_list_post error"),
                    data: None
                });
        }
        Ok(client) => {
            let filter = params.filter.clone();
            let items = params.items.clone();
            let page = params.page.clone();

            let roles = Roles::new(client);
            match roles.list(
                &filter,
                &items,
                &page
            ).await {
                Err(e) => {
                    error!("unable to list roles: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO role_list_post error"),
                            data: None
                        });
                }
                Ok(roles) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("// TODO role_list_post success"),
                            data: Some(json!({
                                "roles": roles
                            }))
                        });
                }
            }
        }
    }
}


async fn role_get_get() -> impl Responder {
    info!("role_get_get()");
    return HttpResponse::Ok().body("use POST /role/get instead");
}


async fn role_get_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<GetObjectRequest>
) -> impl Responder {
    info!("role_get_post()");
    
    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client connection: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO role_get_post error"),
                    data: None
                });
        }
        Ok(client) => {
            let id = params.id.clone();

            let roles = Roles::new(client);
            match roles.get(
                &id
            ).await {
                Err(e) => {
                    error!("unable to get role: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO role_get_post error"),
                            data: None
                        });
                }
                Ok(role) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("// TODO role_get_post success"),
                            data: Some(json!({
                                "role": role
                            }))
                        });
                }
            }
        }
    }
}



async fn role_set_active_get() -> impl Responder {
    info!("role_set_active_get()");
    return HttpResponse::Ok().body("use POST /role/set/active instead");
}


async fn role_set_active_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<RoleSetActiveRequest>
) -> impl Responder {
    info!("role_set_active_post()");
    
    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client connection: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO role_set_active_post error"),
                    data: None
                });
        }
        Ok(client) => {
            let id = params.id.clone();
            let active = params.active.clone();

            let roles = Roles::new(client);
            match roles.set_active(
                &id,
                &active
            ).await {
                Err(e) => {
                    error!("unable to add role: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO role_set_active_post error"),
                            data: None
                        });
                }
                Ok(_) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("// TODO role_set_active_post success"),
                            data: None
                        });
                }
            }
        }
    }
}


async fn role_update_get() -> impl Responder {
    info!("role_add_get()");
    return HttpResponse::Ok().body("use POST /role/add instead");
}


async fn role_update_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<RoleAddUpdateRequest>
) -> impl Responder {
    info!("role_update_post()");
    
    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client connection: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO role_update_post error"),
                    data: None
                });
        }
        Ok(client) => {
            let id = params.id.clone();
            let domain_id = params.domain_id.clone();
            let name = params.name.clone();
            let slug = params.slug.clone();

            let roles = Roles::new(client);
            match roles.update(
                &id,
                &domain_id,
                &name,
                &slug
            ).await {
                Err(e) => {
                    error!("unable to add role: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO role_update_post error"),
                            data: None
                        });
                }
                Ok(_) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("// TODO role_update_post success"),
                            data: None
                        });
                }
            }
        }
    }
}