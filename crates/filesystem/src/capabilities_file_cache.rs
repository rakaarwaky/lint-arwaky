// FR-007: File Cache — Capabilities layer
// Owns bounded cache state (OnceLock<Mutex<HashMap>>).
// Utility layer keeps stateless string-keyed helpers only.

use crate::utility_file_cache::cache_get_by_str;
use crate::utility_filesystem_io::read_file_safe;

// ═══════════════════════════════════════════════════════════════
// Bounded HashMap Cache (ad-hoc cache)
// ═══════════════════════════════════════════════════════════════

const MAX_CACHE_ENTRIES: usize = 20_000;

static FILE_CACHE_MAP: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<String, String>>,
> = std::sync::OnceLock::new();

fn file_cache_map() -> &'static std::sync::Mutex<std::collections::HashMap<String, String>> {
    FILE_CACHE_MAP.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

/// Read file content using internal bounded cache.
pub fn read_cached(
    path: &shared::common::taxonomy_path_vo::FilePath,
) -> shared::common::taxonomy_source_vo::ContentString {
    let mut cache = file_cache_map().lock().unwrap_or_else(|e| e.into_inner());

    if let Some(content) = cache.get(path.value()) {
        return shared::common::taxonomy_source_vo::ContentString::new(content.clone());
    }

    let content = cache_get_by_str(path.value()).unwrap_or_else(|| read_file_safe(path.value()));

    if cache.len() < MAX_CACHE_ENTRIES {
        cache.insert(path.value().to_string(), content.clone());
    }

    shared::common::taxonomy_source_vo::ContentString::new(content)
}
