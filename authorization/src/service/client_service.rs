use anyhow::{Ok, Result};

use crate::{
    dto::{request::client_requests::ClientRequest, response::client_responses::ClientResponse},
    repository::client_repository::ClientRepository,
};

pub struct ClientService;

impl ClientService {
    pub async fn save(&self, record: &ClientRequest) -> Result<ClientResponse> {
        let entity = record.into_entity();
        if record.id.is_some() {
            Ok(ClientRepository.update(&entity).await?)
        } else {
            Ok(ClientRepository.insert(&entity).await?)
        }
    }
}
