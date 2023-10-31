use std::borrow::Cow;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;
use validator::{HasLen, ValidationError, ValidationErrors};

use crate::domains::error::ErrorResponse;
use crate::domains::session::SessionError;
use crate::domains::user::UserError;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct RegisterData {
    #[validate(custom(function = "validate_name"))]
    pub name: String,
    #[validate(email(message = "invalidEmail"))]
    pub email: String,
    #[validate(length(min = 8, message = "weakPassword"))]
    pub password: String,
}

fn validate_name(name: &str) -> Result<(), ValidationError> {
    let name = name.trim();
    let name = crate::consts::SPACE_REGEX.replace_all(name, " ");

    if name.length() < 2 || name.length() > 255 {
        return Err(ValidationError {
            code: Cow::Owned("trimmed_length".to_string()),
            message: Some(Cow::Owned("invalidNameLength".to_string())),
            params: Default::default(),
        });
    }

    Ok(())
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, validator::Validate)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

pub type AuthResult<R> = Result<R, AuthError>;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("validation errors: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("user not found")]
    UserNotFound,
    #[error("email already in use")]
    EmailAlreadyInUse,
    #[error("wrong password")]
    WrongPassword,
    #[error("invalid session")]
    InvalidSession,
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<UserError> for AuthError {
    fn from(value: UserError) -> Self {
        match value {
            UserError::AlreadyExists => AuthError::EmailAlreadyInUse,
            UserError::Unknown(error) => AuthError::Unknown(error),
            UserError::NotFound => AuthError::UserNotFound,
        }
    }
}

impl From<password_hash::Error> for AuthError {
    fn from(value: password_hash::Error) -> Self {
        match value {
            password_hash::Error::Password => AuthError::WrongPassword,
            _ => AuthError::Unknown(value.to_string()),
        }
    }
}

impl From<SessionError> for AuthError {
    fn from(value: SessionError) -> Self {
        match value {
            SessionError::Invalid => AuthError::InvalidSession,
            SessionError::Unknown(err) => AuthError::Unknown(err),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::Validation(err) => {
                ErrorResponse::of(StatusCode::BAD_REQUEST, err).into_response()
            }
            AuthError::EmailAlreadyInUse => {
                ErrorResponse::of(StatusCode::CONFLICT, self.to_string()).into_response()
            }
            AuthError::UserNotFound => {
                ErrorResponse::of(StatusCode::NOT_FOUND, self.to_string()).into_response()
            }
            AuthError::Unknown(err) => {
                error!("{:?}", err);
                ErrorResponse::of(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
                    .into_response()
            }
            AuthError::WrongPassword => {
                ErrorResponse::of(StatusCode::LOCKED, "wrong password").into_response()
            }
            AuthError::InvalidSession => {
                ErrorResponse::of(StatusCode::FORBIDDEN, "invalid session").into_response()
            }
        }
    }
}
