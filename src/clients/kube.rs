use crate::consts::AsNamespaceName;
use crate::domains::organization::Organization;
use crate::domains::proxy::Proxy;
use crate::domains::proxy_template::ProxyTemplate;
use k8s_openapi::api::core::v1::{
    Container, Namespace, Pod, PodSpec, Service, ServicePort, ServiceSpec,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use kube::api::PostParams;
use kube::Api;
use maplit::btreemap;

#[derive(Clone)]
pub struct KubeWrappedClient {
    client: kube::Client,
}

impl KubeWrappedClient {
    pub fn new(client: kube::Client) -> Self {
        Self { client }
    }

    pub async fn create_organization_namespace(&self, organization: &Organization) {
        let namespaces: Api<Namespace> = Api::all(self.client.clone());

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

    pub async fn create_proxy_pod(
        &self,
        organization: &Organization,
        template: &ProxyTemplate,
        proxy: &Proxy,
    ) {
        let pods: Api<Pod> =
            Api::namespaced(self.client.clone(), &organization.slug.as_namespace_name());

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
            Api::namespaced(self.client.clone(), &organization.slug.as_namespace_name());

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
