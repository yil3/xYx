use anyhow::{Ok, Result};
use x_common::model::page::{Page, PageParam};

use crate::{
    vo::client_vo::ClientParam, dto::client_dto::ClientDto, po::client::Client,
    repository::client_repository::ClientRepository,
};

pub struct ClientService;

impl ClientService {
    pub async fn save(&self, record: &ClientParam) -> Result<ClientDto> {
        let entity = record.into_entity();
        if record.id.is_some() {
            Ok(ClientRepository.update(&entity).await?)
        } else {
            Ok(ClientRepository.insert(&entity).await?)
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Client> {
        Ok(ClientRepository.fetch_by_id(id).await?)
    }

    pub async fn get_page(&self, params: &PageParam) -> Result<Page<ClientDto>> {
        Ok(Page::build(
            params.page,
            params.size,
            ClientRepository.fetch_page(params).await?,
        ))
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        Ok(ClientRepository.delete(id).await?)
    }
}
