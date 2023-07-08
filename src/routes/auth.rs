use std::env;
use std::process::{Command, Stdio};

use actix_web::{web::Data, get,  HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use Uuid;
use serde::Serialize;

use crate::structs;
use structs::{AppState, TokenClaims};

#[derive(Serialize)]
struct SignInResp {
    status_code: i32,
    token: String,
}


#[get("/sign-in")]
pub async fn sign_in(state: Data<AppState>, creds: BasicAuth) -> impl Responder {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
        .expect("JWT_SECRET env var must be set!")
        .as_bytes(),
    ).unwrap();
    let key = creds.password();
    let users = &state.users;

    match key {
        None => HttpResponse::Unauthorized().json("Must provide password brooo!"),
        Some(key) => {
            match users.hashmap.get(key) {
                Some(user) => {
                    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET env var must be set!");
                    let claims = TokenClaims { id: Uuid::parse_str(user.id).unwrap() };
                    let token_str = claims.sign_with_key(&jwt_secret).unwrap();

                    HttpResponse::Ok().json(SignInResp {
                        status_code: 200,
                        token: token_str,
                    })
                },
                None => HttpResponse::Unauthorized().json("Must provide password brooo!") 
            }
        }
    }
}
 
