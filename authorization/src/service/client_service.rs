use anyhow::{Ok, Result};

use crate::{
    dto::{request::client_requests::ClientRequest, response::client_responses::ClientResponse},
    repository::client_repository::ClientRepository, entity::client::ClientEntity,
};

pub struct ClientService;

impl ClientService {
    pub async fn save(&self, record: &ClientRequest) -> Result<ClientEntity> {
        let entity = record.into_entity();
        if record.id.is_some() {
            Ok(ClientRepository.update(&entity).await?)
        } else {
            Ok(ClientRepository.insert(&entity).await?)
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<ClientEntity> {
        Ok(ClientRepository.fetch_by_id(id).await?)
    }

    pub async fn get_list(&self) -> Result<Vec<ClientResponse>> {
        Ok(ClientRepository.list().await?)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        Ok(ClientRepository.delete(id).await?)
    }
}
