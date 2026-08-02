// PURPOSE: Watch command — CLI thin wrapper
// Calls dispatcher for watch business logic, only adds CLI output
use shared::common::{ExitCode, FilePath};
use shared::file_watch::IWatchAggregate;
use std::sync::Arc;

pub fn handle_watch(watch_aggregate: Arc<dyn IWatchAggregate>, path: Option<FilePath>) -> ExitCode {
    // Delegate to dispatcher
    dispatcher::surface_watch_action::handle_watch(watch_aggregate, path)
}
