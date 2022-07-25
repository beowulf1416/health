extern crate log;

mod services;
mod middleware;
mod endpoints;

use log::{ 
    info 
};

use actix_web::{ 
    HttpServer, 
    App, 
    web, 
    // HttpResponse, 
    // Responder 
};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting up");

    let server = HttpServer::new(move || {
        App::new()
            // .configure(data::data::configure)
            // .configure(users::jwt::configure)
            // .configure(crate::utils::bus::configure)
            // .wrap(crate::middleware::cache::Cache::new())
            // .wrap(crate::middleware::cors::CORS::new())
            // .wrap(crate::middleware::user::User::new())
            // .service(web::scope("/status").configure(crate::endpoints::status::config))
            // .service(web::scope("/user").configure(crate::endpoints::user::config))
            // .service(web::scope("/tenants").configure(crate::endpoints::tenants::config))
            // .service(web::scope("/admin/tenants").configure(crate::endpoints::tenant::admin::config))
            // .route("/status", web::get().to(crate::endpoints::status_get))
            .configure(crate::services::jwt::configure)
            .configure(crate::services::db::configure)
            .wrap(crate::middleware::cors::CORS::new())
            .service(web::scope("/status").configure(crate::endpoints::status::config))
            .service(web::scope("/user").configure(crate::endpoints::user::config))
    })
    .bind("0.0.0.0:8081")?
    .run();

    info!("Server running at https://0.0.0.0:8081");
    server.await
}
