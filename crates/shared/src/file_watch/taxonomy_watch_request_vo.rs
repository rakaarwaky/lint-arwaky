// PURPOSE: WatchRequestVO / WatchResponseVO — aggregate request/response for the watch domain
use crate::common::taxonomy_common_error::ExitCode;
use crate::common::taxonomy_path_vo::FilePath;
use crate::file_watch::taxonomy_watch_config_vo::WatchConfig;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

/// Consumer verb carried by the watch aggregate's single entry point.
pub enum WatchRequest {
    /// Start the watch loop and block until the running flag clears.
    Run {
        config: WatchConfig,
        running: Arc<AtomicBool>,
    },
    /// Ask whether a path has a lintable source extension.
    IsLintable { path: FilePath },
}

/// Result of a watch aggregate request.
pub enum WatchResponse {
    /// Terminal status of a completed run request.
    Run { exit_code: ExitCode },
    /// Answer to an `IsLintable` request.
    IsLintable { lintable: bool },
}

impl WatchRequest {
    pub fn run(config: WatchConfig, running: Arc<AtomicBool>) -> Self {
        Self::Run { config, running }
    }

    pub fn is_lintable(path: &str) -> Self {
        Self::IsLintable {
            path: FilePath::new(path.to_string()).unwrap_or_default(),
        }
    }
}
