# LogForge

Unified infrastructure logging system — Rust ingest/sampling service + Next.js UI, backed by Prometheus, Loki, Tempo, and Grafana.

## Architecture

```
[Your services]
      │  POST /api/ingest
      ▼
┌─────────────────────────────────┐
│  Rust service (logforge:3001)   │
│                                 │
│  BurstSuppression  ← outermost  │
│  TraceAwareSampler              │
│  ProbabilisticSampler           │
│  JSON formatter    ← innermost  │
│                                 │
│  GET /api/logs  (UI query)      │
│  GET /metrics   (Prometheus)    │
└────────┬──────────┬─────────────┘
         │          │
    Loki/OTLP   Prometheus
         │          │
      ┌──▼──────────▼──┐
      │    Grafana      │
      └─────────────────┘
         +
  ┌──────────────┐
  │  Next.js UI  │  (port 3000)
  │  Live stream │
  │  + filters   │
  └──────────────┘
```

## Quick start

```bash
# 1. Clone / unzip this project
cd logforge

# 2. Start everything
docker compose up --build

# 3. Fire test events
chmod +x test-logs.sh && ./test-logs.sh

# 4. Open the UI
open http://localhost:3000

# 5. Open Grafana (auto-login, all datasources pre-wired)
open http://localhost:3003
```

## Services

| Service    | URL                       | Purpose                     |
|------------|---------------------------|-----------------------------|
| Next.js UI | http://localhost:3000     | Live log viewer             |
| Rust API   | http://localhost:3001     | Ingest + query endpoint     |
| Prometheus | http://localhost:9091     | Metrics scraper             |
| Grafana    | http://localhost:3003     | Dashboards (Loki + Tempo)   |
| Loki       | http://localhost:3100     | Log storage                 |
| Tempo      | http://localhost:3200     | Trace storage               |

## Sending logs

```bash
curl -X POST http://localhost:3001/api/ingest \
  -H "Content-Type: application/json" \
  -d '{
    "ts":         "2026-04-06T10:00:00Z",
    "level":      "ERROR",
    "service":    "api-gateway",
    "msg":        "upstream timeout",
    "path":       "/api/orders",
    "status":     504,
    "latency_ms": 5003,
    "trace_id":   "4bf92f3577b34da6"
  }'
```

## Querying logs

```bash
# All errors
curl "http://localhost:3001/api/logs?level=ERROR&limit=50"

# By service + full-text search
curl "http://localhost:3001/api/logs?service=payment&search=timeout"
```

## Sampling rules (configured in main.rs)

| Layer               | Rule                                                         |
|---------------------|--------------------------------------------------------------|
| BurstSuppression    | Max 5 identical events per 60s window; summary warn emitted  |
| TraceAwareSampler   | 10% of traces kept; all spans in a kept trace are kept       |
| ProbabilisticSampler| 2xx: 1% · 4xx: 50% · 5xx: 100% · ERROR/WARN: always kept   |

## Development (without Docker)

```bash
# Rust service
cd rust-service
RUST_LOG=debug cargo run

# Next.js UI (separate terminal)
cd nextjs-ui
npm install
npm run dev
```

## Tuning

Edit `src/main.rs` and adjust the layer constructors:

```rust
.with(BurstSuppressionLayer::new(
    60,   // window_secs
    5,    // max_per_window
))
.with(TraceAwareSamplerLayer::new(
    0.10, // 10% of traces
))
.with(ProbabilisticSamplerLayer::new(
    0.01, // 2xx: 1%
    0.50, // 4xx: 50%
    1.0,  // 5xx: 100%
))
```
