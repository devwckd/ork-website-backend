use crate::clients::bridge_service::BridgeServiceError;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Bridge {
    pub id: Uuid,
    pub slug: String,
    #[serde(skip_serializing)]
    pub external_id: Uuid,
}

#[derive(Clone, Debug, serde::Deserialize, validator::Validate)]
pub struct CreateBridgeData {
    #[validate(regex(path = "crate::consts::SLUG_REGEX"))]
    pub slug: String,
}

pub type BridgeResult<R> = Result<R, BridgeError>;

#[derive(Debug, thiserror::Error)]
pub enum BridgeError {
    #[error("validation errors: {0}")]
    Validation(#[from] ValidationErrors),
}

impl From<sqlx::Error> for BridgeError {
    fn from(value: sqlx::Error) -> Self {
        todo!()
    }
}

impl From<BridgeServiceError> for BridgeError {
    fn from(value: BridgeServiceError) -> Self {
        todo!()
    }
}

impl IntoResponse for BridgeError {
    fn into_response(self) -> Response {
        todo!()
    }
}
