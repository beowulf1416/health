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

use serde::{
    Serialize,
    Deserialize
};
use serde_json::json;

use postgres::{ 
    Db,
    patients::Patients
};

use crate::endpoints::{
    ApiResponse,
    GetObjectRequest,
    GetObjectBySlugRequest,
    api_options
};


#[derive(Debug, Deserialize)]
struct PatientAddRequest {
    id: uuid::Uuid,
    email: String,
    given_name: String,
    family_name: String,
    prefix: String,
    suffix: String
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("add")
                .route(web::method(http::Method::OPTIONS).to(api_options))
                .route(web::get().to(patient_add_get))
                .route(web::post().to(patient_add_post))
        )
    ;
}


async fn patient_add_get() -> impl Responder {
    info!("patient_add_get()");
    return HttpResponse::Ok().body("use POST /add instead");
}


async fn patient_add_post(
    _request: HttpRequest,
    db: web::Data<Db>,
    params: web::Json<PatientAddRequest>

) -> impl Responder {
    info!("patient_add_post()");

    match db.pool().get().await {
        Err(e) => {
            error!("unable to retrieve client: {:?}", e);

            return HttpResponse::InternalServerError()
                .json(ApiResponse {
                    success: false,
                    message: String::from("// TODO patient add error"),
                    data: None
                });
        }
        Ok(client) => {
            let id = params.id.clone();
            let given_name = params.given_name.clone();
            let family_name = params.family_name.clone();
            let prefix = params.prefix.clone();
            let suffix = params.suffix.clone();

            let patients = Patients::new(client);
            match patients.add(
                &id,
                &given_name,
                &family_name,
                &prefix,
                &suffix
            ).await {
                Err(e) => {
                    error!("unable to add patient record: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse {
                            success: false,
                            message: String::from("// TODO patient add error"),
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
