use parking_lot::Mutex;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
    time::{Duration, Instant},
};
use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

struct BurstEntry {
    count:      u64,
    first_seen: Instant,
    suppressed: u64,
}

/// Fingerprints each event by (target, level, message prefix) and suppresses
/// repeated events within a sliding time window. Emits a single summary warn
/// when a burst is first detected, so the storm is visible but not deafening.
pub struct BurstSuppressionLayer {
    state:          Arc<Mutex<HashMap<u64, BurstEntry>>>,
    window_secs:    u64,
    max_per_window: u64,
}

impl BurstSuppressionLayer {
    pub fn new(window_secs: u64, max_per_window: u64) -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            window_secs,
            max_per_window,
        }
    }
}

struct MsgVisitor(String);

impl tracing::field::Visit for MsgVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" || field.name() == "msg" {
            // Take first 80 chars as fingerprint prefix
            self.0 = value.chars().take(80).collect();
        }
    }
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" || field.name() == "msg" {
            self.0 = format!("{:?}", value).chars().take(80).collect();
        }
    }
}

fn fingerprint(event: &Event<'_>) -> u64 {
    let mut visitor = MsgVisitor(String::new());
    event.record(&mut visitor);

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    event.metadata().target().hash(&mut hasher);
    event.metadata().level().hash(&mut hasher);
    visitor.0.hash(&mut hasher);
    std::hash::Hasher::finish(&hasher)
}

impl<S: Subscriber> Layer<S> for BurstSuppressionLayer {
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let fp  = fingerprint(event);
        let now = Instant::now();

        let mut state = self.state.lock();
        let entry = state.entry(fp).or_insert_with(|| BurstEntry {
            count:      0,
            first_seen: now,
            suppressed: 0,
        });

        // Reset window if expired
        if now.duration_since(entry.first_seen) > Duration::from_secs(self.window_secs) {
            *entry = BurstEntry { count: 0, first_seen: now, suppressed: 0 };
        }

        entry.count += 1;

        if entry.count <= self.max_per_window {
            drop(state);
            ctx.event(event);
        } else {
            entry.suppressed += 1;
            let suppressed = entry.suppressed;
            drop(state);

            // Emit a single summary warning on first suppression
            if suppressed == 1 {
                tracing::warn!(
                    burst_suppressed = true,
                    target = event.metadata().target(),
                    "burst detected — suppressing duplicate events"
                );
            }
        }
    }
}
