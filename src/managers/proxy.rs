use uuid::Uuid;

use crate::domains::organization::Organization;
use crate::domains::proxy::{Proxy, ProxyResult};
use crate::domains::proxy_template::ProxyTemplate;
use crate::managers::kube::KubeManager;
use crate::managers::region_connection::RegionConnectionManager;
use crate::repositories::proxy::ProxyRepository;

#[derive(Clone)]
pub struct ProxyManager {
    kube_manager: KubeManager,
    proxy_repository: ProxyRepository,
}

impl ProxyManager {
    pub fn new(kube_manager: KubeManager, proxy_repository: ProxyRepository) -> Self {
        Self {
            kube_manager,
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
            .insert(&organization.id, &proxy)
            .await?;

        self.kube_manager
            .on_proxy_creation(organization, template, proxy)
            .await;

        Ok(())
    }
}
