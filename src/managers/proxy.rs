use uuid::Uuid;

use crate::domains::organization::Organization;
use crate::domains::proxy::{Proxy, ProxyResult};
use crate::domains::proxy_template::ProxyTemplate;
use crate::managers::region_connection::RegionConnectionManager;
use crate::repositories::proxy::ProxyRepository;

#[derive(Clone)]
pub struct ProxyManager {
    region_connection_manager: RegionConnectionManager,
    proxy_repository: ProxyRepository,
}

impl ProxyManager {
    pub fn new(
        region_connection_manager: RegionConnectionManager,
        proxy_repository: ProxyRepository,
    ) -> Self {
        Self {
            region_connection_manager,
            proxy_repository,
        }
    }

    pub async fn list(&self, organization_id: &Uuid) -> ProxyResult<Vec<Proxy>> {
        self.proxy_repository.list(organization_id).await
    }

    pub async fn create(
        &self,
        organization: &Organization,
        template: &ProxyTemplate,
        proxy: &Proxy,
    ) -> ProxyResult<()> {
        self.proxy_repository
            .insert(&organization.id, proxy)
            .await?;

        self.region_connection_manager
            .find_kube_wrapped_client_by_id(&organization.region_id)
            .await
            .unwrap()
            .create_proxy_pod(organization, template, proxy)
            .await;

        Ok(())
    }
}
