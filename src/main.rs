mod application;
mod repository;
mod routes;
mod utils;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use application::AppState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Leave debug stuff for now
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Initialize application's state
    let app_state = web::Data::new(AppState {
        max_payload: 262_144,
        object_hash_cache: HashMap::new(),
        repo_hash_cache: HashMap::new(),
        repos_cache: Arc::new(Mutex::new(Vec::new())),
    });

    // Run the http server
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
    .bind(("0.0.0.0", 6969))?
    .run()
    .await
}
