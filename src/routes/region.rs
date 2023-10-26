use crate::domains::region::{Region, RegionResult};
use crate::extractors::authenticated_user::AuthenticatedUser;
use crate::managers::region::RegionManager;
use axum::extract::State;
use axum::routing::get;
use axum::Json;

pub fn router(region_manager: RegionManager) -> axum::Router {
    let state = RegionState { region_manager };

    axum::Router::new().route("/", get(list)).with_state(state)
}

async fn list(
    State(RegionState { region_manager, .. }): State<RegionState>,
    _user: AuthenticatedUser,
) -> RegionResult<Json<Vec<Region>>> {
    region_manager.list().await.map(Json)
}

#[derive(Clone)]
struct RegionState {
    region_manager: RegionManager,
}
