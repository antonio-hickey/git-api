use actix_web::HttpResponse;
use serde::Serialize;

/// Serialize object to JSON and create a HttpResponse
pub fn successful_response<T: Serialize>(obj: &T) -> HttpResponse {
    match serde_json::to_string(obj) {
        Ok(json) => HttpResponse::Ok().body(json),
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize to JSON"),
    }
}

pub fn internal_server_error() -> HttpResponse {
    HttpResponse::InternalServerError().body("Internal Server Error")
}
