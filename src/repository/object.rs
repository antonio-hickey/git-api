use crate::{
    application::{GitApiError, REPOS_PATH},
    utils::commands::{change_directory, get_filename_from_hash, run_git_command},
};
use serde::Serialize;

#[derive(Serialize, Debug)]
/// Model representing an object in a repository
/// example: code file, image file, etc.
pub struct Object {
    pub name: String,
    pub content: String,
    pub size: String,
    pub ext: String,
}
impl Object {
    /// Try to get a specific objects content in a repo by a given hash
    pub async fn by_hash(repo: &str, hash: &str) -> Result<Object, GitApiError> {
        // Start at the filesystem's path for the given repository
        let repos_path = format!("{}{}.git", REPOS_PATH, repo);
        change_directory(&repos_path)?;

        // Parse out the filename and extension
        let name = get_filename_from_hash(hash)?;
        let mut ext = name
            .split('.')
            .collect::<Vec<&str>>()
            .last()
            .ok_or(GitApiError::NoLastElement)?
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
        let image_exts = ["png", "jpg", "jpeg"];
        let content = if image_exts.contains(&ext.as_str()) {
            run_git_command(&["show", "-p", hash], true)?
        } else {
            run_git_command(&["show", "-p", hash], false)?
        };

        // Collect the objects data
        let size = run_git_command(&["cat-file", "-s", hash], false)?;

        Ok(Object {
            name,
            content,
            size,
            ext,
        })
    }
}
