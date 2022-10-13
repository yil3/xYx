use serde::{Serialize, Deserialize};

use crate::entity::client::ClientEntity;
use x_common::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientRequest {
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
}

impl ClientRequest {
    pub fn into_entity(&self) -> ClientEntity {
        ClientEntity {
            id: utils::code::unique_id(),
            secret: String::from(&self.secret),
            name: String::from(&self.name),
            redirect_uri: String::from(&self.redirect_uri),
            scope: String::from(&self.scope),
            owner: None,
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
        }
    }
}
