use std::borrow::Cow;
use std::{collections::HashMap, fmt::Debug};

use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::{ValidationErrors, ValidationErrorsKind};

use crate::model::response::R;

pub type XResult<T> = Result<T, XError>;

pub type XErrorMap = HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>;

#[derive(Error, Debug)]
pub enum XError {
    #[error("authentication is required to access this resource")]
    Unauthorized,
    #[error("username or password is incorrect")]
    InvalidLoginAttmpt,
    #[error("user does not have privilege to access this resource")]
    Forbidden,
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    ApplicationStartup(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("unexpected error has occurred")]
    InternalServerError,
    #[error("{0}")]
    InternalServerErrorWithContext(String),
    #[error("{0}")]
    ObjectConflict(String),
    #[error("unprocessable request has occurred")]
    UnprocessableEntity { errors: XErrorMap },
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub errors: HashMap<String, Vec<String>>,
}

impl ApiError {
    pub fn new(error: String) -> Self {
        let mut error_map: HashMap<String, Vec<String>> = HashMap::new();
        error_map.insert("message".to_owned(), vec![error]);
        Self { errors: error_map }
    }
}

impl XError {
    /// Maps `validator`'s `ValidationrErrors` to a simple map of property name/error messages structure.
    pub fn unprocessable_entity(errors: ValidationErrors) -> Response {
        let mut validation_errors = XErrorMap::new();

        // roll through the struct errors at the top level
        for (_, error_kind) in errors.into_errors() {
            // structs may contain validators on themselves, roll through first-depth validators
            if let ValidationErrorsKind::Struct(meta) = error_kind {
                // on structs with validation errors, roll through each of the structs properties to build a list of errors
                for (struct_property, struct_error_kind) in meta.into_errors() {
                    if let ValidationErrorsKind::Field(field_meta) = struct_error_kind {
                        for error in field_meta.into_iter() {
                            validation_errors
                                .entry(Cow::from(struct_property))
                                .or_insert_with(Vec::new)
                                .push(error.message.unwrap_or_else(|| {
                                    // required validators contain None for their message, assume a default response
                                    Cow::from(format!("{} is required", struct_property))
                                }));
                        }
                    }
                }
            }
        }

        // let body = Json(json!({
        //     "error": validation_errors,
        // }));

        let body = Json(R::<&str>::error(
            &serde_json::to_string(&validation_errors).unwrap_or("validation error".to_string()),
        ));

        (StatusCode::UNPROCESSABLE_ENTITY, body).into_response()
    }
}

impl IntoResponse for XError {
    fn into_response(self) -> Response {
        if let Self::ValidationError(e) = self {
            return Self::unprocessable_entity(e);
        }

        let (status, error_message) = match self {
            Self::InternalServerErrorWithContext(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
            Self::NotFound(err) => (StatusCode::NOT_FOUND, err),
            Self::ObjectConflict(err) => (StatusCode::CONFLICT, err),
            Self::InvalidLoginAttmpt => (StatusCode::BAD_REQUEST, Self::InvalidLoginAttmpt.to_string()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, Self::Unauthorized.to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("unexpected error occurred"),
            ),
        };

        // I'm not a fan of the error specification, so for the sake of consistency,
        // serialize singular errors as a map of vectors similar to the 422 validation responses
        let body = Json(R::<&str>::error(&error_message));

        (status, body).into_response()
    }
}
