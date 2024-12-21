use crate::repository::{
    object::Object,
    repo::{RepoBranchFile, RepoData},
};
use std::sync::Arc;
use std::{collections::HashMap, fmt};
use tokio::sync::Mutex;

/// The filesystem path where public repositories live
pub const REPOS_PATH: &str = "/home/git/repos/public/";

/// A model for the applications state
pub struct AppState {
    /// The maximum size of a payload the application should accept
    pub max_payload: i64,
    /// The `Object` cache which is accessable using a hash key
    pub object_hash_cache: HashMap<String, Object>,
    /// The `Repo` cache which is accessable using a hash key
    pub repo_hash_cache: HashMap<String, Vec<RepoBranchFile>>,
    /// A cache of the `RepoData` (name, description, and last commit)
    pub repos_cache: Arc<Mutex<Vec<RepoData>>>,
}

#[derive(Debug)]
/// The standard error type for this application
pub enum GitApiError {
    /// GitApiError when last() is None
    NoLastElement,
    /// GitApiError involving a failed process command
    CommandFailed,
    /// GitApiError involving a failed cwd change
    DirectoryChangeError,
    /// GitApiError involving invalid UTF-8
    InvalidUtf8,
    /// GitApiError -> std::io::Error
    StdIoError(std::io::Error),
    /// GitApiError for std::string::FromUtf8Error
    FromUtf8(std::string::FromUtf8Error),
}
// Implement display trait for RevereGitApiError
impl fmt::Display for GitApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Think of more meaningful display messages
        write!(f, "error")
    }
}
impl From<std::io::Error> for GitApiError {
    fn from(err: std::io::Error) -> GitApiError {
        GitApiError::StdIoError(err)
    }
}
impl From<std::string::FromUtf8Error> for GitApiError {
    fn from(err: std::string::FromUtf8Error) -> GitApiError {
        GitApiError::FromUtf8(err)
    }
}
