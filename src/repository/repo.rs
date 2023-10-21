use crate::{
    error::Error,
    structs::{RepoBranchFile, RepoData},
    utils::{
        commands::{change_directory, run_git_command},
        commits::get_last_commit,
        dates::parse_string_to_date,
    },
};
use std::{
    fs,
    process::{Command, Stdio},
    result::Result,
};

const REPOS_PATH: &str = "/home/git/srv/git/";

pub async fn all() -> Result<Vec<RepoData>, Error> {
    change_directory(REPOS_PATH)?;

    let mut repos: Vec<RepoData> = Vec::new();

    // Walking the repo
    if let Ok(entries) = fs::read_dir(REPOS_PATH) {
        for entry in entries.filter_map(Result::ok) {
            change_directory(entry.path().to_str().expect("some path string"))?;
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let name = entry
                    .path()
                    .display()
                    .to_string()
                    .replacen(&REPOS_PATH, "", 1)
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

    repos.sort_by_key(|a| parse_string_to_date(&a.last_commit.date));
    repos.reverse();
    Ok(repos)
}

pub async fn by_hash(repo: &str, hash: &str) -> Result<Vec<RepoBranchFile>, Error> {
    let path = format!("/home/git/srv/git/{}.git/", &repo);
    change_directory(&path)?;

    let git_command = Command::new("git")
        .args(["rev-list", "--objects", "--all"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute git rev-list command");

    let parent = String::from_utf8(
        Command::new("grep")
            .arg(&hash)
            .stdin(
                git_command
                    .stdout
                    .expect("Failed to retrieve git rev-list output"),
            )
            .output()
            .expect("Failed to execute grep command")
            .stdout,
    )?
    .split(" ")
    .last()
    .ok_or(Error::NoLastElement)?
    .trim_end()
    .to_string();

    Ok(run_git_command(&["ls-tree", hash])?
        .lines()
        .map(|x| {
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
        })
        .collect::<Vec<RepoBranchFile>>())
}

pub async fn by_branch(
    repo: &str,
    branch: &str,
) -> Result<(Vec<RepoBranchFile>, Option<String>), Error> {
    // Initiate a mutable variable to store README.md content
    // as a string if the repo has one else default to None.
    let mut read_me: Option<String> = None;

    let path = format!("/home/git/srv/git/{}.git/", &repo);
    change_directory(&path)?;

    Ok((
        run_git_command(&["ls-tree", branch])?
            .lines()
            .map(|x| {
                let y = x.split(" ").collect::<Vec<&str>>();
                let name = y[2].split("\t").collect::<Vec<&str>>()[1].to_string();

                // Checks if the objects name is README.md
                // and if so updates `read_me` to a string
                // of the README's content.
                if name == "README.md" {
                    let branch_filename = format!("{}:README.md", &branch);
                    let content = run_git_command(&["show", &branch_filename]).unwrap();
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
            })
            .collect::<Vec<RepoBranchFile>>(),
        read_me,
    ))
}
