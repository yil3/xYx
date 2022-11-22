use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::model::page::Pageable;
use x_common::utils::date::DateTimeFormat;


#[serde_as]
#[derive(FromRow)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientDto {
    pub id: String,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub status: bool,
    pub owner: Option<String>,
    #[serde_as(as = "DateTimeFormat")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "DateTimeFormat")]
    pub updated_at: OffsetDateTime,
    #[serde(skip)]
    #[sqlx(default)]
    pub total: Option<i64>,
}

impl Pageable for ClientDto {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}

