use std::fmt;

use k8s_custom_metrics_apiserver::pkg::types::NamespacedName;
use k8s_custom_metrics_apiserver::CustomMetricInfo;
use k8s_custom_metrics_apiserver::CustomMetricsProvider;
use k8s_metrics::custom_metrics::v1beta2 as cmetricsv1;
use k8s_openapi as k8s;

use k8s::api::core::v1 as corev1;
use k8s::apimachinery::pkg::apis::meta::v1 as metav1;

struct PodContainerCounter {
    client: kube::Client,
}

impl PodContainerCounter {
    async fn new() -> kube::Result<Self> {
        kube::Client::try_default()
            .await
            .map(|client| Self { client })
    }
}

impl CustomMetricsProvider for PodContainerCounter {
    type K = corev1::Pod;
    type Err = u8;

    fn list_all_metrics(&self) -> Vec<CustomMetricInfo> {
        let container_count = CustomMetricInfo::namespaced::<corev1::Pod>("container_count");
        let namespace_count = CustomMetricInfo::cluster::<corev1::Namespace>("namespace_count");
        vec![container_count, namespace_count]
    }

    fn get_metric_by_name(
        &self,
        name: &NamespacedName,
        info: &CustomMetricInfo,
        metric_selector: metav1::LabelSelector,
    ) -> Result<cmetricsv1::MetricValue<Self::K>, Self::Err> {
        todo!()
    }

    fn get_metric_by_selector(
        &self,
        namespace: &str,
        selector: metav1::LabelSelector,
        info: CustomMetricInfo,
        metric_selector: metav1::LabelSelector,
    ) -> Result<cmetricsv1::MetricValueList<Self::K>, Self::Err> {
        todo!()
    }
}

impl fmt::Debug for PodContainerCounter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PodContainerCounter")
            .field("client", &"<kube::Client>")
            .finish()
    }
}

#[tokio::main]
pub async fn main() {
    println!("Simple corev1::Pod container count provider");
    let container_counter = PodContainerCounter::new().await.unwrap();
    println!("{info:?}", info = container_counter.list_all_metrics());
}
