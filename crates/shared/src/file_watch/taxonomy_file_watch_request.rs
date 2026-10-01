// PURPOSE: WatchRequest — request payload for the watch aggregate

use crate::taxonomy_file_watch_vo::WatchConfig;
use shared_common::taxonomy_path_vo::FilePath;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub enum WatchRequest {
    /// Start the watch loop and block until the running flag clears.
    Run {
        config: WatchConfig,
        running: Arc<AtomicBool>,
    },
    /// Ask whether a path has a lintable source extension.
    IsLintable { path: FilePath },
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
