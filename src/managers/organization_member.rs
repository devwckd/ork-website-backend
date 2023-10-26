use crate::domains::organization_member::{OrganizationMember, OrganizationMemberResult};
use crate::repositories::organization_member::OrganizationMemberRepository;
use std::cmp::min;
use uuid::Uuid;

#[derive(Clone)]
pub struct OrganizationMemberManager {
    organization_member_repository: OrganizationMemberRepository,
}

impl OrganizationMemberManager {
    pub fn new(organization_member_repository: OrganizationMemberRepository) -> Self {
        Self {
            organization_member_repository,
        }
    }

    pub async fn find_with_role(
        &self,
        organization_id: &Uuid,
        user_id: &Uuid,
        min_role: i16,
    ) -> OrganizationMemberResult<OrganizationMember> {
        self.organization_member_repository
            .find_with_role(organization_id, user_id, min_role)
            .await
    }

    pub async fn find_by_user_id(
        &self,
        organization_id: &Uuid,
        user_id: &Uuid,
    ) -> OrganizationMemberResult<OrganizationMember> {
        self.organization_member_repository
            .find_by_user_id(organization_id, user_id)
            .await
    }

    pub async fn list(
        &self,
        organization_id: &Uuid,
    ) -> OrganizationMemberResult<Vec<OrganizationMember>> {
        self.organization_member_repository
            .list(organization_id)
            .await
    }

    pub async fn insert(
        &self,
        organization_id: &Uuid,
        organization_member: &OrganizationMember,
    ) -> OrganizationMemberResult<()> {
        self.organization_member_repository
            .insert(organization_id, organization_member)
            .await
    }
}
