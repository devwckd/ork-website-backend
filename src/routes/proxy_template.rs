use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

use crate::domains::proxy_template::{CreateProxyTemplateData, ProxyTemplate, ProxyTemplateResult};
use crate::extractors::authenticated_org_member::AuthenticatedOrgMember;
use crate::managers::proxy_template::ProxyTemplateManager;

pub fn router(proxy_template_manager: ProxyTemplateManager) -> axum::Router {
    let state = ProxyTemplateState {
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
        proxy_template_manager,
        ..
    }): State<ProxyTemplateState>,
    Path((organization_id,)): Path<(Uuid,)>,
    _org_member: AuthenticatedOrgMember,
    Json(data): Json<CreateProxyTemplateData>,
) -> ProxyTemplateResult<Json<ProxyTemplate>> {
    data.validate()?;

    let proxy_template = ProxyTemplate {
        id: Uuid::new_v4(),
        slug: data.slug,
        image: data.image,
        plugins_dir: data.plugins_dir,
    };

    proxy_template_manager
        .insert(&organization_id, &proxy_template)
        .await?;

    Ok(Json(proxy_template))
}

#[derive(Clone)]
struct ProxyTemplateState {
    proxy_template_manager: ProxyTemplateManager,
}
