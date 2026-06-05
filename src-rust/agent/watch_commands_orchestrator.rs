// watch_commands_orchestrator — Implementation of WatchCommandsAggregate (Agent Logic).
use crate::contract::watch_commands_aggregate::WatchCommandsAggregate;
use crate::taxonomy::source_path_vo::FilePath;

use async_trait::async_trait;

pub struct WatchCommandsOrchestrator;

#[async_trait]
impl WatchCommandsAggregate for WatchCommandsOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }
    async fn watch(&self, path: &FilePath) {
        self.watch_old(path);
    }
}

impl WatchCommandsOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub fn watch_old(&self, _path: &FilePath) {
        // The watch command is inherently interactive/blocking at the surface.
        // The orchestrator provides the logic execution for changes.
        // Logic is handled by WatchExecutionOrchestrator.
    }
}
