use std::collections::HashMap;
use std::sync::Arc;

use kube::config::KubeConfigOptions;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::managers::region::RegionManager;

#[derive(Clone)]
pub struct RegionConnectionManager {
    kube: Arc<RwLock<HashMap<Uuid, kube::Client>>>,
}

impl RegionConnectionManager {
    pub async fn new(region_manager: RegionManager) -> Self {
        let kube: Arc<RwLock<HashMap<Uuid, kube::Client>>> = Default::default();

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
            kube.insert(region.id, client);
        }

        Self { kube }
    }

    pub async fn find_kube_client_by_id(&self, region_id: &Uuid) -> Option<kube::Client> {
        let kube = self.kube.read().await;
        kube.get(region_id).cloned()
    }
}
