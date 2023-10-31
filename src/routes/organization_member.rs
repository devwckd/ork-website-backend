use crate::domains::organization_member::{OrganizationMember, OrganizationMemberResult};
use crate::managers::organization_member::OrganizationMemberManager;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::Json;
use uuid::Uuid;

pub fn router(organization_member_manager: OrganizationMemberManager) -> axum::Router {
    let state = OrganizationMemberState {
        organization_member_manager,
    };

    axum::Router::new().route("/", get(list)).with_state(state)
}

async fn list(
    State(OrganizationMemberState {
        organization_member_manager,
        ..
    }): State<OrganizationMemberState>,
    Path((org_id,)): Path<(Uuid,)>,
) -> OrganizationMemberResult<Json<Vec<OrganizationMember>>> {
    organization_member_manager.list(&org_id).await.map(Json)
}

#[derive(Clone)]
struct OrganizationMemberState {
    organization_member_manager: OrganizationMemberManager,
}
