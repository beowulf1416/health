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
    domain::Domains
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


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(domain_add_get))
                .route(web::post().to(domain_add_post))
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

    let id = params.id.clone();
    let name = params.name.clone();
    let slug = params.slug.clone();

    if let Ok(client) = db.pool().get().await {
        let domains = Domains::new(client);
        match domains.domain_add(
            id,
            name,
            slug
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
    } else {
        return HttpResponse::InternalServerError()
            .json(ApiResponse {
                success: false,
                message: String::from("// TODO domain add error"),
                data: None
            });
    }
}