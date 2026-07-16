// PURPOSE: ChangeAnalyzer — deduplicates and batches watch events for lint
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::taxonomy_watch_event_vo::WatchEvent;
use std::collections::HashMap;

pub struct ChangeAnalyzer;

impl Default for ChangeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ChangeAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

impl IChangeAnalyzerProtocol for ChangeAnalyzer {
    fn analyze(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent> {
        let mut deduped: HashMap<String, WatchEvent> = HashMap::new();
        for event in events {
            deduped.insert(event.path.clone(), event);
        }
        deduped.into_values().collect()
    }

    fn is_lintable(&self, path: &str) -> bool {
        let lintable_exts = [
            ".rs", ".py", ".js", ".ts", ".tsx", ".jsx", ".mjs", ".cjs", ".json", ".css", ".md",
            ".toml", ".yaml", ".yml",
        ];
        lintable_exts.iter().any(|ext| path.ends_with(ext))
    }

    fn filter_lintable(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent> {
        events
            .into_iter()
            .filter(|e| self.is_lintable(&e.path))
            .collect()
    }
}
