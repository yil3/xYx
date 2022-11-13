use crate::entity::client::ClientEntity;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use time::OffsetDateTime;
use x_common::model::page::Pageable;
use x_common::utils;
use x_common::utils::date::DateTimeFormat;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientParam {
    pub id: Option<String>,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: Option<String>,
}

#[serde_as]
#[derive(FromRow)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRecord {
    pub id: String,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: Option<String>,
    #[serde_as(as = "DateTimeFormat")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "DateTimeFormat")]
    pub updated_at: OffsetDateTime,
    #[serde(skip)]
    #[sqlx(default)]
    pub total: Option<i64>,
}

impl Pageable for ClientRecord {
    fn total(&self) -> i64 {
        self.total.unwrap_or_default()
    }
}

impl ClientParam {
    pub fn into_entity(&self) -> ClientEntity {
        ClientEntity {
            id: match &self.id {
                Some(id) => id.clone(),
                None => utils::code::unique_id(),
            },
            secret: String::from(&self.secret),
            name: String::from(&self.name),
            redirect_uri: String::from(&self.redirect_uri),
            scope: String::from(&self.scope),
            owner: self.owner.clone(),
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}
