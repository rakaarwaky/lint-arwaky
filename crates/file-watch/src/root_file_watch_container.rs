// PURPOSE: FileWatchContainer — wiring for file-watch feature (root layer, wiring only)

use std::sync::Arc;

use crate::agent_watch_orchestrator::WatchOrchestrator;
use crate::capabilities_change_analyzer::ChangeAnalyzer;
use crate::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::IWatchAggregate;
use shared::file_watch::contract_watch_protocol::{
    IEventDedupProtocol, ILintableFilterProtocol, IWatchBroadcastProtocol, IWatchShutdownProtocol,
    IWatchStartProtocol,
};
use shared::quality_rules::ICodeAnalysisAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct FileWatchContainer {
    start: Arc<dyn IWatchStartProtocol>,
    broadcast: Arc<dyn IWatchBroadcastProtocol>,
    shutdown: Arc<dyn IWatchShutdownProtocol>,
    filter: Arc<dyn ILintableFilterProtocol>,
    dedup: Arc<dyn IEventDedupProtocol>,
}

// ─── Block 2: Wiring & Factory ────────────────────────────

impl FileWatchContainer {
    pub fn new() -> Self {
        let provider = Arc::new(NotifyWatchProvider::new());
        let analyzer = Arc::new(ChangeAnalyzer::new());
        Self {
            start: provider.clone(),
            broadcast: provider.clone(),
            shutdown: provider,
            filter: analyzer.clone(),
            dedup: analyzer,
        }
    }

    /// The FR-FileWatch-001 watcher-start capability.
    pub fn start(&self) -> Arc<dyn IWatchStartProtocol> {
        self.start.clone()
    }

    /// The FR-FileWatch-002 event broadcast capability.
    pub fn broadcast(&self) -> Arc<dyn IWatchBroadcastProtocol> {
        self.broadcast.clone()
    }

    /// The FR-FileWatch-006 shutdown capability.
    pub fn shutdown(&self) -> Arc<dyn IWatchShutdownProtocol> {
        self.shutdown.clone()
    }

    /// The FR-FileWatch-003 lintable-filter capability.
    pub fn filter(&self) -> Arc<dyn ILintableFilterProtocol> {
        self.filter.clone()
    }

    /// The FR-FileWatch-004 event-dedup capability.
    pub fn dedup(&self) -> Arc<dyn IEventDedupProtocol> {
        self.dedup.clone()
    }

    pub fn aggregate(&self, linter: Arc<dyn ICodeAnalysisAggregate>) -> Arc<dyn IWatchAggregate> {
        Arc::new(WatchOrchestrator::new(
            self.start(),
            self.broadcast(),
            self.shutdown(),
            self.filter(),
            self.dedup(),
            linter,
        ))
    }
}

impl Default for FileWatchContainer {
    fn default() -> Self {
        Self::new()
    }
}
