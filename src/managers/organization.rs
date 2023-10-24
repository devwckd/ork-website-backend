use crate::domains::organization::{Organization, OrganizationResult};
use crate::repositories::organization::OrganizationRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct OrganizationManager {
    organization_repository: OrganizationRepository,
}

impl OrganizationManager {
    pub fn new(organization_repository: OrganizationRepository) -> Self {
        Self {
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

    pub async fn insert(&self, organization: &Organization) -> OrganizationResult<()> {
        self.organization_repository.insert(organization).await
    }
}
