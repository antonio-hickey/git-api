use crate::{
    error::Error,
    structs::ObjectContent,
    utils::commands::{change_directory, run_git_command},
};
use base64::{engine::general_purpose, Engine as _};
use std::process::{Command, Stdio};

/// Try to get a filename from a hash
fn derive_filename_from_hash(hash: &str) -> Result<String, Error> {
    let hash_list = Command::new("git")
        .args(["rev-list", "--objects", "--all"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute git rev-list command");

    Ok(String::from_utf8(
        Command::new("grep")
            .arg(&hash)
            .stdin(
                hash_list
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
    .to_string())
}

/// Try to get a specific objects content in a repo by a given hash
pub async fn by_hash(repo: &str, hash: &str) -> Result<ObjectContent, Error> {
    // Navigate to the repos path
    let repos_path = format!("/home/git/srv/git/{}.git", repo);
    change_directory(&repos_path)?;

    let name = derive_filename_from_hash(&hash)?;
    let mut ext = name
        .split(".")
        .collect::<Vec<&str>>()
        .last()
        .ok_or(Error::NoLastElement)?
        .trim_end()
        .to_string();
    ext = if ext == name {
        String::from("diff")
    } else {
        ext
    };

    // Handle images by checking extension and if
    // it's an image extension than convert the content
    // to bytes and then base64 encode the bytes.
    let image_exts = vec!["png", "jpg", "jpeg"];
    let content = if image_exts.contains(&ext.as_str()) {
        run_git_command(&["show", "-p", &hash], true)?
    } else {
        run_git_command(&["show", "-p", &hash], false)?
    };

    // Collect the objects data
    let size = run_git_command(&["cat-file", "-s", &hash], false)?;

    Ok(ObjectContent {
        name,
        content,
        size,
        ext,
    })
}
