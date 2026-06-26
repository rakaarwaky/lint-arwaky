// PURPOSE: taxonomy_orphan_filename_helper — shared filename parsing utilities for orphan analyzers
// Extracts basename and suffix from file paths in a consistent way across all orphan layer analyzers.

/// Extract the basename (filename without directory) from a file path string.
/// Handles both `/`-separated and OS paths gracefully.
pub fn file_basename(fp: &str) -> &str {
    fp.split('/').next_back().unwrap_or(fp)
}

/// Extract the domain suffix from a file stem (the part after the last `_`, with
/// language extension stripped). Returns an empty string if no underscore is present.
///
/// Example: `capabilities_import_checker.rs` → `checker`
pub fn file_suffix(fp: &str) -> String {
    let basename = file_basename(fp);
    basename
        .rsplit('_')
        .next()
        .unwrap_or_default()
        .replace(".rs", "")
        .replace(".py", "")
        .replace(".ts", "")
        .replace(".js", "")
}
