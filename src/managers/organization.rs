use uuid::Uuid;

use crate::domains::organization::{Organization, OrganizationResult};
use crate::managers::region_connection::RegionConnectionManager;
use crate::repositories::organization::OrganizationRepository;

#[derive(Clone)]
pub struct OrganizationManager {
    region_connection_manager: RegionConnectionManager,
    organization_repository: OrganizationRepository,
}

impl OrganizationManager {
    pub fn new(
        region_connection_manager: RegionConnectionManager,
        organization_repository: OrganizationRepository,
    ) -> Self {
        Self {
            region_connection_manager,
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

        self.region_connection_manager
            .find_kube_wrapped_client_by_id(&organization.region_id)
            .await
            .unwrap()
            .create_organization_namespace(&organization)
            .await;

        Ok(())
    }
}
