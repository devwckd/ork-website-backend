use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::domains::bridge::{Bridge, BridgeResult, CreateBridgeData};
use crate::extractors::authenticated_org_member::AuthenticatedOrgMember;
use crate::managers::bridge::BridgeManager;

pub fn router(bridge_manager: BridgeManager) -> axum::Router {
    let state = BridgeState { bridge_manager };

    axum::Router::new().with_state(state)
}

async fn create(
    State(BridgeState { bridge_manager }): State<BridgeState>,
    org_member: AuthenticatedOrgMember,
    Json(data): Json<CreateBridgeData>,
) -> BridgeResult<Json<Bridge>> {
    data.validate()?;

    let mut bridge = Bridge {
        id: Uuid::new_v4(),
        slug: data.slug,
        external_id: Default::default(),
    };

    bridge_manager.create(org_member.org(), &mut bridge).await?;

    Ok(Json(bridge))
}

#[derive(Clone)]
struct BridgeState {
    bridge_manager: BridgeManager,
}
