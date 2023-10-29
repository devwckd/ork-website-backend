use ork_bridge_service::domains::namespace::{CreateNamespaceData, Namespace};
use reqwest::StatusCode;

#[derive(Clone)]
pub struct BridgeServiceClient {
    base_path: String,
    reqwest: reqwest::Client,
}

impl BridgeServiceClient {
    pub fn new(base_path: String) -> Self {
        Self {
            base_path,
            reqwest: reqwest::Client::builder().build().unwrap(),
        }
    }

    pub async fn create_namespace(
        &self,
        data: &CreateNamespaceData,
    ) -> BridgeServiceResult<Namespace> {
        match self
            .reqwest
            .post(format!("{}/namespaces", &self.base_path))
            .json(&data)
            .send()
            .await
        {
            Ok(res) => match res.status() {
                StatusCode::OK => Ok(res.json().await.unwrap()),
                StatusCode::CONFLICT => Err(BridgeServiceError::NamespaceAlreadyExists),
                _ => Err(BridgeServiceError::Unknown(res.status().to_string())),
            },
            Err(err) => Err(BridgeServiceError::Unknown(err.to_string())),
        }
    }
}

pub type BridgeServiceResult<R> = Result<R, BridgeServiceError>;

#[derive(Debug, thiserror::Error)]
pub enum BridgeServiceError {
    #[error("namespace already exists")]
    NamespaceAlreadyExists,
    #[error("unknown error: {0}")]
    Unknown(String),
}
