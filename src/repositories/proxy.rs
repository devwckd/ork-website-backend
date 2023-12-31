use crate::domains::proxy::{Proxy, ProxyResult};
use uuid::Uuid;

#[derive(Clone)]
pub struct ProxyRepository {
    pg_pool: sqlx::PgPool,
}

impl ProxyRepository {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn list(&self, organization_id: &Uuid) -> ProxyResult<Vec<Proxy>> {
        Ok(
            sqlx::query_as("SELECT * FROM proxies WHERE organization_id = $1;")
                .bind(organization_id)
                .fetch_all(&self.pg_pool)
                .await?,
        )
    }

    pub async fn insert(&self, organization_id: &Uuid, proxy: &Proxy) -> ProxyResult<()> {
        sqlx::query(
            "INSERT INTO proxies(id, slug, bridge_id, bs_proxy_id, template_id, organization_id) VALUES ($1, $2, $3, $4);",
        )
        .bind(&proxy.id)
        .bind(&proxy.slug)
        .bind(&proxy.bridge_id)
        .bind(&proxy.bs_proxy_id)
        .bind(&proxy.template_id)
        .bind(&organization_id)
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }
}
