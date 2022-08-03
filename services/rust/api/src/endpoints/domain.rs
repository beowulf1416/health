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

use jwt::JWT;

use postgres::{ 
    Db,
    domains::Domains
};

use crate::endpoints::{
    ApiResponse,
    api_options
};


#[derive(Debug, Serialize, Deserialize)]
struct DomainAddRequest {
    pub id: uuid::Uuid,
    pub name: String,
    pub slug: String
}


#[derive(Debug, Serialize, Deserialize)]
struct DomainListRequest {
    pub filter: String,
    pub items: i32,
    pub page: i32
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(domain_add_get))
                .route(web::post().to(domain_add_post))
        )
        .service(
            web::resource("list")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(domain_list_get))
                .route(web::post().to(domain_list_post))
        )
    ;
}


async fn domain_add_get() -> impl Responder {
    info!("domain_add_get()");
    return HttpResponse::Ok().body("use POST /add instead");
}


async fn domain_add_post(
    request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<DomainAddRequest>

) -> impl Responder {
    info!("domain_add_post()");

    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client: {:?}", e);

            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO domain add error"),
                    data: None
                });
        }
        Ok(client) => {
            let id = params.id.clone();
            let name = params.name.clone();
            let slug = params.slug.clone();

            let domains = Domains::new(client);
            match domains.add(
                &id,
                &name,
                &slug
            ).await {
                Err(e) => {
                    error!("unable to add domain record: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO domain add error"),
                            data: None
                        });
                }
                Ok(_) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("added domain record"),
                            data: None
                        });
                }
            }
        }
    }
}


async fn domain_list_get() -> impl Responder {
    info!("domain_list_get()");
    return HttpResponse::Ok().body("use POST /list instead");
}


async fn domain_list_post(
    request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<DomainListRequest>
) -> impl Responder {
    info!("domain_list_post()");

    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client: {:?}", e);

            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO domain list error"),
                    data: None
                });
        }
        Ok(client) => {
            let filter = params.filter.clone();
            let items = params.items.clone();
            let page = params.page.clone();

            let domains = Domains::new(client);
            match domains.list(
                &filter,
                &items,
                &page
            ).await {
                Err(e) => {
                    error!("unable to list domains: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO domain list error"),
                            data: None
                        });
                }
                Ok(result) => {
                    return HttpResponse::Ok()
                        .json(ApiResponse {
                            success: true,
                            message: String::from("// TODO domain list success"),
                            data: Some(json!({
                                "domains": result
                            }))
                        });
                }
            }
        }
    }
}