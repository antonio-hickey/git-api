use std::fs;
use std::env;
use std::process::Command;
use actix_web::{get, http, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RepoData {
    name: String,
    description: String,
    last_commit: LastCommit,
}

#[derive(Serialize, Debug)]
struct LastCommit {
    hash: String,
    date: String,
    msg: String,
}

fn get_last_commit() -> LastCommit {
    let output = Command::new("git")
        .arg("log")
        .output()
        .expect("Failed to execute command");

    let git_log_string = String::from_utf8(output.stdout).expect("Failed to parse output as UTF-8");

    let last_commit_strings: Vec<&str> = git_log_string
        .lines()
        .take(6)
        .collect::<Vec<&str>>();

    let hash = last_commit_strings[0]
        .split_whitespace()
        .nth(1)
        .expect("Failed to extract commit hash")
        .to_string();

    let (date, msg) = if last_commit_strings[2].contains("Date:") {
        (last_commit_strings[2]
            .trim_start_matches("Date:")
            .trim()
            .to_string(),
        last_commit_strings[4]
            .trim_start()
            .to_string()
        )
    } else {
        (last_commit_strings[3]
            .trim_start_matches("Date:")
            .trim()
            .to_string(),

         last_commit_strings[5]
            .trim_start()
            .to_string()
        )
    };



    LastCommit {
        hash,
        date,
        msg,
    }
}

#[get("/")]
async fn get_repositories() -> impl Responder {
    let repos_path = "../../../srv/git/";
    let mut repos: Vec<RepoData> = Vec::new();

    if let Ok(entries) = fs::read_dir(&repos_path) {
        for entry in entries.filter_map(Result::ok) {
            env::set_current_dir(entry.path()).unwrap();
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let name = entry.path()
                    .display()
                    .to_string()
                    .replacen(&repos_path, "", 1)
                    .replacen(".git", "", 1);

                let description = match fs::read_to_string(entry.path().join("description")) {
                    Ok(contents) => contents.trim().to_string(),
                    Err(_) => String::new(),
                };

                let last_commit = get_last_commit();

                repos.push(RepoData {
                    name,
                    description,
                    last_commit,
                });
            }
        }
    } else {
        eprintln!("Failed to read directory");
    }

    let json = serde_json::to_string(&repos).expect("Failed to serialize JSON");
    HttpResponse::Ok().body(json)
}

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
            .service(get_repositories)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
