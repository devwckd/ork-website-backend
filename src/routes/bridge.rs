use axum::extract::State;
use axum::routing::post;
use axum::Json;
use ork_bridge_service::domains::namespace::CreateNamespaceData;
use validator::Validate;

use crate::domains::bridge::{Bridge, BridgeResult, CreateBridgeData};
use crate::extractors::authenticated_org_member::AuthenticatedOrgMember;
use crate::managers::bridge::BridgeManager;
use crate::managers::region_connection::RegionConnectionManager;

pub fn router(
    bridge_manager: BridgeManager,
    region_connection_manager: RegionConnectionManager,
) -> axum::Router {
    let state = BridgeState {
        bridge_manager,
        region_connection_manager,
    };

    axum::Router::new()
        .route("/", post(create))
        .with_state(state)
}

async fn create(
    State(BridgeState {
        bridge_manager,
        region_connection_manager,
    }): State<BridgeState>,
    org_member: AuthenticatedOrgMember,
    Json(data): Json<CreateBridgeData>,
) -> BridgeResult<Json<Bridge>> {
    data.validate()?;

    let organization = org_member.org();

    let bs_client = region_connection_manager
        .find_bridge_service_client_by_id(&organization.region_id)
        .await
        .unwrap();

    let namespace = bs_client
        .create_namespace(&CreateNamespaceData {
            slug: format!("{}-{}-{}", &organization.slug, data.slug, "00000000"),
        })
        .await?;

    let mut bridge = Bridge {
        id: Default::default(), // Placeholder id, BridgeManager#create will override it with the service id.
        slug: data.slug,
        bs_namespace_id: namespace.id,
    };

    bridge_manager.create(organization, &mut bridge).await?;

    Ok(Json(bridge))
}

#[derive(Clone)]
struct BridgeState {
    bridge_manager: BridgeManager,
    region_connection_manager: RegionConnectionManager,
}
