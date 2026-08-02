// FR-007: File Cache — Capabilities layer
// Owns bounded cache state (OnceLock<Mutex<HashMap>>).
// String cache lives in the orchestrator struct (not utility statics).

use dashmap::DashMap;
use std::sync::OnceLock;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::utility_filesystem_io::read_file_safe;

// ═══════════════════════════════════════════════════════════════
// Bounded HashMap Cache (ad-hoc cache)
// ═══════════════════════════════════════════════════════════════

const MAX_CACHE_ENTRIES: usize = 20_000;

static FILE_CACHE_MAP: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn file_cache_map() -> &'static Mutex<HashMap<String, String>> {
    FILE_CACHE_MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Read file content using internal bounded cache.
/// String cache (DashMap) is owned by the orchestrator struct.
pub fn read_cached(
    path: &shared::common::taxonomy_path_vo::FilePath,
    string_cache: &DashMap<String, String>,
) -> shared::common::taxonomy_source_vo::ContentString {
    let mut cache = file_cache_map().lock().unwrap_or_else(|e| e.into_inner());

    if let Some(content) = cache.get(path.value()) {
        return shared::common::taxonomy_source_vo::ContentString::new(content.clone());
    }

    let content = string_cache
        .get(path.value())
        .map(|r| r.value().clone())
        .unwrap_or_else(|| read_file_safe(path.value()));

    if cache.len() < MAX_CACHE_ENTRIES {
        cache.insert(path.value().to_string(), content.clone());
    }

    shared::common::taxonomy_source_vo::ContentString::new(content)
}
