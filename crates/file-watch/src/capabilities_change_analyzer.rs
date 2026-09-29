// PURPOSE: ChangeAnalyzer — ILintableFilterProtocol + IEventDedupProtocol (FR-003 + FR-004)

use std::collections::HashMap;

use shared::file_watch::contract_watch_protocol::{IEventDedupProtocol, ILintableFilterProtocol};
use shared::file_watch::taxonomy_file_watch_vo::WatchEvent;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ChangeAnalyzer;

// ─── Block 2: Protocol Trait Implementations ──────────────

impl ILintableFilterProtocol for ChangeAnalyzer {
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

impl IEventDedupProtocol for ChangeAnalyzer {
    fn dedup_events(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent> {
        let mut deduped: HashMap<String, WatchEvent> = HashMap::new();
        for event in events {
            deduped.insert(event.path.clone(), event);
        }
        deduped.into_values().collect()
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
