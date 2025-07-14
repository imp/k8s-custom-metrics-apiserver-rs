use std::net;

use anyhow::Context;
use axum::routing::get;

use super::*;

#[derive(Debug)]
pub struct MetricsServer {
    custom: Vec<Box<dyn CustomMetricsProvider>>,
}

impl MetricsServer {
    pub fn add_custom_metrics_provider(self, provider: impl CustomMetricsProvider) -> Self {
        self
    }

    pub async fn start(self, addr: net::SocketAddr) -> anyhow::Result<()> {
        let state = MetricsAggregator::new().await?;
        let app = axum::Router::new()
            .route("/livez", get(livez))
            .route("/readyz", get(readyz))
            .nest(
                "/apis/custom.metrics.k8s.io/v1beta2",
                metrics::custom_metrics(state),
            );

        let config = tls_config(&["localhost"]).await?;

        axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service())
            .await
            .context("server failed")
    }
}

async fn tls_config(names: &[&str]) -> anyhow::Result<axum_server::tls_rustls::RustlsConfig> {
    let names = names.iter().map(ToString::to_string).collect::<Vec<_>>();
    let self_signed = rcgen::generate_simple_self_signed(names)?;
    let cert = self_signed.cert.pem().into();
    let key = self_signed.key_pair.serialize_pem().into();
    axum_server::tls_rustls::RustlsConfig::from_pem(cert, key)
        .await
        .context("Failed to build TLS config")
}

async fn livez() -> &'static str {
    "ok"
}

async fn readyz() -> &'static str {
    "ok"
}
