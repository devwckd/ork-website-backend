use crate::domains::region::{Region, RegionResult};
use crate::extractors::authenticated::{Authenticated, SessionState};
use crate::managers::region::RegionManager;
use crate::managers::session::SessionManager;
use axum::extract::State;
use axum::routing::get;
use axum::Json;

pub fn router(session_manager: SessionManager, region_manager: RegionManager) -> axum::Router {
    let state = RegionState {
        session_manager,
        region_manager,
    };

    axum::Router::new().route("/", get(list)).with_state(state)
}

async fn list(
    State(RegionState { region_manager, .. }): State<RegionState>,
    _user: Authenticated,
) -> RegionResult<Json<Vec<Region>>> {
    region_manager.list().await.map(Json)
}

#[derive(Clone)]
struct RegionState {
    session_manager: SessionManager,
    region_manager: RegionManager,
}

impl SessionState for RegionState {
    fn session_manager(&self) -> &SessionManager {
        &self.session_manager
    }
}
