use std::collections::HashSet;
use std::fs;

/// PathUtils — utility functions for walking directories and matching patterns.
pub struct PathUtils;

impl PathUtils {
    /// Walk a directory recursively, collecting files while skipping ignored patterns.
    /// Supports both flat patterns (e.g., "tests") and path patterns (e.g., "src/tests").
    /// Prevents symlink cycles using a visited set of canonical paths.
    pub fn walk_recursive(dir: &std::path::Path, ignored: &[&str]) -> Vec<std::path::PathBuf> {
        let root = fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
        let mut visited = HashSet::new();
        Self::walk_recursive_internal(&root, &root, ignored, &mut visited)
    }

    fn walk_recursive_internal(
        root: &std::path::Path,
        dir: &std::path::Path,
        ignored: &[&str],
        visited: &mut HashSet<std::path::PathBuf>,
    ) -> Vec<std::path::PathBuf> {
        let mut results = Vec::new();

        if !dir.is_dir() {
            if dir.is_file() {
                // For a single file, check both the file name and the relative path
                if let Some(name_str) = dir.file_name().and_then(|s| s.to_str()) {
                    if !ignored.contains(&name_str) {
                        let rel_path = dir.strip_prefix(root).unwrap_or(dir);
                        let rel_str = rel_path.to_string_lossy();
                        if !Self::matches_any_pattern(&rel_str, ignored) {
                            results.push(dir.to_path_buf());
                        }
                    }
                }
            }
            return results;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let rel_path = path.strip_prefix(root).unwrap_or(&path);
                let rel_str = rel_path.to_string_lossy();

                if Self::matches_any_pattern(&rel_str, ignored) {
                    continue;
                }

                if path.is_dir() {
                    let canonical = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
                    if !visited.insert(canonical) {
                        continue;
                    }
                    results.extend(Self::walk_recursive_internal(root, &path, ignored, visited));
                } else {
                    results.push(path);
                }
            }
        }

        results
    }

    fn matches_any_pattern(rel_path: &str, ignored: &[&str]) -> bool {
        for pattern in ignored {
            // Exact match on the full relative path or any prefix segment
            if rel_path == *pattern || rel_path.starts_with(&format!("{}/", pattern)) {
                return true;
            }
            // Also match just the filename (flat pattern) for backward compatibility
            if let Some(file_name) = std::path::Path::new(rel_path).file_name() {
                if file_name == *pattern {
                    return true;
                }
            }
        }
        false
    }

    /// Convenience wrapper used by OSFileSystemAdapter and workspace helpers.
    pub fn collect_paths(start: &str, ignored: &[&str]) -> Vec<std::path::PathBuf> {
        let root = std::path::Path::new(start);
        Self::walk_recursive(root, ignored)
    }
}
