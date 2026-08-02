// FR-007: File Cache — read from bounded DashMap cache
// Stateless function used by orchestrator's IFilesystemAggregate::read_cached

use dashmap::DashMap;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;

/// Read file content from the bounded string cache.
pub fn read_cached(path: &FilePath, cache: &DashMap<String, String>) -> ContentString {
    let key: &str = path;
    match cache.get(key) {
        Some(entry) => ContentString { value: entry.value().clone() },
        None => ContentString { value: String::new() },
    }
}
