// PURPOSE: WatchCommandsSurface — file watching business logic, no formatting.
// Creates a WatchConfig from the given path, sets up Ctrl+C signal handling,
// and delegates to IWatchAggregate.execute() which blocks until interrupted.
// The `on_stop` callback (provided by the CLI surface) is invoked on Ctrl+C
// so the stop message is printed by the caller, not by this crate.
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use shared_common::ExitCode;
use shared_common::FilePath;
use shared_file_watch::{
    IWatchAggregate, WatchConfig, WatchOutputEvent, WatchRequest, WatchResponse,
};

pub fn handle_watch(
    watch_aggregate: Arc<dyn IWatchAggregate>,
    path: Option<FilePath>,
    on_stop: Arc<dyn Fn() + Send + Sync>,
) -> Result<(), String> {
    handle_watch_with_output(watch_aggregate, path, on_stop, Arc::new(|_| {}))
}

pub fn handle_watch_with_output(
    watch_aggregate: Arc<dyn IWatchAggregate>,
    path: Option<FilePath>,
    on_stop: Arc<dyn Fn() + Send + Sync>,
    on_output: Arc<dyn Fn(WatchOutputEvent) + Send + Sync>,
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

    let WatchResponse::Run { exit_code } =
        watch_aggregate.execute(WatchRequest::run_with_output(config, running, on_output))
    else {
        return Err("watch session returned an unexpected response".to_string());
    };

    if exit_code == ExitCode::OK {
        Ok(())
    } else {
        Err("watch session failed".to_string())
    }
}

pub struct _Aes303Placeholder {
    _placeholder: (),
}
