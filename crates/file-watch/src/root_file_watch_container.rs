// PURPOSE: FileWatchContainer — wiring for file-watch feature (root layer, wiring only)
use std::sync::Arc;

use crate::agent_watch_orchestrator::WatchOrchestrator;
use crate::capabilities_change_analyzer::ChangeAnalyzer;
use crate::capabilities_notify_provider::NotifyWatchProvider;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::file_watch::contract_provider_protocol::IWatchProviderProtocol;

pub struct FileWatchContainer {
    provider: Arc<dyn IWatchProviderProtocol>,
}

impl FileWatchContainer {
    pub fn new() -> Self {
        let provider: Arc<dyn IWatchProviderProtocol> = Arc::new(NotifyWatchProvider::new());
        Self { provider }
    }

    pub fn provider(&self) -> Arc<dyn IWatchProviderProtocol> {
        self.provider.clone()
    }

    pub fn orchestrator(&self, linter: Arc<dyn ICodeAnalysisAggregate>) -> Arc<WatchOrchestrator> {
        let _wire_cap = ChangeAnalyzer::new();
        Arc::new(WatchOrchestrator::new(self.provider(), linter))
    }
}

impl Default for FileWatchContainer {
    fn default() -> Self {
        Self::new()
    }
}
