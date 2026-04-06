use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use serde_json::json;

use crate::{
    schema::{LogEvent, LogQuery},
    state::AppState,
};

// ── GET /health ──────────────────────────────────────────────────────────────

pub async fn health_handler() -> impl IntoResponse {
    Json(json!({ "status": "ok", "ts": Utc::now() }))
}

// ── GET /metrics (Prometheus scrape endpoint) ────────────────────────────────

pub async fn metrics_handler() -> impl IntoResponse {
    match metrics_exporter_prometheus::PrometheusBuilder::new().build() {
        Ok(_) => (StatusCode::OK, "# metrics endpoint\n"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "# error\n"),
    }
}

// ── POST /api/ingest ─────────────────────────────────────────────────────────

pub async fn ingest_handler(
    State(state): State<AppState>,
    Json(mut event): Json<LogEvent>,
) -> impl IntoResponse {
    // Mark slow requests (>1000ms as a simple default)
    if event.latency_ms.unwrap_or(0) > 1000 {
        event.slow = true;
    }

    // Emit a tracing event so our sampling layers can inspect it
    match event.level {
        crate::schema::LogLevel::Error => tracing::error!(
            service = %event.service,
            msg     = %event.msg,
            status  = ?event.status,
            slow    = event.slow,
            trace_id = ?event.trace_id,
        ),
        crate::schema::LogLevel::Warn => tracing::warn!(
            service = %event.service,
            msg     = %event.msg,
            status  = ?event.status,
        ),
        _ => tracing::info!(
            service = %event.service,
            msg     = %event.msg,
            status  = ?event.status,
        ),
    }

    // Update Prometheus counters
    metrics::counter!("logforge_events_ingested_total",
        "level"   => event.level.to_string(),
        "service" => event.service.clone(),
    ).increment(1);

    if event.slow {
        metrics::counter!("logforge_slow_requests_total",
            "service" => event.service.clone(),
        ).increment(1);
    }

    state.push(event);
    StatusCode::ACCEPTED
}

// ── GET /api/logs ─────────────────────────────────────────────────────────────

pub async fn logs_handler(
    State(state): State<AppState>,
    Query(q): Query<LogQuery>,
) -> impl IntoResponse {
    let limit = q.limit.unwrap_or(200).min(1000);
    let events = state.query(
        q.level.as_deref(),
        q.service.as_deref(),
        q.search.as_deref(),
        limit,
    );
    Json(json!({ "count": events.len(), "events": events }))
}
