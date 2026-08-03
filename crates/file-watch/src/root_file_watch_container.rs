// PURPOSE: FileWatchContainer — wiring for file-watch feature (root layer, wiring only)

use std::sync::Arc;

use crate::agent_watch_orchestrator::WatchOrchestrator;
use crate::capabilities_change_analyzer::ChangeAnalyzer;
use crate::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::IWatchAggregate;
use shared::file_watch::IWatchProviderProtocol;
use shared::quality_rules::ICodeAnalysisAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileWatchContainer {
    provider: Arc<dyn IWatchProviderProtocol>,
    analyzer: Arc<ChangeAnalyzer>,
}

// ─── Block 2: Wiring & Factory ────────────────────────────

impl FileWatchContainer {
    pub fn new() -> Self {
        let provider: Arc<dyn IWatchProviderProtocol> = Arc::new(NotifyWatchProvider::new());
        let analyzer: Arc<ChangeAnalyzer> = Arc::new(ChangeAnalyzer::new());
        Self { provider, analyzer }
    }

    pub fn provider(&self) -> Arc<dyn IWatchProviderProtocol> {
        self.provider.clone()
    }

    pub fn aggregate(&self, linter: Arc<dyn ICodeAnalysisAggregate>) -> Arc<dyn IWatchAggregate> {
        Arc::new(WatchOrchestrator::new(
            self.provider(),
            self.analyzer.clone(),
            linter,
        ))
    }
}

impl Default for FileWatchContainer {
    fn default() -> Self {
        Self::new()
    }
}
