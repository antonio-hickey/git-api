use crate::utils::dates::parse_date_to_string;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
/// A model representing a commit in a repository
pub struct Commit {
    pub hash: String,
    pub author: String,
    pub author_email: String,
    pub date: String,
    pub msg: String,
}
impl From<&str> for Commit {
    /// Parse out [`Commit`]'s from a string slice.
    ///
    /// NOTE: The input is ALWAYS a log entry from the
    /// output of the `git log` command to fetch commits.
    fn from(log_entry: &str) -> Self {
        let mut parts = log_entry.split('\x1f');
        let hash = parts.next().unwrap_or("")[..6].trim().to_string();
        let author = parts.next().unwrap_or("").trim().to_string();
        let author_email = parts.next().unwrap_or("").trim().to_string();
        let date = parse_date_to_string(parts.next().unwrap_or("").trim().to_string());
        let msg = parts
            .skip(1)
            .collect::<Vec<_>>()
            .join("\x1f")
            .lines()
            .take(1)
            .collect::<String>()
            .trim()
            .to_string();

        Commit {
            hash,
            author,
            author_email,
            date,
            msg,
        }
    }
}
