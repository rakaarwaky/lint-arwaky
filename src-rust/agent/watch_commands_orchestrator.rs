// watch_commands_orchestrator — Implementation of WatchCommandsAggregate (Agent Logic).
use crate::contract::WatchCommandsAggregate;
use crate::taxonomy::FilePath;

pub struct WatchCommandsOrchestrator;

impl WatchCommandsAggregate for WatchCommandsOrchestrator {}

impl WatchCommandsOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub fn watch(&self, _path: &FilePath) {
        // The watch command is inherently interactive/blocking at the surface.
        // The orchestrator provides the logic execution for changes.
        // Logic is handled by WatchExecutionOrchestrator.
    }
}
