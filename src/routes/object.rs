use crate::{
    application::AppState,
    repository::object::Object,
    utils::{
        responses::{internal_server_error, successful_response},
        validation::{validate_hash, validate_repo_name},
    },
};
use actix_web::{get, web, web::Data, Responder};

/// Endpoint to get a objects content
#[get("/by-hash/{repo}/{hash}")]
pub async fn get_object_content(
    state: Data<AppState>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    // Consume path into variables
    let (repo_name, hash) = path.into_inner();

    // Validate user inputs
    let repo_name = match validate_repo_name(&repo_name) {
        Ok(name) => name,
        Err(e) => {
            eprintln!("Validation error: {e:?}");
            return internal_server_error();
        }
    };
    let hash = match validate_hash(&hash) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Validation error: {e:?}");
            return internal_server_error();
        }
    };

    // Derive a key for the hash cache and try to fetch content
    // from cache before trying to process the request
    let hash_cache_key = format!("{}{}", repo_name, hash);
    if let Some(cached_content) = &state.object_hash_cache.get(&hash_cache_key) {
        return successful_response(&cached_content);
    }

    // Try to get an specific objects content in a repo by a given hash
    // and matching a response based on the result
    match Object::by_hash(repo_name, hash).await {
        Ok(object_content) => successful_response(&object_content),
        Err(_) => internal_server_error(),
    }
}
