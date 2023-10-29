use crate::domains::bridge::{Bridge, BridgeResult};
use crate::domains::organization::Organization;
use crate::managers::region_connection::RegionConnectionManager;
use crate::repositories::bridge::BridgeRepository;
use ork_bridge_service::domains::namespace::CreateNamespaceData;

#[derive(Clone)]
pub struct BridgeManager {
    bridge_repository: BridgeRepository,
    region_connection_manager: RegionConnectionManager,
}

impl BridgeManager {
    pub fn new(
        bridge_repository: BridgeRepository,
        region_connection_manager: RegionConnectionManager,
    ) -> Self {
        Self {
            bridge_repository,
            region_connection_manager,
        }
    }

    pub async fn create(
        &self,
        organization: &Organization,
        bridge: &mut Bridge,
    ) -> BridgeResult<()> {
        let bridge_service_client = self
            .region_connection_manager
            .find_bridge_service_client_by_id(&organization.region_id)
            .await
            .unwrap();

        let create_namespace_data = CreateNamespaceData {
            slug: format!("{}-{}", organization.slug, bridge.slug),
        };

        let namespace = bridge_service_client
            .create_namespace(&create_namespace_data)
            .await?;

        bridge.external_id = namespace.id;

        self.bridge_repository
            .insert(&organization.id, &bridge)
            .await?;

        Ok(())
    }
}
