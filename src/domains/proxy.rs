use crate::domains::error::ErrorResponse;
use crate::domains::proxy_template::ProxyTemplateError;
use crate::utils::handle_sqlx_unique;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct Proxy {
    pub id: Uuid,
    pub slug: String,

    #[serde(skip_serializing)]
    pub bridge_id: Option<Uuid>,
    #[serde(skip_serializing)]
    pub bs_proxy_id: Option<Uuid>,

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
    #[error("proxy already exists")]
    AlreadyExists,
    #[error("proxy template not found")]
    TemplateNotFound,
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for ProxyError {
    fn from(value: sqlx::Error) -> Self {
        handle_sqlx_unique(
            value,
            "unique_proxy_slug_per_organization",
            |_| ProxyError::AlreadyExists,
            ProxyError::Unknown,
        )
    }
}

impl From<ProxyTemplateError> for ProxyError {
    fn from(value: ProxyTemplateError) -> Self {
        match value {
            ProxyTemplateError::NotFound => ProxyError::TemplateNotFound,
            ProxyTemplateError::Unknown(err) => ProxyError::Unknown(err),
            _ => unreachable!(),
        }
    }
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        match self {
            ProxyError::Unknown(err) => {
                error!("{}", err);
                ErrorResponse::of(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
                    .into_response()
            }
            ProxyError::TemplateNotFound => {
                ErrorResponse::of(StatusCode::PRECONDITION_FAILED, "template not found")
                    .into_response()
            }
            ProxyError::AlreadyExists => {
                ErrorResponse::of(StatusCode::CONFLICT, "organization member already exists")
                    .into_response()
            }
            ProxyError::ValidationErrors(err) => {
                ErrorResponse::of(StatusCode::BAD_REQUEST, err).into_response()
            }
        }
    }
}
