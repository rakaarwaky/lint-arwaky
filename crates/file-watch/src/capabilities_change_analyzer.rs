// PURPOSE: ChangeAnalyzer — deduplicates and batches watch events for lint
#[allow(unused_imports)]
use shared::file_watch::contract_provider_port::IWatchProviderPort;
use shared::file_watch::taxonomy_watch_event_vo::WatchEvent;
use std::collections::HashMap;

pub struct ChangeAnalyzer;

impl ChangeAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent> {
        let mut deduped: HashMap<String, WatchEvent> = HashMap::new();
        for event in events {
            deduped.insert(event.path.clone(), event);
        }
        deduped.into_values().collect()
    }

    pub fn is_lintable(path: &str) -> bool {
        let lintable_exts = [".rs", ".py", ".js", ".ts", ".tsx", ".jsx", ".mjs", ".cjs"];
        lintable_exts.iter().any(|ext| path.ends_with(ext))
    }

    pub fn filter_lintable(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent> {
        events
            .into_iter()
            .filter(|e| Self::is_lintable(&e.path))
            .collect()
    }
}

impl Default for ChangeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
