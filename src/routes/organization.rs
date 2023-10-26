use crate::domains::organization::{CreateOrganizationData, Organization, OrganizationResult};
use crate::domains::organization_member::OrganizationMember;
use crate::extractors::authenticated_user::AuthenticatedUser;
use crate::managers::organization::OrganizationManager;
use crate::managers::organization_member::OrganizationMemberManager;
use crate::managers::region::RegionManager;
use crate::managers::session::SessionManager;
use axum::extract::State;
use axum::routing::{get, post};
use axum::Json;
use log::info;
use uuid::Uuid;
use validator::Validate;

pub fn router(
    organization_manager: OrganizationManager,
    organization_member_manager: OrganizationMemberManager,
    region_manager: RegionManager,
) -> axum::Router {
    let state = OrganizationState {
        organization_manager,
        organization_member_manager,
        region_manager,
    };

    axum::Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .with_state(state)
}

async fn list(
    State(OrganizationState {
        organization_manager,
        ..
    }): State<OrganizationState>,
    user: AuthenticatedUser,
) -> OrganizationResult<Json<Vec<Organization>>> {
    organization_manager
        .list_participating(&user.id)
        .await
        .map(Json)
}

async fn create(
    State(OrganizationState {
        organization_manager,
        organization_member_manager,
        region_manager,
        ..
    }): State<OrganizationState>,
    user: AuthenticatedUser,
    Json(data): Json<CreateOrganizationData>,
) -> OrganizationResult<Json<Organization>> {
    data.validate()?;

    let region = region_manager.find_by_slug(&data.region_slug).await?;

    let organization = Organization {
        id: Uuid::new_v4(),
        slug: data.slug,
        region_id: region.id.clone(),
    };

    organization_manager.create(&organization).await?;

    let organization_member = OrganizationMember {
        organization_id: organization.id.clone(),
        user_id: user.id.clone(),
        role: 2,
    };

    organization_member_manager
        .insert(&organization.id, &organization_member)
        .await?;

    Ok(Json(organization))
}

#[derive(Clone)]
struct OrganizationState {
    organization_manager: OrganizationManager,
    organization_member_manager: OrganizationMemberManager,
    region_manager: RegionManager,
}
