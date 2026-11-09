// PURPOSE: Watch command — CLI thin wrapper
// Calls dispatcher for watch business logic, only adds CLI output.
use shared_common::{ExitCode, FilePath};
use shared_file_watch::{IWatchAggregate, WatchOutputEvent};
use std::sync::Arc;
use tracing::error;

pub fn format_watch_output(event: WatchOutputEvent) -> (bool, String) {
    match event {
        WatchOutputEvent::Started { path } => (false, format!("Watching {path} — press Ctrl+C to stop")),
        WatchOutputEvent::InitialScanComplete { violations, score } => (
            false,
            format!("Initial scan complete: {violations} violations, score {score:.1}"),
        ),
        WatchOutputEvent::FileLinted { path, succeeded: true } => (false, format!("Linted {path}")),
        WatchOutputEvent::FileLinted { path, succeeded: false } => (true, format!("Lint failed for {path}")),
        WatchOutputEvent::Stopped => (false, "Watcher stopped".to_string()),
    }
}

pub fn handle_watch(watch_aggregate: Arc<dyn IWatchAggregate>, path: Option<FilePath>) -> ExitCode {
    let on_stop: Arc<dyn Fn() + Send + Sync> = Arc::new(|| {
        println!("Stopping watcher...");
    });
    let on_output: Arc<dyn Fn(WatchOutputEvent) + Send + Sync> = Arc::new(|event| {
        let (is_error, message) = format_watch_output(event);
        if is_error {
            eprintln!("{message}");
        } else {
            println!("{message}");
        }
    });
    match dispatcher::surface_watch_action::handle_watch_with_output(
        watch_aggregate,
        path,
        on_stop,
        on_output,
    ) {
        Ok(()) => ExitCode::OK,
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}

pub struct _Aes303Placeholder {
    _placeholder: (),
}
