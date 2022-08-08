
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::{BoxError, Json};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::errors::XError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidationExtractor<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidationExtractor<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = XError;

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(request).await?;
        value.validate()?;
        Ok(ValidationExtractor(value))
    }
}
