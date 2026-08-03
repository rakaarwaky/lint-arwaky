// PURPOSE: Watch command — CLI thin wrapper
// Calls dispatcher for watch business logic, only adds CLI output.
use shared::common::{ExitCode, FilePath};
use shared::file_watch::IWatchAggregate;
use std::sync::Arc;
use tracing::{error, info};

pub fn handle_watch(watch_aggregate: Arc<dyn IWatchAggregate>, path: Option<FilePath>) -> ExitCode {
    let on_stop: Arc<dyn Fn() + Send + Sync> = Arc::new(|| {
        info!("stopping watcher");
    });
    match dispatcher::surface_watch_action::handle_watch(watch_aggregate, path, on_stop) {
        Ok(()) => ExitCode::OK,
        Err(e) => {
            error!(error = %e, "operation failed");
            ExitCode::RUNTIME_ERROR
        }
    }
}
