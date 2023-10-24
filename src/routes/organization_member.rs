use crate::domains::organization_member::{OrganizationMember, OrganizationMemberResult};
use crate::extractors::authenticated::SessionState;
use crate::managers::organization_member::OrganizationMemberManager;
use crate::managers::session::SessionManager;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::Json;
use uuid::Uuid;

pub fn router(
    session_manager: SessionManager,
    organization_member_manager: OrganizationMemberManager,
) -> axum::Router {
    let state = OrganizationMemberState {
        session_manager,
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
    session_manager: SessionManager,
    organization_member_manager: OrganizationMemberManager,
}

impl SessionState for OrganizationMemberState {
    fn session_manager(&self) -> &SessionManager {
        &self.session_manager
    }
}
