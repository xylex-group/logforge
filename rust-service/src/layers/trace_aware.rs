use dashmap::DashMap;
use rand::random;
use std::sync::Arc;
use tracing::{Event, Id, Subscriber};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

/// Makes a single keep/drop decision per trace_id and applies it to every
/// event that shares that trace. Errors are always kept regardless.
///
/// Uses a `DashMap` for lock-free concurrent access. A background task
/// should periodically drain stale entries (trace_id → last_seen > TTL).
pub struct TraceAwareSamplerLayer {
    decisions:   Arc<DashMap<String, bool>>,
    sample_rate: f64,
}

impl TraceAwareSamplerLayer {
    pub fn new(sample_rate: f64) -> Self {
        let layer = Self {
            decisions:   Arc::new(DashMap::new()),
            sample_rate,
        };
        // Spawn a cleanup task every 5 minutes
        let decisions = layer.decisions.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(300)).await;
                // Simple strategy: clear entirely if > 50k entries
                if decisions.len() > 50_000 {
                    decisions.clear();
                    tracing::debug!("trace-aware sampler: cleared decision cache");
                }
            }
        });
        layer
    }
}

struct TraceIdVisitor(Option<String>);

impl tracing::field::Visit for TraceIdVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "trace_id" {
            self.0 = Some(value.to_owned());
        }
    }
    fn record_debug(&mut self, _f: &tracing::field::Field, _v: &dyn std::fmt::Debug) {}
}

impl<S> Layer<S> for TraceAwareSamplerLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        _id: &Id,
        _ctx: Context<'_, S>,
    ) {
        let mut visitor = TraceIdVisitor(None);
        attrs.record(&mut visitor);
        if let Some(tid) = visitor.0 {
            // Register decision once, reuse for all subsequent spans/events
            self.decisions
                .entry(tid)
                .or_insert_with(|| random::<f64>() < self.sample_rate);
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // Errors always bypass sampling
        if *event.metadata().level() == tracing::Level::ERROR {
            ctx.event(event);
            return;
        }

        // Try to get trace_id from the event fields
        let mut visitor = TraceIdVisitor(None);
        event.record(&mut visitor);

        let keep = visitor.0
            .as_ref()
            .and_then(|tid| self.decisions.get(tid).map(|d| *d))
            .unwrap_or(true); // unknown trace → keep (conservative)

        if keep {
            ctx.event(event);
        }
    }
}
