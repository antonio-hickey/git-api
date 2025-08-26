use crate::application::GitApiError;
use regex::Regex;
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

// Allowed patterns
static REPO_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._-]+$").expect("Failed to compile repo name regex"));
static BRANCH_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9._/-]+$").expect("Failed to compile branch name regex")
});
static HASH_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-fA-F0-9]{6,40}$").expect("Failed to compile hash regex"));

/// Validate and sanitize a repository name.
pub fn validate_repo_name(repo_name: &str) -> Result<&str, GitApiError> {
    if repo_name.is_empty() {
        return Err(GitApiError::InvalidInput(
            "Repository name cannot be empty".to_string(),
        ));
    }

    if repo_name.len() > 100 {
        return Err(GitApiError::InvalidInput(
            "Repository name too long".to_string(),
        ));
    }

    if repo_name.contains("..") || repo_name.contains('/') || repo_name.contains('\\') {
        return Err(GitApiError::InvalidInput(
            "Invalid characters in repository name".to_string(),
        ));
    }

    if !REPO_NAME_REGEX.is_match(repo_name) {
        return Err(GitApiError::InvalidInput(
            "Repository name contains invalid characters".to_string(),
        ));
    }

    Ok(repo_name)
}

/// Validate and sanitize a branch name.
pub fn validate_branch_name(branch_name: &str) -> Result<&str, GitApiError> {
    if branch_name.is_empty() {
        return Err(GitApiError::InvalidInput(
            "Branch name cannot be empty".to_string(),
        ));
    }

    if branch_name.len() > 250 {
        return Err(GitApiError::InvalidInput(
            "Branch name too long".to_string(),
        ));
    }

    if branch_name.contains("..") || branch_name.starts_with('-') || branch_name.contains("//") {
        return Err(GitApiError::InvalidInput(
            "Invalid branch name format".to_string(),
        ));
    }

    if !BRANCH_NAME_REGEX.is_match(branch_name) {
        return Err(GitApiError::InvalidInput(
            "Branch name contains invalid characters".to_string(),
        ));
    }

    Ok(branch_name)
}

/// Validate and sanitize a git hash.
pub fn validate_hash(hash: &str) -> Result<&str, GitApiError> {
    if hash.is_empty() {
        return Err(GitApiError::InvalidInput(
            "Hash cannot be empty".to_string(),
        ));
    }

    if !HASH_REGEX.is_match(hash) {
        return Err(GitApiError::InvalidInput("Invalid hash format".to_string()));
    }

    Ok(hash)
}

/// Validate and construct a safe repository path
pub fn validate_repo_path(base_path: &str, repo_name: &str) -> Result<PathBuf, GitApiError> {
    validate_repo_name(repo_name)?;

    let base = Path::new(base_path);
    let repo_path = base.join(format!("{}.git", repo_name));

    // Ensure the constructed path is still under the base directory
    if let Ok(canonical_repo) = repo_path.canonicalize() {
        if let Ok(canonical_base) = base.canonicalize() {
            if canonical_repo.starts_with(canonical_base) {
                return Ok(repo_path);
            }
        }
    }

    // If canonicalization fails (path doesn't exist yet), check manually
    let normalized_path = normalize_path(&repo_path);
    let normalized_base = normalize_path(base);

    if normalized_path.starts_with(&normalized_base) {
        Ok(repo_path)
    } else {
        Err(GitApiError::InvalidInput(
            "Repository path traversal attempt detected".to_string(),
        ))
    }
}

/// Normalize a path by resolving `..` and `.` components without requiring the path to exist.
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            std::path::Component::Normal(name) => components.push(name),
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::RootDir => {
                components.clear();
                components.push(std::ffi::OsStr::new("/"));
            }
            std::path::Component::CurDir => {}
            std::path::Component::Prefix(_) => {}
        }
    }

    components.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_repo_name() {
        assert!(validate_repo_name("valid-repo").is_ok());
        assert!(validate_repo_name("valid_repo").is_ok());
        assert!(validate_repo_name("valid.repo").is_ok());
        assert!(validate_repo_name("ValidRepo123").is_ok());

        assert!(validate_repo_name("../etc/passwd").is_err());
        assert!(validate_repo_name("repo/with/slashes").is_err());
        assert!(validate_repo_name("repo with spaces").is_err());
        assert!(validate_repo_name("").is_err());
    }

    #[test]
    fn test_validate_branch_name() {
        assert!(validate_branch_name("main").is_ok());
        assert!(validate_branch_name("feature/new-feature").is_ok());
        assert!(validate_branch_name("release-1.0").is_ok());

        assert!(validate_branch_name("../main").is_err());
        assert!(validate_branch_name("-invalid").is_err());
        assert!(validate_branch_name("branch//double").is_err());
        assert!(validate_branch_name("").is_err());
    }

    #[test]
    fn test_validate_hash() {
        assert!(validate_hash("abc123").is_ok());
        assert!(validate_hash("1234567890abcdef").is_ok());
        assert!(validate_hash("a1b2c3d4e5f6789012345678901234567890abcd").is_ok());

        assert!(validate_hash("invalid-hash").is_err());
        assert!(validate_hash("short").is_err());
        assert!(validate_hash("").is_err());
    }

    #[test]
    fn test_validate_repo_path() {
        let base = "/tmp/repos";

        // Valid repo names should work
        assert!(validate_repo_path(base, "valid-repo").is_ok());

        // Path traversal attempts should be blocked
        assert!(validate_repo_path(base, "../etc").is_err());
        assert!(validate_repo_path(base, "repo/../etc").is_err());
    }

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path(Path::new("/a/b/c")), PathBuf::from("/a/b/c"));
        assert_eq!(
            normalize_path(Path::new("/a/b/../c")),
            PathBuf::from("/a/c")
        );
        assert_eq!(
            normalize_path(Path::new("/a/./b/c")),
            PathBuf::from("/a/b/c")
        );
        assert_eq!(
            normalize_path(Path::new("/a/b/../../c")),
            PathBuf::from("/c")
        );
    }
}
