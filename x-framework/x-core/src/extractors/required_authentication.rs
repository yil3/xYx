use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::Extension;
use http::header::AUTHORIZATION;
use tracing::error;
use x_common::errors::XError;
use x_common::traits::authen::token::DynToken;


/// Extracts the JWT from the Authorization token header.
pub struct RequiredAuthentication(pub i64);

#[async_trait]
impl<B> FromRequest<B> for RequiredAuthentication
where
    B: Send + Sync,
{
    type Rejection = XError;

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(token_service): Extension<DynToken> = Extension::from_request(request)
            .await
            .map_err(|err| XError::InternalServerErrorWithContext(err.to_string()))?;

        if let Some(authorization_header) = request.headers().get(AUTHORIZATION) {
            let header_value = authorization_header.to_str().map_err(|_| XError::Unauthorized)?;

            if !header_value.contains("Token") {
                error!("request does not contain valid 'Token' prefix for authorization");
                return Err(XError::Unauthorized);
            }

            let tokenized_value: Vec<_> = header_value.split(' ').collect();

            if tokenized_value.len() != 2 || tokenized_value.get(1).is_none() {
                error!("request does not contain a valid token");
                return Err(XError::Unauthorized);
            }

            let token_value = tokenized_value.into_iter().nth(1).unwrap();
            let user_id = token_service
                .get_user_id_from_token(String::from(token_value))
                .map_err(|err| {
                    error!("could not validate user ID from token: {:?}", err);
                    XError::Unauthorized
                })?;

            Ok(RequiredAuthentication(user_id))
        } else {
            Err(XError::Unauthorized)
        }
    }
}
