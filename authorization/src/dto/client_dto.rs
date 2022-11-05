use crate::entity::client::ClientEntity;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;
use x_common::model::page::Pageable;
use x_common::utils;
use x_common::utils::date::DateTimeFormat;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientRequest {
    pub id: Option<String>,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: Option<String>,
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
    #[serde_as(as = "Option<DateTimeFormat>")]
    pub updated_at: Option<OffsetDateTime>,
    pub total: Option<i64>,
}

impl Pageable for ClientResponse {
    fn total(&self) -> i64 {
        self.total.unwrap_or(0)
    }
}

impl ClientRequest {
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
            updated_at: None,
        }
    }
}
