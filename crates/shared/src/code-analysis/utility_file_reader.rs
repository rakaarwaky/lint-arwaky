// PURPOSE: Stateless utility functions for reading lintable files
// Domain-agnostic, reusable — valid utility per ARCHITECTURE §7
// ALGORITHM (read_lintable_file):
//   1. Check file metadata for size limit (2 MiB)
//   2. Read file content as UTF-8 string
//   3. Return Ok(Some(content)) if readable and within limit
//   4. Return Ok(None) if file exceeds size limit (graceful skip)
//   5. Return Err(message) if file is unreadable

/// Maximum file size for linting (2 MiB).
pub const MAX_LINT_FILE_BYTES: u64 = 2 * 1024 * 1024;

/// Read a file for linting. Returns:
/// - Ok(Some(content)) if file is readable and within size limit
/// - Ok(None) if file exceeds size limit (graceful skip, not an error)
/// - Err(message) if file is unreadable
pub fn read_lintable_file(path: &str) -> Result<Option<String>, String> {
    let meta = std::fs::metadata(path).map_err(|e| format!("{}: {}", path, e))?;
    if meta.len() > MAX_LINT_FILE_BYTES {
        return Ok(None);
    }
    std::fs::read_to_string(path)
        .map(Some)
        .map_err(|e| format!("{}: {}", path, e))
}
