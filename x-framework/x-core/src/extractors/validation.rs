use async_trait::async_trait;
use axum::extract::rejection::FormRejection;
use axum::extract::FromRequest;
use axum::Form;
use http::Request;
use serde::de::DeserializeOwned;
use validator::Validate;
use x_common::errors::XError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidationForm<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidationForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, B, Rejection = FormRejection>,
    B: Send + 'static,
{
    type Rejection = XError;

    async fn from_request(request: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(request, state).await?;
        value.validate()?;
        Ok(ValidationForm(value))
    }
}
