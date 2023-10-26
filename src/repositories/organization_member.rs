use crate::domains::organization_member::{
    OrganizationMember, OrganizationMemberError, OrganizationMemberResult,
};
use std::cmp::min;
use uuid::Uuid;

#[derive(Clone)]
pub struct OrganizationMemberRepository {
    pg_pool: sqlx::PgPool,
}

impl OrganizationMemberRepository {
    pub fn new(pg_pool: sqlx::PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn list(
        &self,
        organization_id: &Uuid,
    ) -> OrganizationMemberResult<Vec<OrganizationMember>> {
        let members = sqlx::query_as(
            r#"
        SELECT * 
        FROM organization_members
        WHERE organization_id = $1;
        "#,
        )
        .bind(organization_id)
        .fetch_all(&self.pg_pool)
        .await?;

        Ok(members)
    }

    pub async fn find_by_user_id(
        &self,
        organization_id: &Uuid,
        user_id: &Uuid,
    ) -> OrganizationMemberResult<OrganizationMember> {
        sqlx::query_as(
            "SELECT * FROM organization_members WHERE organization_id = $1 AND user_id = $2;",
        )
        .bind(&organization_id)
        .bind(&user_id)
        .fetch_optional(&self.pg_pool)
        .await?
        .ok_or(OrganizationMemberError::NotFound)
    }

    pub async fn find_with_role(
        &self,
        organization_id: &Uuid,
        user_id: &Uuid,
        min_role: i16,
    ) -> OrganizationMemberResult<OrganizationMember> {
        sqlx::query_as("SELECT * FROM organization_members WHERE organization_id = $1 AND user_id = $2 AND role >= $3;")
            .bind(&organization_id)
            .bind(&user_id)
            .bind(&min_role).fetch_optional(&self.pg_pool)
            .await?
            .ok_or(OrganizationMemberError::NotFound)
    }

    pub async fn insert(
        &self,
        organization_id: &Uuid,
        organization_member: &OrganizationMember,
    ) -> OrganizationMemberResult<()> {
        sqlx::query(
            "INSERT INTO organization_members(user_id, organization_id, role) VALUES ($1, $2, $3);",
        )
        .bind(organization_member.user_id)
        .bind(organization_id)
        .bind(organization_member.role)
        .execute(&self.pg_pool)
        .await?;

        Ok(())
    }
}
