use actix_web::{get, http, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetRepoResp {
    name: String,
    description: String,
    #[serde(rename = "langs")]
    languages: String,
    last_commit: String,
}

#[get("/")]
async fn get_repositories() -> impl Responder {
    let x = vec!(
        GetRepoResp {
            name: String::from("Repo A"),
            description: String::from("This is just a test broo"),
            languages: String::from("Python, Bash"),
            last_commit: String::from("2019-09-11 22:46")
        },
        GetRepoResp {
            name: String::from("Repo B"),
            description: String::from("This is just a test broo"),
            languages: String::from("Rust"),
            last_commit: String::from("2023-06-23 13:28")
        },
        GetRepoResp {
            name: String::from("Repo C"),
            description: String::from("This is just a test broo"),
            languages: String::from("Idek"),
            last_commit: String::from("2022-01-12 13:33")
        },
    );
    let json = serde_json::to_string(&x).unwrap();
    HttpResponse::Ok().body(json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors_config = Cors::default() 
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors_config)
            .service(get_repositories)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
