use crate::application::GitApiError;
use base64::{engine::general_purpose, Engine as _};
use std::{
    env,
    process::{Command, Stdio},
};

/// Try to run git commands on the server
pub fn run_git_command(args: &[&str], is_binary: bool) -> Result<String, GitApiError> {
    // Run the command and store the output
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|_| GitApiError::CommandFailed)?;

    if is_binary {
        // Encode the binary output into base 64 bytes
        Ok(general_purpose::STANDARD_NO_PAD.encode(output.stdout))
    } else {
        String::from_utf8(output.stdout).map_err(|_| GitApiError::InvalidUtf8)
    }
}

/// Try to change the current directory
pub fn change_directory(path: &str) -> Result<(), GitApiError> {
    env::set_current_dir(path).map_err(|_| GitApiError::DirectoryChangeError)
}

/// Try to derive a filename from a hash.
///
/// Runs a git command to get a reversed git list, piped into a
/// grep command to find objects/files that match the given hash.
pub fn get_filename_from_hash(hash: &str) -> Result<String, GitApiError> {
    let git_command = Command::new("git")
        .args(["rev-list", "--objects", "--all"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute git rev-list command");

    Ok(String::from_utf8(
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
    .ok_or(GitApiError::NoLastElement)?
    .trim_end()
    .to_string())
}
