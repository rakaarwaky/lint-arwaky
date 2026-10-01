// PURPOSE: file-watch-domain capability contracts (AES102 `_protocol`).
//
// One file for the file-watch feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::taxonomy_file_watch_error::WatchServiceError;
use crate::taxonomy_file_watch_vo::WatchConfig;
use crate::taxonomy_file_watch_vo::WatchEvent;
use shared_common::taxonomy_common_vo::BooleanVO;

/// FR-FileWatch-001: Watch Filesystem Lifecycle.
///
/// Combines start, subscribe, stop, and is_available into a single capability
/// seam: the filesystem watcher lifecycle.
#[async_trait::async_trait]
pub trait IWatchLifecycleProtocol: Send + Sync {
    async fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError>;
    /// Returns true when the watch feature is compiled in for this platform.
    async fn is_available(&self) -> BooleanVO;
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<WatchEvent>;
    async fn stop(&self) -> Result<(), WatchServiceError>;
}

/// FR-FileWatch-002: Filter and Deduplicate Change Events.
///
/// Deduplicates a batch of events by path (last-write-wins) and then filters
/// to lintable extensions only.
pub trait IChangeFilterProtocol: Send + Sync {
    /// Returns true if the path has a lintable source extension.
    fn is_lintable(&self, path: &str) -> bool;
    fn filter_events(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent>;
}

/// FR-FileWatch-003: Run Lint on Changed Files.
pub trait IChangeLintProtocol: Send + Sync {
    fn lint_changed(&self, event: &WatchEvent) -> Result<(), WatchServiceError>;
}
