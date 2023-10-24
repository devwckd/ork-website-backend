use crate::domains::organization::{Organization, OrganizationError, OrganizationResult};
use sqlx::{query, query_as, Column, Row, ValueRef};
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct OrganizationRepository {
    pg_pool: sqlx::PgPool,
}

impl OrganizationRepository {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn list_participating(
        &self,
        user_id: &Uuid,
    ) -> OrganizationResult<Vec<Organization>> {
        let organizations = query_as(
            r#"
        SELECT *
        FROM organizations
        INNER JOIN organization_members ON organization_id = organizations.id 
            AND user_id = $1;
        "#,
        )
        .bind(user_id)
        .fetch_all(&self.pg_pool)
        .await?;

        Ok(organizations)
    }

    pub async fn find_by_id(&self, organization_id: &Uuid) -> OrganizationResult<Organization> {
        query_as("SELECT * FROM organizations WHERE id = $1;")
            .bind(&organization_id)
            .fetch_optional(&self.pg_pool)
            .await?
            .ok_or(OrganizationError::NotFound)
    }

    pub async fn insert(&self, organization: &Organization) -> OrganizationResult<()> {
        query("INSERT INTO organizations(id, slug, region_id) VALUES ($1, $2, $3);")
            .bind(&organization.id)
            .bind(&organization.slug)
            .bind(&organization.region_id)
            .execute(&self.pg_pool)
            .await?;
        Ok(())
    }
}
