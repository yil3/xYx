use serde::{Deserialize, Serialize, Deserializer, Serializer};
use serde_with::{serde_as, DeserializeAs, SerializeAs};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

struct DateTimeFormat;

impl SerializeAs<OffsetDateTime> for DateTimeFormat {
    fn serialize_as<S>(date_time: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        serializer.serialize_str(&date_time.format(&format).map_err(serde::ser::Error::custom)?)
    }
}

impl<'de> DeserializeAs<'de, OffsetDateTime> for DateTimeFormat {
    fn deserialize_as<D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let format = time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        let s = String::deserialize(deserializer)?;
        OffsetDateTime::parse(&s, &format).map_err(serde::de::Error::custom)
    }
}


#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientResponse {
    pub id: String,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: Option<String>,
    #[serde_as(as = "DateTimeFormat")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Option<Rfc3339>")]
    pub updated_at: Option<OffsetDateTime>,
    pub total: Option<i64>,
}
