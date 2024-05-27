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
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    if output.status.success() {
        if is_binary {
            // Encode the binary output into base-64 bytes
            Ok(general_purpose::STANDARD_NO_PAD.encode(output.stdout))
        } else {
            Ok(String::from_utf8(output.stdout)?)
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(GitApiError::CommandFailed)
    }
}

/// Try to change the current directory
pub fn change_directory(path: &str) -> Result<(), GitApiError> {
    Ok(env::set_current_dir(path)?)
}

/// Try to derive a filename from a hash.
///
/// Runs a git command to get a reversed git list, piped into a
/// grep command to find objects/files that match the given hash.
pub fn get_filename_from_hash(hash: &str) -> Result<String, GitApiError> {
    let mut git_command = Command::new("git")
        .args(["rev-list", "--objects", "--all"])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut grep_command = Command::new("grep")
        .arg(hash)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(ref mut git_stdout) = git_command.stdout {
        if let Some(ref mut grep_stdin) = grep_command.stdin {
            std::io::copy(git_stdout, grep_stdin)?;
        }
    }

    let grep_output = grep_command.wait_with_output()?;

    git_command.wait()?;

    let grep_output_str = String::from_utf8(grep_output.stdout)?;

    grep_output_str
        .split_whitespace()
        .last()
        .map(|s| s.to_string())
        .ok_or(GitApiError::NoLastElement)
}
