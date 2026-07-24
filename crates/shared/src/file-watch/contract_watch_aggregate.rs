// PURPOSE: IWatchAggregate — contract trait for watch operations used by surfaces
use crate::common::taxonomy_common_error::ExitCode;
use crate::file_watch::taxonomy_watch_config_vo::WatchConfig;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

/// Aggregate that runs the file-watch loop.
///
/// Implementations create a file-system watcher, process events through
/// [`IChangeAnalyzerProtocol`], and trigger re-lints when relevant files
/// change. The `running` flag signals when to shut down.
pub trait IWatchAggregate: Send + Sync {
    fn run(&self, config: WatchConfig, running: Arc<AtomicBool>) -> ExitCode;
}
