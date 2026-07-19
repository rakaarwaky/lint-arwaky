// PURPOSE: WatchCommandsSurface — CLI surface for file watching with auto-lint on changes
//
// Creates a WatchConfig from the given path, sets up Ctrl+C signal handling,
// and delegates to IWatchAggregate.run() which blocks until interrupted.
//
// The atomic `running` flag coordinates graceful shutdown on Ctrl+C.
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

pub struct WatchCommandsSurface {}

impl Default for WatchCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl WatchCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn handle_watch(watch_aggregate: Arc<dyn IWatchAggregate>, path: Option<String>) -> ExitCode {
    let root = match path {
        Some(p) => p,
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
        return ExitCode::FAILURE;
    }

    watch_aggregate.run(config, running)
}
