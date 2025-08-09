use chrono::{DateTime, FixedOffset, TimeZone, Utc};

/// Parse a datetime string into my desired date format (mm/dd/yyyy HH:MM)
pub fn parse_date_to_string(date: String) -> String {
    if let Ok(datetime) = DateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S%:z") {
        let fixed_datetime: DateTime<FixedOffset> = DateTime::from(datetime);

        fixed_datetime.format("%m/%d/%Y %H:%M").to_string()
    } else {
        let datetime = DateTime::parse_from_str(&date, "%a %b %e %H:%M:%S %Y %z")
            .expect("Failed to parse datetime");

        let fixed_datetime: DateTime<FixedOffset> = DateTime::from(datetime);
        fixed_datetime.format("%m/%d/%Y %H:%M").to_string()
    }
}

/// Parse a string into a `DateTime<Utc>`
pub fn parse_string_to_date(date: &str) -> DateTime<Utc> {
    let format = "%m/%d/%Y %H:%M";

    Utc.datetime_from_str(date, format).unwrap()
}
