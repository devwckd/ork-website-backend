use crate::domains::bridge::{Bridge, BridgeResult};
use crate::domains::organization::Organization;
use crate::managers::region_connection::RegionConnectionManager;
use crate::repositories::bridge::BridgeRepository;
use ork_bridge_service::domains::namespace::CreateNamespaceData;
use uuid::Uuid;

#[derive(Clone)]
pub struct BridgeManager {
    bridge_repository: BridgeRepository,
}

impl BridgeManager {
    pub fn new(bridge_repository: BridgeRepository) -> Self {
        Self { bridge_repository }
    }

    pub async fn find_by_slug(
        &self,
        organization_id: &Uuid,
        slug: &String,
    ) -> BridgeResult<Bridge> {
        self.bridge_repository
            .find_by_slug(organization_id, slug)
            .await
    }

    pub async fn create(
        &self,
        organization: &Organization,
        bridge: &mut Bridge,
    ) -> BridgeResult<()> {
        self.bridge_repository
            .insert(&organization.id, &bridge)
            .await?;

        Ok(())
    }
}
