use uuid::Uuid;

use crate::domains::organization::{Organization, OrganizationResult};
use crate::managers::kube::KubeManager;
use crate::repositories::organization::OrganizationRepository;

#[derive(Clone)]
pub struct OrganizationManager {
    kube_manager: KubeManager,
    organization_repository: OrganizationRepository,
}

impl OrganizationManager {
    pub fn new(kube_manager: KubeManager, organization_repository: OrganizationRepository) -> Self {
        Self {
            kube_manager,
            organization_repository,
        }
    }

    pub async fn list_participating(
        &self,
        user_id: &Uuid,
    ) -> OrganizationResult<Vec<Organization>> {
        self.organization_repository
            .list_participating(user_id)
            .await
    }

    pub async fn find_by_id(&self, organization_id: &Uuid) -> OrganizationResult<Organization> {
        self.organization_repository
            .find_by_id(organization_id)
            .await
    }

    pub async fn create(&self, organization: &Organization) -> OrganizationResult<()> {
        self.organization_repository.insert(organization).await?;

        self.kube_manager
            .on_organization_creation(&organization)
            .await;

        Ok(())
    }
}
