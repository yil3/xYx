use crate::entity::client::ClientEntity;
use serde::{Deserialize, Serialize};
use x_common::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientRequest {
    pub id: Option<String>,
    pub secret: String,
    pub name: String,
    pub redirect_uri: String,
    pub scope: String,
    pub owner: Option<String>,
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
        }
    }
}
