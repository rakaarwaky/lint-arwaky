// PURPOSE: WatchEvent — value object representing a filesystem change event
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WatchEventKind {
    Created,
    Modified,
    Removed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WatchEvent {
    pub path: String,
    pub kind: WatchEventKind,
    pub timestamp_ms: u64,
}

impl WatchEvent {
    pub fn new(path: String, kind: WatchEventKind) -> Self {
        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        Self {
            path,
            kind,
            timestamp_ms,
        }
    }
}
