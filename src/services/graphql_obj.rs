use actix_web::{
    get, route,
    web::{self, Data},
    HttpResponse, Responder,
};
use std::{sync::Arc};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use crate::dep::graphql_schema::{create_schema, Schema};
use actix_web_lab::respond::Html;
use log::info;
use crate::dep::config::CONFIG;
use crate::dep::redis_core::{increment};


#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}


#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    increment("graphql");
    let graphql_results = data.execute(&st, &()).await;
    HttpResponse::Ok()
        .append_header(("X-GraphQL-Version", CONFIG.get_string("app.graphql.version").unwrap()))
        .json(graphql_results)
}

pub fn graphql_config(cfg: &mut web::ServiceConfig) {
    info!("Initializing GraphQL");
    let schema = Arc::new(create_schema());
    cfg
        .app_data(Data::from(schema.clone()))
        .service(graphql)
        .service(graphql_playground);
}
