use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warn  => write!(f, "WARN"),
            LogLevel::Info  => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
        }
    }
}

/// The canonical structured log event accepted by /api/ingest
/// and stored in AppState.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub ts:         DateTime<Utc>,
    pub level:      LogLevel,
    pub service:    String,
    pub host:       Option<String>,
    pub env:        Option<String>,
    pub trace_id:   Option<String>,
    pub span_id:    Option<String>,
    pub msg:        String,
    pub path:       Option<String>,
    pub method:     Option<String>,
    pub status:     Option<u16>,
    pub latency_ms: Option<u64>,
    pub user_id:    Option<String>,
    #[serde(default)]
    pub slow:       bool,
    #[serde(default)]
    pub sampled:    bool,
    #[serde(flatten)]
    pub extra:      serde_json::Map<String, serde_json::Value>,
}

/// Query params for GET /api/logs
#[derive(Debug, Deserialize)]
pub struct LogQuery {
    pub level:   Option<String>,
    pub service: Option<String>,
    pub search:  Option<String>,
    pub limit:   Option<usize>,
}
