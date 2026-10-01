// PURPOSE: ChangeFilter — IChangeFilterProtocol (FR-FileWatch-002)
//
// Deduplicates a batch of events by file path (last-write-wins) and then
// filters to lintable extensions only. Combines two utility-level operations
// into one capability seam so the FRD maps 1:1 to the protocol count.

use std::collections::HashMap;

use shared_file_watch::contract_watch_protocol::IChangeFilterProtocol;
use shared_file_watch::taxonomy_file_watch_vo::WatchEvent;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ChangeFilter {
    lintable_exts: &'static [&'static str],
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IChangeFilterProtocol for ChangeFilter {
    fn is_lintable(&self, path: &str) -> bool {
        self.lintable_exts.iter().any(|ext| path.ends_with(ext))
    }

    fn filter_events(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent> {
        let deduped = self.dedup_events(events);
        self.filter_lintable(deduped)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for ChangeFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl ChangeFilter {
    pub fn new() -> Self {
        Self {
            lintable_exts: &[
                ".rs", ".py", ".js", ".ts", ".tsx", ".jsx", ".mjs", ".cjs", ".json", ".css", ".md",
                ".toml", ".yaml", ".yml",
            ],
        }
    }

    fn is_lintable(&self, path: &str) -> bool {
        self.lintable_exts.iter().any(|ext| path.ends_with(ext))
    }

    pub fn dedup_events(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent> {
        let mut deduped: HashMap<String, WatchEvent> = HashMap::new();
        for event in events {
            deduped.insert(event.path.clone(), event);
        }
        deduped.into_values().collect()
    }

    fn filter_lintable(&self, events: Vec<WatchEvent>) -> Vec<WatchEvent> {
        events
            .into_iter()
            .filter(|e| self.is_lintable(&e.path))
            .collect()
    }
}
