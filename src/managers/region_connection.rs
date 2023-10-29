use std::collections::HashMap;
use std::sync::Arc;

use kube::config::KubeConfigOptions;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::clients::bridge_service::BridgeServiceClient;
use crate::clients::kube::KubeWrappedClient;
use crate::managers::region::RegionManager;

#[derive(Clone)]
pub struct RegionConnectionManager {
    kube: Arc<RwLock<HashMap<Uuid, KubeWrappedClient>>>,
    bridge: Arc<RwLock<HashMap<Uuid, BridgeServiceClient>>>,
}

impl RegionConnectionManager {
    pub async fn new(region_manager: RegionManager) -> Self {
        let kube: Arc<RwLock<HashMap<Uuid, KubeWrappedClient>>> = Default::default();
        let bridge: Arc<RwLock<HashMap<Uuid, BridgeServiceClient>>> = Default::default();

        for region in region_manager.list().await.unwrap() {
            let client: kube::Client = kube::Config::from_custom_kubeconfig(
                region.options.kube.clone(),
                &KubeConfigOptions::default(),
            )
            .await
            .unwrap()
            .try_into()
            .unwrap();

            let mut kube = kube.write().await;
            kube.insert(region.id.clone(), KubeWrappedClient::new(client));

            let mut bridge = bridge.write().await;
            bridge.insert(
                region.id.clone(),
                BridgeServiceClient::new(region.options.bridge.base_path.clone()),
            );
        }

        Self { kube, bridge }
    }

    pub async fn find_kube_wrapped_client_by_id(
        &self,
        region_id: &Uuid,
    ) -> Option<KubeWrappedClient> {
        let kube = self.kube.read().await;
        kube.get(region_id).cloned()
    }

    pub async fn find_bridge_service_client_by_id(
        &self,
        region_id: &Uuid,
    ) -> Option<BridgeServiceClient> {
        let bridge = self.bridge.read().await;
        bridge.get(region_id).cloned()
    }
}
