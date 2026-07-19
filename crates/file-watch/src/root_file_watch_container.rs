// PURPOSE: FileWatchContainer — wiring for file-watch feature (root layer, wiring only)
use std::sync::Arc;

use crate::agent_watch_orchestrator::WatchOrchestrator;
use crate::capabilities_change_analyzer::ChangeAnalyzer;
use crate::infrastructure_notify_provider::NotifyWatchProvider;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::contract_provider_port::IWatchProviderPort;

// Block 1: struct Definition
pub struct FileWatchContainer {
    provider: Arc<NotifyWatchProvider>,
}

// ─── Block 2: Public Contract ─────────────────────────────
// (No trait impl — root container is wiring only)

// Block 3: constructors & public API
impl FileWatchContainer {
    pub fn new() -> Self {
        Self {
            provider: Arc::new(NotifyWatchProvider::new()),
        }
    }

    pub fn provider(&self) -> Arc<dyn IWatchProviderPort> {
        self.provider.clone()
    }

    pub fn orchestrator(&self, linter: Arc<dyn ICodeAnalysisAggregate>) -> Arc<WatchOrchestrator> {
        let change_analyzer: Arc<dyn IChangeAnalyzerProtocol> = Arc::new(ChangeAnalyzer::new());
        Arc::new(WatchOrchestrator::new(
            self.provider(),
            linter,
            change_analyzer,
        ))
    }
}

impl Default for FileWatchContainer {
    fn default() -> Self {
        Self::new()
    }
}
