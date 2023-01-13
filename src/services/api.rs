use actix_web::{Responder,HttpResponse,web};
use log::info;

pub fn api_config(cfg: &mut web::ServiceConfig) {
    info!("Initializing API");
    cfg.service(
        web::resource("/api/v1").route(web::get().to(hello))
    );
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}