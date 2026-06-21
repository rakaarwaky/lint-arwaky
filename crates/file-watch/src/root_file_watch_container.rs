// PURPOSE: FileWatchContainer — wiring for file-watch feature (root layer, wiring only)
use std::sync::Arc;

use crate::agent_watch_orchestrator::WatchOrchestrator;
use crate::capabilities_change_analyzer::ChangeAnalyzer;
use crate::infrastructure_notify_provider::NotifyWatchProvider;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;
use shared::file_watch::contract_provider_port::IWatchProviderPort;

pub struct FileWatchContainer {
    provider: Arc<NotifyWatchProvider>,
}

impl FileWatchContainer {
    pub fn new() -> Self {
        Self {
            provider: Arc::new(NotifyWatchProvider::new()),
        }
    }

    pub fn provider(&self) -> Arc<dyn IWatchProviderPort> {
        self.provider.clone()
    }

    pub fn orchestrator(&self, linter: Arc<dyn IArchLintProtocol>) -> Arc<WatchOrchestrator> {
        Arc::new(WatchOrchestrator::new(self.provider(), linter))
    }
}

impl Default for FileWatchContainer {
    fn default() -> Self {
        Self::new()
    }
}
