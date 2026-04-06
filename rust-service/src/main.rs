mod layers;
mod metrics;
mod routes;
mod state;
mod schema;

use anyhow::Result;
use axum::{Router, routing::get, routing::post};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::layers::{
    burst_suppression::BurstSuppressionLayer,
    probabilistic::ProbabilisticSamplerLayer,
    trace_aware::TraceAwareSamplerLayer,
};
use crate::metrics::setup_metrics;
use crate::routes::{ingest_handler, logs_handler, health_handler, metrics_handler};
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // ── 1. Metrics (Prometheus) ──────────────────────────────────────────────
    setup_metrics()?;

    // ── 2. Tracing pipeline ───────────────────────────────────────────────────
    //   Layers run bottom-up (last .with() = outermost = first to see events).
    //   Order: BurstSuppression → TraceAware → Probabilistic → JSON formatter
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(true)
        .with_target(true);

    tracing_subscriber::registry()
        .with(env_filter)
        // Outermost (checked first) — cheapest
        .with(BurstSuppressionLayer::new(60, 5))
        // Middle — trace-coherent sampling
        .with(TraceAwareSamplerLayer::new(0.10))
        // Inner — probabilistic per-status sampling
        .with(ProbabilisticSamplerLayer::new(0.01, 0.50, 1.0))
        // Formatter (innermost — receives only kept events)
        .with(fmt_layer)
        .init();

    // ── 3. App state ─────────────────────────────────────────────────────────
    let state = AppState::new();

    // ── 4. Router ────────────────────────────────────────────────────────────
    let app = Router::new()
        .route("/health",       get(health_handler))
        .route("/metrics",      get(metrics_handler))
        .route("/api/ingest",   post(ingest_handler))
        .route("/api/logs",     get(logs_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!(addr = %addr, "logforge starting");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
