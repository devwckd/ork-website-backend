use crate::domains::bridge::{Bridge, BridgeResult};
use uuid::Uuid;

#[derive(Clone)]
pub struct BridgeRepository {
    pg_pool: sqlx::PgPool,
}

impl BridgeRepository {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn insert(&self, organization_id: &Uuid, bridge: &Bridge) -> BridgeResult<()> {
        sqlx::query(
            "INSERT INTO bridges(id, slug, external_id, organization_id) VALUES ($1, $2, $3, $4);",
        )
        .bind(&bridge.id)
        .bind(&bridge.slug)
        .bind(&bridge.external_id)
        .bind(&organization_id)
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }
}
