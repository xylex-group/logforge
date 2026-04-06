use anyhow::Result;
use metrics_exporter_prometheus::PrometheusBuilder;

pub fn setup_metrics() -> Result<()> {
    PrometheusBuilder::new()
        .with_http_listener(([0, 0, 0, 0], 9090))
        .install()?;
    Ok(())
}
