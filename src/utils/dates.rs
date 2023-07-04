use chrono::{DateTime, FixedOffset, TimeZone, Utc};

pub fn parse_date_to_string(date: String) ->String {
    let x = DateTime::parse_from_str(&date, 
        "%a %b %e %H:%M:%S %Y %z"
    ).expect("Failed to parse datetime");
    let y: DateTime<FixedOffset> = DateTime::from(x);

    y.format("%m/%d/%Y %H:%M").to_string()
}

pub fn parse_string_to_date(date: &str) ->DateTime<Utc> {
    let format = "%m/%d/%Y %H:%M";

    Utc.datetime_from_str(date, format).unwrap()
}
