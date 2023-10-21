use crate::structs::Commit;
use crate::utils::dates::parse_date_to_string;

fn parse_author(input: &str) -> Option<(String, String)> {
    // Find the position of the colon and the angle brackets
    let colon_pos = input.find(": ").unwrap_or(0) + 2;
    let open_angle = input.find('<').unwrap_or(0);
    let close_angle = input.find('>').unwrap_or(0);

    // Slice the string to get the name and email
    let name = &input[colon_pos..open_angle].trim();
    let email = &input[(open_angle + 1)..close_angle].trim();

    Some((name.to_string(), email.to_string()))
}

pub fn parse_commit_log_block(block: Vec<&str>) -> Commit {
    let hash = block[0]
        .split_whitespace()
        .nth(1)
        .expect("Failed to extract commit hash")
        .chars()
        .take(6)
        .collect::<String>();

    let (author, author_email) = parse_author(block[1]).expect("failed to parse author");

    let (date, msg) = if block[2].contains("Date:") {
        (
            parse_date_to_string(block[2].trim_start_matches("Date:").trim().to_string()),
            block[4].trim_start().to_string(),
        )
    } else {
        (
            parse_date_to_string(block[3].trim_start_matches("Date:").trim().to_string()),
            block[5].trim_start().to_string(),
        )
    };

    Commit {
        hash,
        date,
        msg,
        author,
        author_email,
    }
}
