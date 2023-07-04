use crate::structs::LastCommit;
use crate::utils::dates::parse_date_to_string;
use std::process::Command;


pub fn get_last_commit(file: Option<&str>) -> LastCommit {
    let output = match file {
        Some(file_name) => Command::new("git")
            .args(["log", "--", file_name])
            .output()
            .expect("Failed to execute command"),
        None => Command::new("git")
            .arg("log")
            .output()
            .expect("Failed to execute command") 
    };

    let git_log_string = String::from_utf8(output.stdout).expect("Failed to parse output as UTF-8");

    let last_commit_strings: Vec<&str> = git_log_string
        .lines()
        .take(6)
        .collect::<Vec<&str>>();

    let hash = last_commit_strings[0]
        .split_whitespace()
        .nth(1)
        .expect("Failed to extract commit hash")
        .chars()
        .take(6)
        .collect::<String>();

    let (date, msg) = if last_commit_strings[2].contains("Date:") {
        (parse_date_to_string(last_commit_strings[2]
            .trim_start_matches("Date:")
            .trim()
            .to_string()),
        last_commit_strings[4]
            .trim_start()
            .to_string()
        )
    } else {
        (parse_date_to_string(last_commit_strings[3]
            .trim_start_matches("Date:")
            .trim()
            .to_string()),
         last_commit_strings[5]
            .trim_start()
            .to_string()
        )
    };

    LastCommit {
        hash,
        date,
        msg,
    }
}


