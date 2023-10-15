use std::fs;
use std::env;
use std::process::{Command, Stdio};

use actix_web::{web::{Data, Path}, get,  HttpResponse, Responder};
use serde::Serialize;

use crate::structs::{LastCommit, RepoData, AppState};
use crate::utils::{
    commits::get_last_commit,
    dates::parse_string_to_date,
};


#[get("/")]
pub async fn get_repositories() -> impl Responder {
    /* Get all repos on server */
    let repos_path = "/home/git/srv/git/";
    let _ = env::set_current_dir(&repos_path);

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

                let last_commit = get_last_commit(None);

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

    repos.sort_by_key(
        |a| parse_string_to_date(&a.last_commit.date)
    );
    repos.reverse();

    let json = serde_json::to_string(&repos).expect("Failed to serialize JSON");
    HttpResponse::Ok().body(json)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RepoBranchFile {
    name: String,
    file_type: String,
    object_hash: String,
    last_commit: LastCommit,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Repo {
    objects: Vec<RepoBranchFile>,
    read_me: Option<String>,
}

#[get("/by-branch/{repo}/{branch}")]
pub async fn get_repository_branch(path: Path<(String, String)>) -> impl Responder {
    /* Get a specific repo by branch */
    let repo = &path.0;
    let branch = &path.1;

    // Initiate a mutable variable to store README.md content
    // as a string if the repo has one else default to None.
    let mut read_me: Option<String> = None;

    let path = format!("/home/git/srv/git/{}.git/", &repo);
    let _ = env::set_current_dir(&path);
    let git_branch_tree = String::from_utf8(Command::new("git").args(["ls-tree", branch]).output().unwrap().stdout).expect("Invalid UTF-8");
    let objects: Vec<RepoBranchFile> = git_branch_tree.lines().map(|x| {
        let y = x.split(" ").collect::<Vec<&str>>();
        let name = y[2].split("\t").collect::<Vec<&str>>()[1].to_string();

        // Checks if the objects name is README.md
        // and if so updates `read_me` to a string
        // of the README's content.
        if &name == "README.md" {
            let content = String::from_utf8(
                Command::new("cat").args(["README.md"]).output().unwrap().stdout
            ).unwrap();
            read_me = Some(content)
        };

        RepoBranchFile {
            name,
            file_type: y[1].to_string(),
            object_hash: y[2].split("\t").collect::<Vec<&str>>()[0].to_string(),
            last_commit: get_last_commit(match y[1] == "tree" {
                true => None,
                false => Some(y[2].split("\t").collect::<Vec<&str>>()[1]),
            }),
        }
    }).collect::<Vec<RepoBranchFile>>();

    // Build a json string of our output struct `Repo`
    let json = serde_json::to_string(&Repo {
        objects,
        read_me,
    })
    .expect("Failed to serialize JSON");

    HttpResponse::Ok().body(json)
}

#[get("/by-hash/{repo}/{hash}")]
pub async fn get_repository_hash(state: Data<AppState>, path: Path<(String, String)>) -> impl Responder {
    /* Get a specific repo by hash */
    let repo = &path.0;
    let hash = &path.1;
    let hash_cache_key = format!("{}{}", &repo, &hash);

    if state.repo_hash_cache.contains_key(&hash_cache_key) {
        let repo_content = state.repo_hash_cache.get(&hash_cache_key);
        let json = serde_json::to_string(&repo_content).expect("Failed to serialize to JSON");
        HttpResponse::Ok().body(json) 
    } else {
        let path = format!("/home/git/srv/git/{}.git/", &repo);
        let _ = env::set_current_dir(&path);
        let git_command = Command::new("git")
            .args(["rev-list", "--objects", "--all"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute git rev-list command");

        let parent = String::from_utf8(Command::new("grep")
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

        let git_branch_tree = String::from_utf8(Command::new("git").args(["ls-tree", hash]).output().unwrap().stdout).expect("Invalid UTF-8");
        let output: Vec<RepoBranchFile> = git_branch_tree.lines().map(|x| {
            let y = x.split(" ").collect::<Vec<&str>>();
            let z = format!("{}/{}", parent, y[2].split("\t").collect::<Vec<&str>>()[1]);
            RepoBranchFile {
                name: y[2].split("\t").collect::<Vec<&str>>()[1].to_string(),
                file_type: y[1].to_string(),
                object_hash: y[2].split("\t").collect::<Vec<&str>>()[0].to_string(),
                last_commit: get_last_commit(match y[1] == "tree" {
                    true => None,
                    false => Some(&z),
                }),
            }
        }).collect::<Vec<RepoBranchFile>>();

        let json = serde_json::to_string(&output).expect("Failed to serialize JSON");
        HttpResponse::Ok().body(json)
    }
}
