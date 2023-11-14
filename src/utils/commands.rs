use crate::error::Error;
use base64::{engine::general_purpose, Engine as _};
use std::env;
use std::process::Command;

/// Try to run git commands on the server
pub fn run_git_command(args: &[&str], is_binary: bool) -> Result<String, Error> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|_| Error::CommandFailed)?;

    if is_binary {
        Ok(general_purpose::STANDARD_NO_PAD.encode(output.stdout))
    } else {
        String::from_utf8(output.stdout).map_err(|_| Error::InvalidUtf8)
    }
}

/// Try to change the current directory
pub fn change_directory(path: &str) -> Result<(), Error> {
    env::set_current_dir(path).map_err(|_| Error::DirectoryChangeError)
}
