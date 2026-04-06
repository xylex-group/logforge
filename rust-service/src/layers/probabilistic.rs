use rand::random;
use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

/// Keeps events based on their HTTP status code (read from a field named
/// `status`). Falls back to level-based sampling if no status is present.
///
/// - 2xx: `ok_rate`   (e.g. 0.01 = 1%)
/// - 4xx: `err4_rate` (e.g. 0.50 = 50%)
/// - 5xx+: always kept (`1.0`)
pub struct ProbabilisticSamplerLayer {
    ok_rate:   f64,
    err4_rate: f64,
    err5_rate: f64,
}

impl ProbabilisticSamplerLayer {
    pub fn new(ok_rate: f64, err4_rate: f64, err5_rate: f64) -> Self {
        Self { ok_rate, err4_rate, err5_rate }
    }
}

struct StatusVisitor(Option<u64>);

impl tracing::field::Visit for StatusVisitor {
    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        if field.name() == "status" {
            self.0 = Some(value);
        }
    }
    fn record_debug(&mut self, _field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {}
}

impl<S: Subscriber> Layer<S> for ProbabilisticSamplerLayer {
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let mut visitor = StatusVisitor(None);
        event.record(&mut visitor);

        let rate = match visitor.0 {
            Some(s) if s < 300 => self.ok_rate,
            Some(s) if s < 500 => self.err4_rate,
            Some(_)            => self.err5_rate,
            // No status field: fall back to level
            None => match *event.metadata().level() {
                tracing::Level::ERROR | tracing::Level::WARN => 1.0,
                tracing::Level::INFO  => 0.10,
                _                     => 0.01,
            },
        };

        if random::<f64>() < rate {
            // Forward to the next inner layer
            self.inner_layer_event(event, ctx);
        }
    }
}

// Helper trait to call the inner subscriber
trait InnerLayerEvent<S: Subscriber> {
    fn inner_layer_event(&self, event: &Event<'_>, ctx: Context<'_, S>);
}

impl<S: Subscriber> InnerLayerEvent<S> for ProbabilisticSamplerLayer {
    fn inner_layer_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        ctx.event(event);
    }
}
