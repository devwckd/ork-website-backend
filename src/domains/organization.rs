use crate::domains::error::ErrorResponse;
use crate::domains::organization_member::OrganizationMemberError;
use crate::domains::region::RegionError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub slug: String,
    pub region_id: Uuid,
}

#[derive(Clone, Debug, serde::Deserialize, validator::Validate)]
pub struct CreateOrganizationData {
    #[validate(length(min = 4, max = 32), regex = "crate::consts::SLUG_REGEX")]
    pub slug: String,
    #[validate(length(min = 4, max = 32), regex = "crate::consts::SLUG_REGEX")]
    pub region_slug: String,
}

pub type OrganizationResult<R> = Result<R, OrganizationError>;

#[derive(Debug, thiserror::Error)]
pub enum OrganizationError {
    #[error("validation errors: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("organization not found")]
    NotFound,
    #[error("region not found")]
    RegionNotFound,
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for OrganizationError {
    fn from(value: sqlx::Error) -> Self {
        error!("{:?}", value);
        todo!()
    }
}

impl From<RegionError> for OrganizationError {
    fn from(value: RegionError) -> Self {
        match value {
            RegionError::NotFound => OrganizationError::RegionNotFound,
            RegionError::Unknown(err) => OrganizationError::Unknown(err),
        }
    }
}

impl From<OrganizationMemberError> for OrganizationError {
    fn from(value: OrganizationMemberError) -> Self {
        match value {}
    }
}

impl IntoResponse for OrganizationError {
    fn into_response(self) -> Response {
        match self {
            OrganizationError::Validation(errs) => {
                ErrorResponse::of(StatusCode::BAD_REQUEST, errs).into_response()
            }
            OrganizationError::RegionNotFound => {
                ErrorResponse::of(StatusCode::NOT_FOUND, "region not found").into_response()
            }
            OrganizationError::Unknown(_) => {
                ErrorResponse::of(StatusCode::INTERNAL_SERVER_ERROR, "internalServerError")
                    .into_response()
            }
            OrganizationError::NotFound => {
                ErrorResponse::of(StatusCode::NOT_FOUND, "organization not found").into_response()
            }
        }
    }
}
