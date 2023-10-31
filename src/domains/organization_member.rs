use crate::domains::error::ErrorResponse;
use crate::utils::handle_sqlx_unique;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct OrganizationMember {
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub role: i16,
}

pub type OrganizationMemberResult<R> = Result<R, OrganizationMemberError>;

#[derive(Debug, thiserror::Error)]
pub enum OrganizationMemberError {
    #[error("organization member not found")]
    NotFound,
    #[error("organization member already exists")]
    AlreadyExists,
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for OrganizationMemberError {
    fn from(value: sqlx::Error) -> Self {
        handle_sqlx_unique(
            value,
            "unique_organization_member",
            |_| OrganizationMemberError::AlreadyExists,
            OrganizationMemberError::Unknown,
        )
    }
}

impl IntoResponse for OrganizationMemberError {
    fn into_response(self) -> Response {
        match self {
            OrganizationMemberError::Unknown(err) => {
                error!("{}", err);
                ErrorResponse::of(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
                    .into_response()
            }
            OrganizationMemberError::NotFound => {
                ErrorResponse::of(StatusCode::NOT_FOUND, "organization member not found")
                    .into_response()
            }
            OrganizationMemberError::AlreadyExists => {
                ErrorResponse::of(StatusCode::CONFLICT, "organization member already exists")
                    .into_response()
            }
        }
    }
}
