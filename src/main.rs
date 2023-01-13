mod dep;

use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use crate::dep::config::CONFIG;
use log::{info};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let address = CONFIG
        .get_string("app.server.address")
        .unwrap();
    let workers = CONFIG
        .get_int("app.server.workers")
        .unwrap();
    info!("Starting server at {}", address);
    HttpServer::new(|| {
        let prometheus = dep::prometheus::init_prometheus();
        App::new()
            .wrap(prometheus.clone())
            .configure(dep::api::api_config)
            .configure(dep::graphql_obj::graphql_config)
            .service(echo)
    }).workers(workers as usize)
        .bind(address)?
        .run().await
}