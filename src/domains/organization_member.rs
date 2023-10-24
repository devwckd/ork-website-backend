use axum::response::{IntoResponse, Response};
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
}

impl From<sqlx::Error> for OrganizationMemberError {
    fn from(value: sqlx::Error) -> Self {
        todo!()
    }
}

impl IntoResponse for OrganizationMemberError {
    fn into_response(self) -> Response {
        todo!()
    }
}
