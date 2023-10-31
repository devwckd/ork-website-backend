use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;
use uuid::Uuid;
use validator::ValidationErrors;

use crate::clients::bridge_service::BridgeServiceError;
use crate::domains::error::ErrorResponse;
use crate::utils::handle_sqlx_unique;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct Bridge {
    pub id: Uuid,
    pub slug: String,
    #[serde(skip_serializing)]
    pub bs_namespace_id: Uuid,
}

#[derive(Clone, Debug, serde::Deserialize, validator::Validate)]
pub struct CreateBridgeData {
    #[validate(regex(path = "crate::consts::SLUG_REGEX"))]
    pub slug: String,
}

pub type BridgeResult<R> = Result<R, BridgeError>;

#[derive(Debug, thiserror::Error)]
pub enum BridgeError {
    #[error("bridge not found")]
    NotFound,
    #[error("bridge already exists")]
    AlreadyExists,
    #[error("validation errors: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for BridgeError {
    fn from(value: sqlx::Error) -> Self {
        handle_sqlx_unique(
            value,
            "unique_bridge_slug_per_organization",
            |_| BridgeError::AlreadyExists,
            BridgeError::Unknown,
        )
    }
}

impl From<BridgeServiceError> for BridgeError {
    fn from(value: BridgeServiceError) -> Self {
        match value {
            BridgeServiceError::NamespaceAlreadyExists => BridgeError::Unknown(value.to_string()), // this should not happen
            BridgeServiceError::Unknown(err) => BridgeError::Unknown(err),
        }
    }
}

impl IntoResponse for BridgeError {
    fn into_response(self) -> Response {
        match self {
            BridgeError::NotFound => {
                ErrorResponse::of(StatusCode::NOT_FOUND, "bridge not found").into_response()
            }
            BridgeError::Validation(err) => {
                ErrorResponse::of(StatusCode::BAD_REQUEST, err).into_response()
            }
            BridgeError::Unknown(err) => {
                error!("{}", err);
                ErrorResponse::of(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
                    .into_response()
            }
            BridgeError::AlreadyExists => {
                ErrorResponse::of(StatusCode::CONFLICT, "bridge already exists").into_response()
            }
        }
    }
}
