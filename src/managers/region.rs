use crate::domains::region::{Region, RegionResult};
use crate::repositories::regions::RegionRepository;

#[derive(Clone)]
pub struct RegionManager {
    region_repository: RegionRepository,
}

impl RegionManager {
    pub fn new(region_repository: RegionRepository) -> Self {
        Self { region_repository }
    }

    pub async fn list(&self) -> RegionResult<Vec<Region>> {
        self.region_repository.list().await
    }

    pub async fn find_by_slug(&self, slug: &String) -> RegionResult<Region> {
        self.region_repository.find_by_slug(slug).await
    }
}
