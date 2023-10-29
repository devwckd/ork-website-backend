mod clients;
mod consts;
mod domains;
mod extractors;
mod managers;
mod repositories;
mod routes;

use crate::managers::bridge::BridgeManager;
use crate::managers::organization::OrganizationManager;
use crate::managers::organization_member::OrganizationMemberManager;
use crate::managers::proxy::ProxyManager;
use crate::managers::proxy_template::ProxyTemplateManager;
use crate::managers::region::RegionManager;
use crate::managers::region_connection::RegionConnectionManager;
use crate::managers::session::SessionManager;
use crate::managers::user::UserManager;
use crate::repositories::bridge::BridgeRepository;
use crate::repositories::organization::OrganizationRepository;
use crate::repositories::organization_member::OrganizationMemberRepository;
use crate::repositories::proxy::ProxyRepository;
use crate::repositories::proxy_template::ProxyTemplateRepository;
use crate::repositories::regions::RegionRepository;
use crate::repositories::session::SessionRepository;
use crate::repositories::user::UserRepository;
use axum::http::{HeaderName, HeaderValue};
use axum::Extension;
use std::net::SocketAddr;
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::info;

#[tokio::main]
async fn main() {
    init_logging();

    let address: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    let pg_pool = create_pg_pool().await;

    let bridge_repository = BridgeRepository::new(pg_pool.clone());
    let organization_repository = OrganizationRepository::new(pg_pool.clone());
    let organization_member_repository = OrganizationMemberRepository::new(pg_pool.clone());
    let proxy_repository = ProxyRepository::new(pg_pool.clone());
    let proxy_template_repository = ProxyTemplateRepository::new(pg_pool.clone());
    let region_repository = RegionRepository::new(pg_pool.clone());
    let user_repository = UserRepository::new(pg_pool.clone());
    let session_repository = SessionRepository::new(pg_pool.clone());

    let region_manager = RegionManager::new(region_repository.clone());
    let region_connection_manager = RegionConnectionManager::new(region_manager.clone()).await;
    let bridge_manager =
        BridgeManager::new(bridge_repository.clone(), region_connection_manager.clone());
    let organization_manager = OrganizationManager::new(
        region_connection_manager.clone(),
        organization_repository.clone(),
    );
    let organization_member_manager =
        OrganizationMemberManager::new(organization_member_repository.clone());
    let proxy_manager =
        ProxyManager::new(region_connection_manager.clone(), proxy_repository.clone());
    let proxy_template_manager = ProxyTemplateManager::new(proxy_template_repository.clone());
    let user_manager = UserManager::new(user_repository.clone());
    let session_manager = SessionManager::new(session_repository.clone());

    let router = axum::Router::new()
        .nest(
            "/auth",
            routes::auth::router(user_manager.clone(), session_manager.clone()),
        )
        .nest(
            "/organizations",
            routes::organization::router(
                organization_manager.clone(),
                organization_member_manager.clone(),
                region_manager.clone(),
            )
            .nest(
                "/:org_id/members",
                routes::organization_member::router(organization_member_manager.clone()),
            )
            .nest(
                "/:org_id/proxies",
                routes::proxy::router(proxy_manager.clone(), proxy_template_manager.clone()),
            )
            .nest(
                "/:org_id/proxy-templates",
                routes::proxy_template::router(proxy_template_manager.clone()),
            )
            .nest(
                "/:org_id/bridges",
                routes::bridge::router(bridge_manager.clone()),
            ),
        )
        .nest("/regions", routes::region::router(region_manager.clone()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .layer(
            CorsLayer::new()
                .allow_headers(AllowHeaders::list(vec![HeaderName::from_static(
                    "content-type",
                )]))
                .allow_credentials(true)
                // .allow_methods(Any)
                .allow_origin(AllowOrigin::exact(HeaderValue::from_static(
                    "http://localhost:5173",
                ))),
        )
        .layer(Extension(session_manager.clone()))
        .layer(Extension(organization_manager.clone()))
        .layer(Extension(organization_member_manager.clone()));

    info!("binding on {}", &address);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn init_logging() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

async fn create_pg_pool() -> sqlx::PgPool {
    let pg_pool = sqlx::PgPool::connect("postgres://postgres:secret@localhost:5432/backend")
        .await
        .unwrap();

    sqlx::migrate!().run(&pg_pool).await.unwrap();

    pg_pool
}
