use crate::domains::organization_member::OrganizationMemberError;
use crate::domains::proxy_template::{
    CreateProxyTemplateData, ProxyTemplate, ProxyTemplateError, ProxyTemplateResult,
};
use crate::extractors::authenticated::{Authenticated, SessionState};
use crate::managers::organization::OrganizationManager;
use crate::managers::organization_member::OrganizationMemberManager;
use crate::managers::proxy_template::ProxyTemplateManager;
use crate::managers::session::SessionManager;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::Json;
use uuid::Uuid;
use validator::Validate;

pub fn router(
    session_manager: SessionManager,
    organization_member_manager: OrganizationMemberManager,
    proxy_template_manager: ProxyTemplateManager,
) -> axum::Router {
    let state = ProxyTemplateState {
        session_manager,
        organization_member_manager,
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
        organization_member_manager,
        ..
    }): State<ProxyTemplateState>,
    Path((organization_id,)): Path<(Uuid,)>,
    user: Authenticated,
) -> ProxyTemplateResult<Json<Vec<ProxyTemplate>>> {
    let _ = organization_member_manager
        .find_with_role(&organization_id, &user.id, 0)
        .await
        .map_err(|err| match err {
            _ => ProxyTemplateError::OrganizationNotFound,
        })?;

    proxy_template_manager
        .list(&organization_id)
        .await
        .map(Json)
}

async fn create(
    State(ProxyTemplateState {
        proxy_template_manager,
        organization_member_manager,
        ..
    }): State<ProxyTemplateState>,
    Path((organization_id,)): Path<(Uuid,)>,
    user: Authenticated,
    Json(data): Json<CreateProxyTemplateData>,
) -> ProxyTemplateResult<Json<ProxyTemplate>> {
    data.validate()?;

    let _ = organization_member_manager
        .find_with_role(&organization_id, &user.id, 0)
        .await
        .map_err(|err| match err {
            _ => ProxyTemplateError::OrganizationNotFound,
        })?;

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
    session_manager: SessionManager,
    organization_member_manager: OrganizationMemberManager,
    proxy_template_manager: ProxyTemplateManager,
}

impl SessionState for ProxyTemplateState {
    fn session_manager(&self) -> &SessionManager {
        &self.session_manager
    }
}
