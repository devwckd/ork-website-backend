use uuid::Uuid;

use crate::domains::bridge::{Bridge, BridgeError, BridgeResult};

#[derive(Clone)]
pub struct BridgeRepository {
    pg_pool: sqlx::PgPool,
}

impl BridgeRepository {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn find_by_slug(
        &self,
        organization_id: &Uuid,
        slug: &String,
    ) -> BridgeResult<Bridge> {
        sqlx::query_as("SELECT * FROM bridges WHERE slug = $1 AND organization_id = $2;")
            .bind(&slug)
            .bind(&organization_id)
            .fetch_optional(&self.pg_pool)
            .await?
            .ok_or(BridgeError::NotFound)
    }

    pub async fn insert(&self, organization_id: &Uuid, bridge: &Bridge) -> BridgeResult<()> {
        sqlx::query(
            "INSERT INTO bridges(id, slug, bs_namespace_id, organization_id) VALUES ($1, $2, $3, $4);",
        )
        .bind(&bridge.id)
        .bind(&bridge.slug)
        .bind(&bridge.bs_namespace_id)
        .bind(&organization_id)
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }
}
