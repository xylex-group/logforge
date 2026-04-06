use parking_lot::RwLock;
use std::collections::VecDeque;
use std::sync::Arc;

use crate::schema::LogEvent;

const MAX_EVENTS: usize = 10_000;

#[derive(Clone)]
pub struct AppState {
    pub log_store: Arc<RwLock<VecDeque<LogEvent>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            log_store: Arc::new(RwLock::new(VecDeque::with_capacity(MAX_EVENTS))),
        }
    }

    pub fn push(&self, event: LogEvent) {
        let mut store = self.log_store.write();
        if store.len() >= MAX_EVENTS {
            store.pop_front();
        }
        store.push_back(event);
    }

    pub fn query(
        &self,
        level:   Option<&str>,
        service: Option<&str>,
        search:  Option<&str>,
        limit:   usize,
    ) -> Vec<LogEvent> {
        let store = self.log_store.read();
        store
            .iter()
            .rev()
            .filter(|e| {
                level.map_or(true, |l| e.level.to_string().eq_ignore_ascii_case(l))
                    && service.map_or(true, |s| e.service.contains(s))
                    && search.map_or(true, |q| e.msg.contains(q))
            })
            .take(limit)
            .cloned()
            .collect()
    }
}
