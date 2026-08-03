// PURPOSE: WatchCommandsSurface — file watching business logic, no formatting.
// Creates a WatchConfig from the given path, sets up Ctrl+C signal handling,
// and delegates to IWatchAggregate.run() which blocks until interrupted.
// The `on_stop` callback (provided by the CLI surface) is invoked on Ctrl+C
// so the stop message is printed by the caller, not by this crate.
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use shared::common::ExitCode;
use shared::common::FilePath;
use shared::file_watch::{IWatchAggregate, WatchConfig};

pub fn handle_watch(
    watch_aggregate: Arc<dyn IWatchAggregate>,
    path: Option<FilePath>,
    on_stop: Arc<dyn Fn() + Send + Sync>,
) -> Result<(), String> {
    let root = match path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    let config = WatchConfig::from_path(root);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    if let Err(e) = ctrlc::set_handler(move || {
        on_stop();
        r.store(false, Ordering::SeqCst);
    }) {
        return Err(format!("[error] failed to set Ctrl+C handler: {e}"));
    }

    let code = watch_aggregate.run(config, running);
    if code == ExitCode::OK {
        Ok(())
    } else {
        Err("watch session failed".to_string())
    }
}
