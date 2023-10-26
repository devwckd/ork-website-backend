use crate::domains::error::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use kube::config::Kubeconfig;
use log::error;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct Region {
    pub id: Uuid,
    pub slug: String,
    #[serde(skip_serializing)]
    pub options: sqlx::types::Json<RegionOptions>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RegionOptions {
    pub kube: Kubeconfig,
}

pub type RegionResult<R> = Result<R, RegionError>;
#[derive(Debug, thiserror::Error)]
pub enum RegionError {
    #[error("region not found")]
    NotFound,
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for RegionError {
    fn from(value: sqlx::Error) -> Self {
        error!("{:?}", value);
        RegionError::Unknown(value.to_string())
    }
}

impl IntoResponse for RegionError {
    fn into_response(self) -> Response {
        match self {
            RegionError::NotFound => {
                ErrorResponse::of(StatusCode::NOT_FOUND, "regionNotFound").into_response()
            }
            RegionError::Unknown(_) => {
                ErrorResponse::of(StatusCode::INTERNAL_SERVER_ERROR, "internalServerError")
                    .into_response()
            }
        }
    }
}
