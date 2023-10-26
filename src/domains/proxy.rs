use crate::domains::proxy_template::ProxyTemplateError;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct Proxy {
    pub id: Uuid,
    pub slug: String,
    pub template_id: Uuid,
}

#[derive(Clone, Debug, serde::Deserialize, validator::Validate)]
pub struct CreateProxyData {
    pub slug: Option<String>,
    pub template_slug: String,
}

pub type ProxyResult<R> = Result<R, ProxyError>;

#[derive(Debug, thiserror::Error)]
pub enum ProxyError {
    #[error("validation errors: {0}")]
    ValidationErrors(#[from] ValidationErrors),
}

impl From<sqlx::Error> for ProxyError {
    fn from(value: sqlx::Error) -> Self {
        todo!()
    }
}

impl From<ProxyTemplateError> for ProxyError {
    fn from(value: ProxyTemplateError) -> Self {
        todo!()
    }
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        todo!()
    }
}
