use shared::file_watch::taxonomy_watch_event_vo::WatchEvent;
use std::collections::HashMap;

// PURPOSE: ChangeAnalyzer — deduplicates and batches watch events for lint
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ChangeAnalyzer;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IChangeAnalyzerProtocol for ChangeAnalyzer {
    fn analyze(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent> {
        let mut deduped: HashMap<String, WatchEvent> = HashMap::new();
        for event in events {
            deduped.insert(event.path.clone(), event);
        }
        deduped.into_values().collect()
    }

    fn is_lintable(path: &str) -> bool {
        let lintable_exts = [
            ".rs", ".py", ".js", ".ts", ".tsx", ".jsx", ".mjs", ".cjs", ".json", ".css", ".md",
            ".toml", ".yaml", ".yml",
        ];
        lintable_exts.iter().any(|ext| path.ends_with(ext))
    }

    fn filter_lintable(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent> {
        events
            .into_iter()
            .filter(|e| Self::is_lintable(&e.path))
            .collect()
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

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

