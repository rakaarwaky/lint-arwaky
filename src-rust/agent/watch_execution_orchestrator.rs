use crate::contract::{DirectoryWatchAggregate, WatchExecutionOrchestratorAggregate, IJobRegistryPort};
use crate::taxonomy::{FilePath, GovernanceReport, WatchResult};
use crate::infrastructure::MemoryJobRegistryAdapter;
use std::collections::HashMap;
use std::sync::OnceLock;

static REGISTRY: OnceLock<MemoryJobRegistryAdapter> = OnceLock::new();

pub struct WatchExecutionOrchestrator;

impl WatchExecutionOrchestratorAggregate for WatchExecutionOrchestrator {
    fn root_path(&self) -> Option<&FilePath> {
        None
    }
    fn job_registry(&self) -> &dyn IJobRegistryPort {
        REGISTRY.get_or_init(MemoryJobRegistryAdapter::new)
    }
}

impl WatchExecutionOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub fn is_available(&self) -> bool {
        // Check if the watchdog library is available for file watching
        // In Rust, we'd check for `notify` crate
        true
    }

    pub async fn execute(&self, _request: &DirectoryWatchAggregate) -> WatchResult {
        // Initial execution for watch mode
        WatchResult {
            file: FilePath::new(".").unwrap(),
            report: GovernanceReport::default(),
        }
    }

    pub fn process_event(&self, file_path: &FilePath) -> HashMap<String, serde_json::Value> {
        // Process a file change event
        let mut result = std::collections::HashMap::new();
        result.insert("file".to_string(), serde_json::json!(file_path.value));
        result.insert("score".to_string(), serde_json::json!(0.0));
        result.insert("is_passing".to_string(), serde_json::json!(false));
        result
    }
}
