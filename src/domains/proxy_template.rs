use crate::domains::bridge::BridgeError;
use crate::domains::error::ErrorResponse;
use crate::domains::organization::OrganizationError;
use crate::utils::handle_sqlx_unique;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tracing::error;
use uuid::Uuid;
use validator::ValidationErrors;

#[derive(Clone, Debug, serde::Serialize, sqlx::FromRow)]
pub struct ProxyTemplate {
    pub id: Uuid,
    pub slug: String,
    pub image: String,
    pub plugins_dir: String,

    pub bridge_id: Option<Uuid>,
}

#[derive(Clone, Debug, serde::Deserialize, validator::Validate)]
pub struct CreateProxyTemplateData {
    #[validate(regex = "crate::consts::SLUG_REGEX")]
    pub slug: String,
    pub image: String,
    pub plugins_dir: String,
    #[validate(regex = "crate::consts::SLUG_REGEX")]
    pub bridge_slug: Option<String>,
}

pub type ProxyTemplateResult<R> = Result<R, ProxyTemplateError>;

#[derive(Debug, thiserror::Error)]
pub enum ProxyTemplateError {
    #[error("proxy template already exists")]
    AlreadyExists,
    #[error("bridge not found")]
    BridgeNotFound,
    #[error("validation errors: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("proxy template not found")]
    NotFound,
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<sqlx::Error> for ProxyTemplateError {
    fn from(value: sqlx::Error) -> Self {
        handle_sqlx_unique(
            value,
            "unique_proxy_template_slug_per_organization",
            |_| ProxyTemplateError::AlreadyExists,
            ProxyTemplateError::Unknown,
        )
    }
}

impl From<BridgeError> for ProxyTemplateError {
    fn from(value: BridgeError) -> Self {
        match value {
            BridgeError::NotFound => ProxyTemplateError::BridgeNotFound,
            BridgeError::AlreadyExists => unreachable!(),
            BridgeError::Validation(_) => unreachable!(),
            BridgeError::Unknown(err) => ProxyTemplateError::Unknown(err),
        }
    }
}
impl IntoResponse for ProxyTemplateError {
    fn into_response(self) -> Response {
        match self {
            ProxyTemplateError::Unknown(err) => {
                error!("{}", err);
                ErrorResponse::of(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
                    .into_response()
            }
            ProxyTemplateError::BridgeNotFound => {
                ErrorResponse::of(StatusCode::PRECONDITION_FAILED, "template not found")
                    .into_response()
            }
            ProxyTemplateError::AlreadyExists => {
                ErrorResponse::of(StatusCode::CONFLICT, "organization member already exists")
                    .into_response()
            }
            ProxyTemplateError::Validation(err) => {
                ErrorResponse::of(StatusCode::BAD_REQUEST, err).into_response()
            }
            ProxyTemplateError::NotFound => {
                ErrorResponse::of(StatusCode::NOT_FOUND, "proxy template error").into_response()
            }
        }
    }
}
