// PURPOSE: FileWatchContainer — wiring for file-watch feature (root layer, wiring only)

use std::sync::Arc;

use crate::agent_watch_orchestrator::WatchOrchestrator;
use crate::capabilities_change_filter::ChangeFilter;
use crate::capabilities_change_lint::ChangeLintHandler;
use crate::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::IWatchAggregate;
use shared::file_watch::contract_watch_protocol::{
    IChangeFilterProtocol, IChangeLintProtocol, IWatchLifecycleProtocol,
};
use shared::quality_rules::ICodeAnalysisAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileWatchContainer {
    lifecycle: Arc<dyn IWatchLifecycleProtocol>,
    filter: Arc<dyn IChangeFilterProtocol>,
}

// ─── Block 2: Wiring & Factory ────────────────────────────

impl FileWatchContainer {
    pub fn new() -> Self {
        let provider = Arc::new(NotifyWatchProvider::new());
        let analyzer = Arc::new(ChangeFilter::new());
        Self {
            lifecycle: provider,
            filter: analyzer,
        }
    }

    /// The FR-FileWatch-001 lifecycle capability.
    pub fn lifecycle(&self) -> Arc<dyn IWatchLifecycleProtocol> {
        self.lifecycle.clone()
    }

    /// The FR-FileWatch-002 filter capability.
    pub fn filter(&self) -> Arc<dyn IChangeFilterProtocol> {
        self.filter.clone()
    }

    pub fn aggregate(&self, linter: Arc<dyn ICodeAnalysisAggregate>) -> Arc<dyn IWatchAggregate> {
        let lint: Arc<dyn IChangeLintProtocol> = Arc::new(ChangeLintHandler::new(linter.clone()));
        Arc::new(WatchOrchestrator::new(
            self.lifecycle(),
            self.filter(),
            lint,
            linter,
        ))
    }
}

impl Default for FileWatchContainer {
    fn default() -> Self {
        Self::new()
    }
}
