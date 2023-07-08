use actix_web::web;
use actix_web_httpauth::
    middleware::HttpAuthentication;

use crate::routes;
use crate::validator;

// Configures the server routes.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let bearer_middleware = HttpAuthentication::bearer(validator);

    cfg
        .service(
            web::scope("/repo")
                .wrap(bearer_middleware.clone())
                .service(routes::repo::get_repositories)
                .service(routes::repo::get_repository_hash)
                .service(routes::repo::get_repository_branch)
        )
        .service(
            web::scope("/object")
                .wrap(bearer_middleware.clone())
                .service(routes::object::get_object_content)
        )
        .service(
            web::scope("/user")
                .service(routes::auth::sign_in)
        );
}
