use super::*;

/// ExternalMetricInfo describes a metric.
///
#[derive(Debug)]
pub struct ExternalMetricInfo {
    pub metric: String,
}

/// ExternalMetricsProvider is a source of external metrics.
/// Metric is normally identified by a name and a set of labels/tags. It is up to a specific
/// implementation how to translate metricSelector to a filter for metric values.
/// Namespace can be used by the implemetation for metric identification, access control or ignored.
pub trait ExternalMetricsProvider {
    type Metric: ExternalMetric;
    type MetricSelector: labels::Selector;
    type Err;

    fn get_external_metric(
        &self,
        namespace: impl AsRef<str>,
        metric_selector: Self::MetricSelector,
        info: ExternalMetricInfo,
    ) -> Result<external_metricsv1::ExternalMetricValueList<Self::Metric>, Self::Err>;

    fn list_all_external_metrics(&self) -> Vec<ExternalMetricInfo>;
}
