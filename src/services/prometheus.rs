use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use log::info;


pub fn init_prometheus() -> PrometheusMetrics {
    info!("Initializing Prometheus");
    PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap()
}