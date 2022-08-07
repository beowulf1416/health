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

use http::header::AUTHORIZATION;

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
struct RoleAddRequest {
    id: uuid::Uuid,
    name: String,
    slug: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(role_add_get))
                .route(web::post().to(role_add_post))
        )
    ;
)


async fn role_add_get() -> impl Responder {
    info!("role_add_get()");
    return HttpResponse::Ok().body("use POST /role/add instead");
}


async fn role_add_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<UserAddRequest>
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
            let roles = Roles::new(client);
            match roles.add(
                &id,
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