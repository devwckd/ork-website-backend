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
use crate::managers::region_connection::RegionConnectionManager;

pub fn router(
    proxy_manager: ProxyManager,
    proxy_template_manager: ProxyTemplateManager,
    region_connection_manager: RegionConnectionManager,
) -> axum::Router {
    let state = ProxyState {
        proxy_manager,
        proxy_template_manager,
        region_connection_manager,
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
        region_connection_manager,
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

    let proxy_slug = data.slug.unwrap_or_else(|| {
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
    });

    let region_id = org_member.org().region_id;
    let (bridge_id, bs_proxy_id) = if template.bridge_id.is_none() {
        (None, None)
    } else {
        let bs_client = region_connection_manager
            .find_bridge_service_client_by_id(&region_id)
            .await
            .unwrap();
        let bs_proxy = bs_client
            .declare_proxy(&ork_bridge_service::domains::proxy::CreateProxyData {
                slug: proxy_slug.clone(),
            })
            .await
            .unwrap();
        (Some(template.bridge_id.unwrap()), Some(bs_proxy.id))
    };

    let proxy = Proxy {
        id: Uuid::new_v4(),
        slug: proxy_slug,
        bridge_id,
        bs_proxy_id,
        template_id: template.id,
    };

    proxy_manager
        .create(org_member.org(), &template, &proxy)
        .await?;

    region_connection_manager
        .find_kube_wrapped_client_by_id(&region_id)
        .await
        .unwrap()
        .create_proxy_pod(org_member.org(), &template, &proxy)
        .await;

    Ok(Json(proxy))
}
#[derive(Clone)]
struct ProxyState {
    proxy_manager: ProxyManager,
    proxy_template_manager: ProxyTemplateManager,
    region_connection_manager: RegionConnectionManager,
}
