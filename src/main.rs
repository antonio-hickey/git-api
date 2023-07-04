use actix_web::{http, App, HttpServer};
use actix_cors::Cors;

mod routes;
mod utils;
mod structs;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Leave debug stuff for now
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        let cors_config = Cors::default() 
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors_config)
            .configure(routes::config::configure_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
