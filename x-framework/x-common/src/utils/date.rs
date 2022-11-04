use serde::Deserialize;
use serde::Deserializer;
use serde::Serializer;
use serde_with::{DeserializeAs, SerializeAs};
use time::format_description;
use time::macros::datetime;
use time::macros::offset;
use time::OffsetDateTime;

pub struct DateTimeFormat;

impl SerializeAs<OffsetDateTime> for DateTimeFormat {
    fn serialize_as<S>(date_time: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&date_time.format(&format).map_err(serde::ser::Error::custom)?)
    }
}

impl DeserializeAs<OffsetDateTime> for DateTimeFormat {
    fn deserialize_as<D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer,
    {
        let s = String::deserialize(deserializer)?;
        let format = time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
            .map_err(serde::de::Error::custom)?;
        OffsetDateTime::parse(&s, &format).map_err(serde::de::Error::custom)
    }
}

// pub fn format (datetime: DateTime<Local>) -> String {
//     datetime.format("%Y-%m-%d %H:%M:%S").to_string()
// }

// pub fn to_date(datetime: &str) -> Result<NaiveDateTime, ParseError> {
//     NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S")
// }

pub fn timestamp() -> usize {
    // SystemTime::now()
    //     .duration_since(SystemTime::UNIX_EPOCH)
    //     .unwrap()
    //     .as_secs() as usize
    OffsetDateTime::now_utc().unix_timestamp() as usize
}

pub fn format(datetime: OffsetDateTime) -> anyhow::Result<String> {
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;
    datetime.format(&format).map_err(|e| anyhow::anyhow!(e))
}

pub fn format_zoned(datetime: OffsetDateTime) -> anyhow::Result<String> {
    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
    )?;
    datetime.format(&format).map_err(|e| anyhow::anyhow!(e))
}

pub fn string_now() -> String {
    format(OffsetDateTime::now_utc().to_offset(offset!(+8))).unwrap()
}

pub fn string_to_date(datetime: &str) -> anyhow::Result<OffsetDateTime> {
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;
    OffsetDateTime::parse(datetime, &format).map_err(|e| anyhow::anyhow!(e))
}
