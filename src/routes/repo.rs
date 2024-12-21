use crate::{
    application::AppState,
    repository::repo::Repo,
    utils::responses::{internal_server_error, successful_response},
};
use actix_web::{
    get,
    web::{Data, Path},
    Responder,
};

/// Endpoint to get all repositories on the server
#[get("/all")]
pub async fn get_repositories(state: Data<AppState>) -> impl Responder {
    let mut repos_cache = state.repos_cache.lock().await;

    // Check if repos cache is not empty that way
    // it can just respond with that instead of having
    // to fetch all the repos for every request.
    if !repos_cache.is_empty() {
        return successful_response(&*repos_cache);
    }

    // Try to get all the repositories on my git server
    // and match a response based on the result
    match Repo::get_all().await {
        Ok(repos) => {
            // Update cache of repo's
            *repos_cache = repos.clone();

            successful_response(&repos)
        }
        Err(e) => {
            eprintln!("{e:?}");
            internal_server_error()
        }
    }
}

/// Endpoint to get a specific repository at a specific branch
#[get("/by-branch/{repo}/{branch}")]
pub async fn get_repository_branch(path: Path<(String, String)>) -> impl Responder {
    // Consume path into variables
    let (repo_name, branch) = path.into_inner();

    // Try to get all objects in the repo as well as an optional
    // readme content string if the project has one and match a
    // response based on the result
    match Repo::by_branch(&repo_name, &branch).await {
        Ok(repo) => successful_response(&repo),
        Err(e) => {
            eprintln!("{e:?}");
            internal_server_error()
        }
    }
}

/// Endpoint to get a repository by a specific hash
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
    match Repo::by_hash(&repo_name, &hash).await {
        Ok(repo) => successful_response(&repo),
        Err(e) => {
            eprintln!("{e:?}");
            internal_server_error()
        }
    }
}

/// Endpoint to get a repository's commit log
#[get("/commit-log/{repo}/{branch}")]
pub async fn get_commit_log(
    _state: Data<AppState>,
    path: Path<(String, String)>,
) -> impl Responder {
    // Consume path into variables
    let (repo_name, branch) = path.into_inner();

    // Try to get a repo's commit log for a branch
    // and matching a response based on the result
    match Repo::get_commit_log(&repo_name, &branch).await {
        Ok(commits) => successful_response(&commits),
        Err(e) => {
            eprintln!("{e:?}");
            internal_server_error()
        }
    }
}
