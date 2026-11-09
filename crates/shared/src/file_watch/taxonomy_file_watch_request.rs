// PURPOSE: WatchRequest — request payload for the watch aggregate

use crate::taxonomy_file_watch_vo::WatchConfig;
use shared_common::taxonomy_path_vo::FilePath;
use crate::taxonomy_file_watch_vo::WatchOutputEvent;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub enum WatchRequest {
    /// Start the watch loop and block until the running flag clears.
    Run {
        config: WatchConfig,
        running: Arc<AtomicBool>,
        on_output: Arc<dyn Fn(WatchOutputEvent) + Send + Sync>,
    },
    /// Ask whether a path has a lintable source extension.
    IsLintable { path: FilePath },
}

impl WatchRequest {
    pub fn run(config: WatchConfig, running: Arc<AtomicBool>) -> Self {
        Self::run_with_output(config, running, Arc::new(|_| {}))
    }

    pub fn run_with_output(
        config: WatchConfig,
        running: Arc<AtomicBool>,
        on_output: Arc<dyn Fn(WatchOutputEvent) + Send + Sync>,
    ) -> Self {
        Self::Run {
            config,
            running,
            on_output,
        }
    }

    pub fn is_lintable(path: &str) -> Self {
        Self::IsLintable {
            path: FilePath::new(path.to_string()).unwrap_or_default(),
        }
    }
}
