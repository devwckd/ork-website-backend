use crate::domains::region::{Region, RegionError, RegionResult};
use log::info;
use sqlx::query_as;

#[derive(Clone)]
pub struct RegionRepository {
    pg_pool: sqlx::PgPool,
}

impl RegionRepository {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn list(&self) -> RegionResult<Vec<Region>> {
        Ok(query_as("SELECT * FROM regions;")
            .fetch_all(&self.pg_pool)
            .await?)
    }

    pub async fn find_by_slug(&self, slug: &String) -> RegionResult<Region> {
        query_as("SELECT * FROM regions WHERE slug = $1 LIMIT 1;")
            .bind(slug)
            .fetch_optional(&self.pg_pool)
            .await?
            .ok_or(RegionError::NotFound)
    }
}
