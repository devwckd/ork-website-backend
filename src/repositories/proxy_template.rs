use crate::domains::proxy_template::{ProxyTemplate, ProxyTemplateResult};
use sqlx::{query, query_as};
use uuid::Uuid;

#[derive(Clone)]
pub struct ProxyTemplateRepository {
    pg_pool: sqlx::PgPool,
}

impl ProxyTemplateRepository {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn list(&self, organization_id: &Uuid) -> ProxyTemplateResult<Vec<ProxyTemplate>> {
        Ok(
            query_as("SELECT * FROM proxy_templates WHERE organization_id = $1;")
                .bind(&organization_id)
                .fetch_all(&self.pg_pool)
                .await?,
        )
    }

    pub async fn insert(
        &self,
        organization_id: &Uuid,
        proxy_template: &ProxyTemplate,
    ) -> ProxyTemplateResult<()> {
        query("INSERT INTO proxy_templates(id, slug, image, plugins_dir, organization_id) VALUES ($1, $2, $3, $4, $5);")
            .bind(&proxy_template.id)
            .bind(&proxy_template.slug)
            .bind(&proxy_template.image)
            .bind(&proxy_template.plugins_dir)
            .bind(&organization_id)
            .execute(&self.pg_pool).await?;

        Ok(())
    }
}
