use super::*;

#[derive(Debug)]
pub struct CustomMetricInfo {
    pub group_resource: schema::GroupResource,
    pub namespaced: bool,
    pub metric: String,
}

/// CustomMetricsProvider is a source of custom metrics
/// which is able to supply a list of available metrics,
/// as well as metric values themselves on demand.
///
/// Note that group-resources are provided  as GroupResources,
/// not GroupKinds.  This is to allow flexibility on the part
/// of the implementor: implementors do not necessarily need
/// to be aware of all existing kinds and their corresponding
/// REST mappings in order to perform queries.
///
/// For queries that use label selectors, it is up to the
/// implementor to decide how to make use of the label selector --
/// they may wish to query the main Kubernetes API server, or may
/// wish to simply make use of stored information in their TSDB.
///
pub trait CustomMetricsProvider {
    type Metric: CustomMetric;
    type Selector: labels::Selector;
    type MetricSelector: labels::Selector;
    type Err;

    /// ListAllMetrics provides a list of all available metrics at
    /// the current time.  Note that this is not allowed to return
    /// an error, so it is recommended that implementors cache and
    /// periodically update this list, instead of querying every time.
    ///
    fn list_all_metrics(&self) -> Vec<CustomMetricInfo>;

    /// GetMetricByName fetches a particular metric for a particular object.
    /// The namespace will be empty if the metric is root-scoped.
    ///
    fn get_metric_by_name(
        &self,
        name: types::NamespacedName,
        info: CustomMetricInfo,
        metric_selector: Self::MetricSelector,
    ) -> Result<custom_metricsv1::MetricValue<Self::Metric>, Self::Err>;

    /// GetMetricBySelector fetches a particular metric for a set of objects matching
    /// the given label selector.  The namespace will be empty if the metric is root-scoped.
    ///
    fn get_metric_by_selector(
        &self,
        namespace: impl AsRef<str>,
        selector: Self::Selector,
        info: CustomMetricInfo,
        metric_selector: Self::MetricSelector,
    ) -> Result<custom_metricsv1::MetricValueList<Self::Metric>, Self::Err>;
}
