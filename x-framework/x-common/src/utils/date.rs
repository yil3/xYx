use std::time::SystemTime;
use chrono::ParseError;
use chrono::Local;
use chrono::DateTime;
use chrono::NaiveDateTime;

pub fn format (datetime: DateTime<Local>) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn to_date(datetime: &str) -> Result<NaiveDateTime, ParseError> {
    NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S")
}

pub fn timestamp() -> usize {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

