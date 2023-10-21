mod error;
mod repository;
mod routes;
mod structs;
mod utils;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use std::collections::HashMap;
use structs::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Leave debug stuff for now
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let app_state = web::Data::new(AppState {
        max_payload: 262_144,
        object_hash_cache: HashMap::new(),
        repo_hash_cache: HashMap::new(),
    });

    HttpServer::new(move || {
        let cors_config = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors_config)
            .configure(routes::config::configure_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
