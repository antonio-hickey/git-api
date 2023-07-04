use std::env;
use std::process::{Command, Stdio};

use actix_web::{web, get,  HttpResponse, Responder};
use serde::Serialize;



#[derive(Serialize, Debug)]
struct ObjectContent {
    name: String,
    content: String,
    size: String,
    ext: String,
}

#[get("/by-hash/{repo}/{hash}")]
pub async fn get_object_content(path: web::Path<(String, String)>) -> impl Responder {
    let repo = &path.0;
    let hash = &path.1;

    let repos_path = format!("/home/git/srv/git/{}.git", repo);
    let _ = env::set_current_dir(&repos_path);
    let content = String::from_utf8(
        Command::new("git").args(["show", "-p", &hash]).output().unwrap().stdout
    ).unwrap();
    let size = String::from_utf8(
        Command::new("git").args(["cat-file", "-s", &hash]).output().unwrap().stdout
    ).unwrap(); 
    let git_command = Command::new("git")
        .args(["rev-list", "--objects", "--all"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute git rev-list command");

    let name = String::from_utf8(Command::new("grep")
        .arg(&hash)
        .stdin(git_command.stdout.expect("Failed to retrieve git rev-list output"))
        .output()
        .expect("Failed to execute grep command")
        .stdout
    )
    .unwrap()
    .split(" ")
    .last()
    .unwrap()
    .trim_end()
    .to_string();

    let ext = name.split(".").collect::<Vec<&str>>().last().unwrap().trim_end().to_string();

    let obj_content = ObjectContent {
        name,
        content,
        size,
        ext
    };
    let json = serde_json::to_string(&obj_content).expect("Failed to serialize JSON");
    HttpResponse::Ok().body(json)
}

