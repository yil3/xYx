use anyhow::{Ok, Result};
use x_common::model::page::{CommonPageRequest, Page};

use crate::{
    dto::client_dto::{ClientRequest, ClientResponse},
    entity::client::ClientEntity,
    repository::client_repository::ClientRepository,
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

    pub async fn get_page(&self, params: &CommonPageRequest) -> Result<Page<ClientResponse>> {
        let list = ClientRepository.fetch_page(params).await?;
        let page = Page::build(list, params.limit(), params.offset());
        Ok(page)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        Ok(ClientRepository.delete(id).await?)
    }
}
