// PURPOSE: Stateless utility functions for reading lintable files
// Domain-agnostic, reusable — valid utility per ARCHITECTURE §7
// ALGORITHM (read_lintable_file):
//   1. Check global file cache (DashMap, populated by filesystem service)
//   2. If not cached, check file metadata for size limit (2 MiB)
//   3. Read file content as UTF-8 string
//   4. Return Ok(Some(content)) if readable and within limit
//   5. Return Ok(None) if file exceeds size limit (graceful skip)
//   6. Return Err(message) if file is unreadable

use dashmap::DashMap;
use std::sync::LazyLock;

/// Maximum file size for linting (2 MiB).
pub const MAX_LINT_FILE_BYTES: u64 = 2 * 1024 * 1024;

/// Global file cache — thread-safe, populated by filesystem service.
/// All linters read from this cache automatically.
static FILE_CACHE: LazyLock<DashMap<String, String>> = LazyLock::new(DashMap::new);

/// Populate the global file cache (called by filesystem service).
pub fn populate_cache(files: &[(String, String)]) {
    for (path, content) in files {
        FILE_CACHE.insert(path.clone(), content.clone());
    }
}

/// Clear the global file cache.
pub fn clear_cache() {
    FILE_CACHE.clear();
}

/// Get a cached file content.
pub fn get_cached(path: &str) -> Option<String> {
    FILE_CACHE.get(path).map(|r| r.value().clone())
}

/// Read a file for linting. Returns:
/// - Ok(Some(content)) if file is readable and within size limit
/// - Ok(None) if file exceeds size limit (graceful skip, not an error)
/// - Err(message) if file is unreadable
pub fn read_lintable_file(path: &str) -> Result<Option<String>, String> {
    // Fast path: check global cache first
    if let Some(content) = FILE_CACHE.get(path) {
        return Ok(Some(content.value().clone()));
    }

    // Slow path: direct I/O
    let meta = std::fs::metadata(path).map_err(|e| format!("{}: {}", path, e))?;
    if meta.len() > MAX_LINT_FILE_BYTES {
        return Ok(None);
    }
    std::fs::read_to_string(path)
        .map(Some)
        .map_err(|e| format!("{}: {}", path, e))
}
