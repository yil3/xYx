use std::sync::Arc;

use async_trait::async_trait;

#[async_trait]
pub trait IntoExtractor {
    async fn into_extractor(&self) -> Arc<Self>;
}
