use crate::domains::proxy_template::{ProxyTemplate, ProxyTemplateResult};
use crate::repositories::proxy_template::ProxyTemplateRepository;
use sqlx::{query, query_as};
use uuid::Uuid;

#[derive(Clone)]
pub struct ProxyTemplateManager {
    proxy_template_repository: ProxyTemplateRepository,
}

impl ProxyTemplateManager {
    pub fn new(proxy_template_repository: ProxyTemplateRepository) -> Self {
        Self {
            proxy_template_repository,
        }
    }

    pub async fn list(&self, organization_id: &Uuid) -> ProxyTemplateResult<Vec<ProxyTemplate>> {
        self.proxy_template_repository.list(organization_id).await
    }

    pub async fn find_by_slug(
        &self,
        organization_id: &Uuid,
        slug: &String,
    ) -> ProxyTemplateResult<ProxyTemplate> {
        self.proxy_template_repository
            .find_by_slug(organization_id, slug)
            .await
    }

    pub async fn insert(
        &self,
        organization_id: &Uuid,
        proxy_template: &ProxyTemplate,
    ) -> ProxyTemplateResult<()> {
        self.proxy_template_repository
            .insert(organization_id, proxy_template)
            .await
    }
}
