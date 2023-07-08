use actix_web::{web, http, App, HttpServer, dev::ServiceRequest, HttpMessage, Error};
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};
use actix_cors::Cors;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

mod routes;
mod utils;
mod structs;

use structs::{AppState, Users, TokenClaims};


pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // grab secret and create Hmac key with it 
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env var must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

    // Verify the token
    let token_string = credentials.token();
    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token!");

    match claims {
        Ok(val) => {
            req.extensions_mut().insert(val);
            Ok(req)
        }
        Err(_) => {
            let config = req.app_data::<bearer::Config>().cloned().unwrap_or_default().scope("");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Leave debug stuff for now
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let app_state = web::Data::new(AppState {
        users: Users::default(),
        max_payload: 262_144,
    });

    HttpServer::new(move || {
        let cors_config = Cors::default() 
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
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
