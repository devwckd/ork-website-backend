use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::domains::proxy_template::{CreateProxyTemplateData, ProxyTemplate, ProxyTemplateResult};
use crate::extractors::authenticated_org_member::AuthenticatedOrgMember;
use crate::managers::bridge::BridgeManager;
use crate::managers::proxy_template::ProxyTemplateManager;

pub fn router(
    bridge_manager: BridgeManager,
    proxy_template_manager: ProxyTemplateManager,
) -> axum::Router {
    let state = ProxyTemplateState {
        bridge_manager,
        proxy_template_manager,
    };

    axum::Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .with_state(state)
}
async fn list(
    State(ProxyTemplateState {
        proxy_template_manager,
        ..
    }): State<ProxyTemplateState>,
    Path((organization_id,)): Path<(Uuid,)>,
    _org_member: AuthenticatedOrgMember,
) -> ProxyTemplateResult<Json<Vec<ProxyTemplate>>> {
    proxy_template_manager
        .list(&organization_id)
        .await
        .map(Json)
}

async fn create(
    State(ProxyTemplateState {
        bridge_manager,
        proxy_template_manager,
        ..
    }): State<ProxyTemplateState>,
    org_member: AuthenticatedOrgMember,
    Json(data): Json<CreateProxyTemplateData>,
) -> ProxyTemplateResult<Json<ProxyTemplate>> {
    data.validate()?;
    let organization = org_member.org();

    let bridge_id = if let Some(bridge_slug) = data.bridge_slug {
        Some(
            bridge_manager
                .find_by_slug(&organization.id, &bridge_slug)
                .await?
                .id,
        )
    } else {
        None
    };

    let proxy_template = ProxyTemplate {
        id: Uuid::new_v4(),
        slug: data.slug,
        image: data.image,
        plugins_dir: data.plugins_dir,
        bridge_id,
    };

    proxy_template_manager
        .insert(&organization.id, &proxy_template)
        .await?;

    Ok(Json(proxy_template))
}

#[derive(Clone)]
struct ProxyTemplateState {
    bridge_manager: BridgeManager,
    proxy_template_manager: ProxyTemplateManager,
}
