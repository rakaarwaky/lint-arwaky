// PURPOSE: IWatchAggregate — contract trait for watch operations used by surfaces
use crate::common::taxonomy_common_error::ExitCode;
use crate::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use crate::file_watch::contract_provider_protocol::IWatchProviderProtocol;
use crate::file_watch::taxonomy_watch_config_vo::WatchConfig;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

/// Aggregate that runs the file-watch loop.
///
/// Implementations create a file-system watcher, process events through
/// [`IChangeAnalyzerProtocol`], and trigger re-lints when relevant files
/// change. The `running` flag signals when to shut down.
pub trait IWatchAggregate: Send + Sync {
    fn run(&self, config: WatchConfig, running: Arc<AtomicBool>) -> ExitCode;

    /// Access the watch provider for subscribing to file change events.
    fn provider(&self) -> Arc<dyn IWatchProviderProtocol>;

    /// Check if a file path is lintable (has a source extension).
    fn is_lintable(&self, path: &str) -> bool;
}
