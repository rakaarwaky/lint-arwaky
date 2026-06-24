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
        let timestamp_ms = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        {
            Ok(d) => d.as_millis() as u64,
            Err(_) => 0,
        };
        Self {
            path,
            kind,
            timestamp_ms,
        }
    }
}
