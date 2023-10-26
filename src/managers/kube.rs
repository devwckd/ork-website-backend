use crate::consts::AsNamespaceName;
use crate::domains::organization::Organization;
use crate::domains::proxy::Proxy;
use crate::domains::proxy_template::ProxyTemplate;
use crate::managers::region_connection::RegionConnectionManager;
use k8s_openapi::api::core::v1::{
    Container, Namespace, Pod, PodSpec, Service, ServicePort, ServiceSpec,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use kube::api::PostParams;
use kube::Api;
use maplit::btreemap;

#[derive(Clone)]
pub struct KubeManager {
    region_connection_manager: RegionConnectionManager,
}

impl KubeManager {
    pub fn new(region_connection_manager: RegionConnectionManager) -> Self {
        Self {
            region_connection_manager,
        }
    }

    pub async fn on_organization_creation(&self, organization: &Organization) {
        let namespaces: Api<Namespace> = Api::all(
            self.region_connection_manager
                .find_kube_client_by_id(&organization.region_id)
                .await
                .unwrap(),
        );

        namespaces
            .create(
                &PostParams::default(),
                &Namespace {
                    metadata: ObjectMeta {
                        name: Some(organization.slug.as_namespace_name()),
                        ..Default::default()
                    },
                    spec: None,
                    status: None,
                },
            )
            .await
            .unwrap();

        // TODO: HANDLE ERRORS
    }

    pub async fn on_proxy_creation(
        &self,
        organization: &Organization,
        template: &ProxyTemplate,
        proxy: &Proxy,
    ) {
        let client = self
            .region_connection_manager
            .find_kube_client_by_id(&organization.region_id)
            .await
            .unwrap();
        let pods: Api<Pod> =
            Api::namespaced(client.clone(), &organization.slug.as_namespace_name());

        pods.create(
            &PostParams::default(),
            &Pod {
                metadata: ObjectMeta {
                    name: Some(proxy.slug.clone()),
                    labels: Some(btreemap! {
                        "kube.ork.gg/proxies".to_string() => proxy.id.to_string()
                    }),
                    ..Default::default()
                },
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: proxy.slug.clone(),
                        image: Some(template.image.clone()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        let services: Api<Service> =
            Api::namespaced(client, &organization.slug.as_namespace_name());

        services
            .create(
                &PostParams::default(),
                &Service {
                    metadata: ObjectMeta {
                        name: Some(format!("{}-svc", &proxy.slug)),
                        ..Default::default()
                    },
                    spec: Some(ServiceSpec {
                        selector: Some(btreemap! {
                            "kube.ork.gg/proxies".to_string() => proxy.id.to_string()
                        }),
                        type_: Some("NodePort".to_string()),
                        ports: Some(vec![ServicePort {
                            protocol: Some("TCP".to_string()),
                            port: 25565,
                            target_port: Some(IntOrString::Int(25577)),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    }),
                    status: None,
                },
            )
            .await
            .unwrap();

        // TODO: HANDLE ERRORS
    }
}
