// PURPOSE: file-watch-domain capability contracts (AES102 `_protocol`).
//
// One file for the file-watch feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_common_vo::BooleanVO;
use crate::file_watch::taxonomy_file_watch_error::WatchServiceError;
use crate::file_watch::taxonomy_file_watch_vo::WatchConfig;
use crate::file_watch::taxonomy_file_watch_vo::WatchEvent;

/// FR-FileWatch-001: Start Filesystem Watcher.
#[async_trait::async_trait]
pub trait IWatchStartProtocol: Send + Sync {
    async fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError>;
    /// Returns true when the watch feature is compiled in for this platform.
    async fn is_available(&self) -> BooleanVO;
}

/// FR-FileWatch-002: Receive and Broadcast File Change Events.
#[async_trait::async_trait]
pub trait IWatchBroadcastProtocol: Send + Sync {
    /// Subscribe to the broadcast channel of file-change events.
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<WatchEvent>;
}

/// FR-FileWatch-006: Graceful Shutdown.
#[async_trait::async_trait]
pub trait IWatchShutdownProtocol: Send + Sync {
    async fn stop(&self) -> Result<(), WatchServiceError>;
}

/// FR-FileWatch-003: Filter Lintable Files.
pub trait ILintableFilterProtocol: Send + Sync {
    fn is_lintable(&self, path: &str) -> bool;
    fn filter_lintable(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent>;
}

/// FR-FileWatch-004: Deduplicate Watch Events.
pub trait IEventDedupProtocol: Send + Sync {
    fn dedup_events(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent>;
}

/// FR-FileWatch-005: Run Lint on Changed Files.
pub trait IChangeLintProtocol: Send + Sync {
    fn lint_changed(&self, event: &WatchEvent) -> Result<(), WatchServiceError>;
}
