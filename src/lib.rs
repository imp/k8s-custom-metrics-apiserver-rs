#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts)]
#![warn(unused)]
#![deny(warnings)]

use std::fmt;
use std::str;

// use k8s_openapi as k8s;

// use k8s::apimachinery::pkg::runtime;
use k8s_metrics::custom_metrics::v1beta2 as custom_metricsv1;
use k8s_metrics::custom_metrics::CustomMetric;
use k8s_metrics::external_metrics::v1beta1 as external_metricsv1;
use k8s_metrics::external_metrics::ExternalMetric;

use pkg::labels;
use pkg::runtime::schema;
use pkg::types;

pub use custom::CustomMetricInfo;
pub use custom::CustomMetricsProvider;
pub use external::ExternalMetricInfo;
pub use external::ExternalMetricsProvider;

mod custom;
mod external;
pub mod pkg;
