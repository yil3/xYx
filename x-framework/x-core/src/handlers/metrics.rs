use anyhow::Context;
use metrics_exporter_prometheus::Matcher;
use metrics_exporter_prometheus::PrometheusBuilder;

use crate::constant::EXPONENTIAL_SECONDS;

pub fn metrics_handle() -> String {
    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full(String::from("http_requests_duration_seconds")),
            *EXPONENTIAL_SECONDS,
        )
        .context("could not setup buckets for metrics, verify matchers are correct")
        .unwrap()
        .install_recorder()
        .context("could not install metrics recorder")
        .unwrap()
        .render()
}
