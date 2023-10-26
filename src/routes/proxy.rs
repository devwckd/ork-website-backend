use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::Json;
use rand::distributions::Alphanumeric;
use rand::Rng;
use uuid::Uuid;
use validator::Validate;

use crate::domains::proxy::{CreateProxyData, Proxy, ProxyResult};
use crate::extractors::authenticated_org_member::AuthenticatedOrgMember;
use crate::managers::proxy::ProxyManager;
use crate::managers::proxy_template::ProxyTemplateManager;

pub fn router(
    proxy_manager: ProxyManager,
    proxy_template_manager: ProxyTemplateManager,
) -> axum::Router {
    let state = ProxyState {
        proxy_manager,
        proxy_template_manager,
    };

    axum::Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .with_state(state)
}

async fn list(
    State(ProxyState { proxy_manager, .. }): State<ProxyState>,
    Path((organization_id,)): Path<(Uuid,)>,
    _org_member: AuthenticatedOrgMember,
) -> ProxyResult<Json<Vec<Proxy>>> {
    proxy_manager.list(&organization_id).await.map(Json)
}

async fn create(
    State(ProxyState {
        proxy_manager,
        proxy_template_manager,
        ..
    }): State<ProxyState>,
    Path((organization_id,)): Path<(Uuid,)>,
    org_member: AuthenticatedOrgMember,
    Json(data): Json<CreateProxyData>,
) -> ProxyResult<Json<Proxy>> {
    data.validate()?;

    let template = proxy_template_manager
        .find_by_slug(&organization_id, &data.template_slug)
        .await?;

    let proxy = Proxy {
        id: Uuid::new_v4(),
        slug: data.slug.unwrap_or_else(|| {
            format!(
                "{}-{}",
                template.slug,
                rand::thread_rng()
                    .sample_iter(Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect::<String>()
                    .to_lowercase()
            )
        }),
        template_id: template.id.clone(),
    };

    proxy_manager
        .create(org_member.org(), &template, &proxy)
        .await?;

    Ok(Json(proxy))
}
#[derive(Clone)]
struct ProxyState {
    proxy_manager: ProxyManager,
    proxy_template_manager: ProxyTemplateManager,
}
