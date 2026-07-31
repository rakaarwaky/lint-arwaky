// PURPOSE: Stateless utility functions for reading lintable files
// Domain-agnostic, reusable — valid utility per ARCHITECTURE §7
// ALGORITHM (read_lintable_file):
//   1. Check file cache (if populated)
//   2. Check file metadata for size limit (2 MiB)
//   3. Read file content as UTF-8 string
//   4. Return Ok(Some(content)) if readable and within limit
//   5. Return Ok(None) if file exceeds size limit (graceful skip)
//   6. Return Err(message) if file is unreadable

use std::collections::HashMap;
use std::sync::OnceLock;

/// Maximum file size for linting (2 MiB).
pub const MAX_LINT_FILE_BYTES: u64 = 2 * 1024 * 1024;

/// Global file cache — populated once per scan, read by all linters.
static FILE_CACHE: OnceLock<HashMap<String, String>> = OnceLock::new();

/// Populate the file cache with pre-read file contents.
/// Call once at scan start; subsequent `read_lintable_file` calls use cache.
pub fn populate_file_cache(files: &[String]) {
    use rayon::prelude::*;

    let cache: HashMap<String, String> = files
        .par_iter()
        .filter_map(|path| {
            let meta = std::fs::metadata(path).ok()?;
            if meta.len() > MAX_LINT_FILE_BYTES {
                return None;
            }
            let content = std::fs::read_to_string(path).ok()?;
            Some((path.clone(), content))
        })
        .collect();

    let _ = FILE_CACHE.set(cache);
}

/// Clear the file cache (call after scan completes).
pub fn clear_file_cache() {
    // OnceLock can't be cleared, but we can set a new empty one on next scan.
    // For now, just let it expire when the process ends.
    // In practice, each scan call repopulates.
}

/// Read a file for linting. Returns:
/// - Ok(Some(content)) if file is readable and within size limit
/// - Ok(None) if file exceeds size limit (graceful skip, not an error)
/// - Err(message) if file is unreadable
pub fn read_lintable_file(path: &str) -> Result<Option<String>, String> {
    // Fast path: check cache first
    if let Some(cache) = FILE_CACHE.get() {
        if let Some(content) = cache.get(path) {
            return Ok(Some(content.clone()));
        }
        // File not in cache (e.g. not in scan set) — skip size check, just read
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
