use crate::domains::organization::OrganizationError;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct ProxyTemplate {
    pub id: Uuid,
    pub slug: String,
    pub image: String,
    pub plugins_dir: String,
}

#[derive(Clone, Debug, serde::Deserialize, validator::Validate)]
pub struct CreateProxyTemplateData {
    pub slug: String,
    pub image: String,
    pub plugins_dir: String,
}

pub type ProxyTemplateResult<R> = Result<R, ProxyTemplateError>;

#[derive(Debug, thiserror::Error)]
pub enum ProxyTemplateError {
    #[error("validation errors: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("proxy template not found")]
    NotFound,
    #[error("organization not found")]
    OrganizationNotFound,
}

impl From<OrganizationError> for ProxyTemplateError {
    fn from(value: OrganizationError) -> Self {
        todo!()
    }
}

impl From<sqlx::Error> for ProxyTemplateError {
    fn from(value: sqlx::Error) -> Self {
        todo!()
    }
}

impl IntoResponse for ProxyTemplateError {
    fn into_response(self) -> Response {
        todo!()
    }
}
