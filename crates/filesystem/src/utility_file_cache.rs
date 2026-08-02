// FR-007: File Cache
// Produces: cached file content
// Consumer: orphan-detector
//
// Utility: static cache + stateless functions

use dashmap::DashMap;
use rayon::iter::ParallelIterator;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use std::path::PathBuf;
use std::sync::LazyLock;

use crate::utility_filesystem_io::read_file_safe;

// ═══════════════════════════════════════════════════════════════
// DashMap Cache (pipeline cache)
// ═══════════════════════════════════════════════════════════════

static FILE_CACHE: LazyLock<DashMap<PathBuf, String>> = LazyLock::new(DashMap::new);

/// Populate cache from file entries (uses content already in FileEntry).
pub fn cache_populate(files: &[FileEntry]) {
    files.par_iter().for_each(|entry| {
        if !entry.content.is_empty() {
            FILE_CACHE.insert(entry.path.clone(), entry.content.clone());
        }
    });
}

/// Get cached file content.
pub fn cache_get(path: &PathBuf) -> Option<String> {
    FILE_CACHE.get(path).map(|r| r.value().clone())
}

/// Check if file is in cache.
pub fn cache_contains(path: &PathBuf) -> bool {
    FILE_CACHE.contains_key(path)
}

/// Get total memory usage in bytes.
pub fn cache_memory_bytes() -> usize {
    FILE_CACHE
        .iter()
        .map(|e| e.key().as_os_str().len() + e.value().len())
        .sum()
}

/// Clear all cached entries.
pub fn cache_clear() {
    FILE_CACHE.clear()
}

// ═══════════════════════════════════════════════════════════════
// Bounded HashMap Cache (ad-hoc cache)
// ═══════════════════════════════════════════════════════════════

const MAX_CACHE_ENTRIES: usize = 20_000;

static FILE_CACHE_MAP: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, String>>> =
    std::sync::OnceLock::new();

fn file_cache_map() -> &'static std::sync::Mutex<std::collections::HashMap<String, String>> {
    FILE_CACHE_MAP.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

/// Read file content using internal bounded cache.
pub fn read_cached(path: &shared::common::taxonomy_path_vo::FilePath) -> shared::common::taxonomy_source_vo::ContentString {
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

/// Clear bounded file cache.
pub fn clear_file_cache() {
    let mut cache = file_cache_map().lock().unwrap_or_else(|e| e.into_inner());
    cache.clear();
}

// ═══════════════════════════════════════════════════════════════
// String-keyed Cache (code-analysis compatibility)
// ═══════════════════════════════════════════════════════════════

static STRING_CACHE: LazyLock<DashMap<String, String>> = LazyLock::new(DashMap::new);

/// Populate the string-keyed file cache.
pub fn cache_populate_from_pairs(files: &[(String, String)]) {
    for (path, content) in files {
        STRING_CACHE.insert(path.clone(), content.clone());
    }
}

/// Get cached file content by string path.
pub fn cache_get_by_str(path: &str) -> Option<String> {
    STRING_CACHE.get(path).map(|r| r.value().clone())
}

/// Check if a string path is in the string-keyed cache.
pub fn cache_contains_str(path: &str) -> bool {
    STRING_CACHE.contains_key(path)
}

/// Clear the string-keyed file cache.
pub fn cache_clear_str() {
    STRING_CACHE.clear();
}
