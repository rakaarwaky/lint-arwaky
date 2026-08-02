// PURPOSE: WatchCommandsSurface — CLI surface for file watching with auto-lint on changes
// Creates a WatchConfig from the given path, sets up Ctrl+C signal handling,
// and delegates to IWatchAggregate.run() which blocks until interrupted.
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use shared::common::{ExitCode, FilePath};
use shared::file_watch::{IWatchAggregate, WatchConfig};

pub fn handle_watch(watch_aggregate: Arc<dyn IWatchAggregate>, path: Option<FilePath>) -> ExitCode {
    let root = match path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let config = WatchConfig::from_path(root);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    if let Err(e) = ctrlc::set_handler(move || {
        eprintln!("\nStopping watcher...");
        r.store(false, Ordering::SeqCst);
    }) {
        eprintln!("[error] failed to set Ctrl+C handler: {}", e);
        return ExitCode::RUNTIME_ERROR;
    }

    watch_aggregate.run(config, running)
}
