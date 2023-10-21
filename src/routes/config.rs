use crate::routes;
use actix_web::web;

// Configures the server routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/repo")
            .service(routes::repo::get_repositories)
            .service(routes::repo::get_repository_hash)
            .service(routes::repo::get_repository_branch),
    )
    .service(web::scope("/object").service(routes::object::get_object_content));
}
