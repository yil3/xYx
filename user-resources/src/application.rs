use crate::service_register::ServiceRegister;
use anyhow::Context;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use lazy_static::lazy_static;

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
    static ref EXPONENTIAL_SECONDS: &'static [f64] = &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
}

pub struct Application;

impl Application {
    pub fn serve(port: u32, cors_origin: &str, service_register: ServiceRegister) -> anyhow::Result<()> {

        let recorder_handle = PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Full(String::from("http_requests_duration_seconds")),
                *EXPONENTIAL_SECONDS,
            )
            .context("could not setup buckets for metrics, verify matchers are correct")?
            .install_recorder()
            .context("could not install metrics recorder")?;

        Ok(())
    }
}
