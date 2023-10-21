use crate::{
    repository::repo,
    structs::{AppState, Repo},
    utils::responses::{internal_server_error, successful_response},
};
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};

/// Endpoint to get all repositories on the server
#[get("/")]
pub async fn get_repositories() -> impl Responder {
    // Try to get all the repositories on my git server
    // and match a response based on the result
    match repo::all().await {
        Ok(repos) => successful_response(&repos),
        Err(_) => internal_server_error(),
    }
}

/* Get a specific repo by branch */
#[get("/by-branch/{repo}/{branch}")]
pub async fn get_repository_branch(path: Path<(String, String)>) -> impl Responder {
    // Consume path into variables
    let (repo_name, branch) = path.into_inner();

    // Try to get all objects in the repo as well as an optional
    // readme content string if the project has one and match a
    // response based on the result
    match repo::by_branch(&repo_name, &branch).await {
        Ok((objects, read_me)) => successful_response(&Repo { objects, read_me }),
        Err(_) => internal_server_error(),
    }
}

/// Get a repository by a specific hash
#[get("/by-hash/{repo}/{hash}")]
pub async fn get_repository_hash(
    state: Data<AppState>,
    path: Path<(String, String)>,
) -> impl Responder {
    // Extract repo name and hash from url path
    let (repo_name, hash) = path.into_inner();

    // Derive a key for the hash cache and try to fetch content
    // from cache before trying to process the request
    let hash_cache_key = format!("{}{}", &repo_name, &hash);
    if let Some(cached_content) = state.repo_hash_cache.get(&hash_cache_key) {
        return successful_response(&cached_content);
    }

    // Try to get all the objects in the repository by
    // the hash and match a response to the result
    match repo::by_hash(&repo_name, &hash).await {
        Ok(objects) => {
            let data = &Repo {
                objects,
                read_me: None,
            };
            successful_response(data)
        }
        Err(_) => internal_server_error(),
    }
}

/// Endpoint to to repositories commit log
#[get("/commit-log/{repo}/{branch}")]
pub async fn get_commit_log(
    _state: Data<AppState>,
    path: Path<(String, String)>,
) -> impl Responder {
    // Consume path into variables
    let (repo_name, branch) = path.into_inner();

    // Try to get a repo's commit log for a branch
    // and matching a response based on the result
    match repo::get_commit_log(&repo_name, &branch).await {
        Ok(commits) => successful_response(&commits),
        Err(_) => internal_server_error(),
    }
}
