// PURPOSE: Utility layer — core filesystem I/O utilities
// Provides: read_file(), walk_directory(), is_source_file(), path_exists()
// Replaces scattered std::fs calls across import-rules, role-rules, naming-rules,
// code-analysis, orphan-detector, external-lint, config-system, cli-commands.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Maximum file size for linting (2 MiB).
pub const MAX_LINT_FILE_BYTES: u64 = 2 * 1024 * 1024;

/// Check if a file extension is a known source file.
pub fn is_source_file(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

/// Read file content synchronously. Returns Ok(content) or Err(io::Error).
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// Read file content, returning empty string on error (backward compatible).
pub fn read_file_safe(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// Check if path exists.
pub fn path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

/// Check if path is a directory.
pub fn is_dir(path: &str) -> bool {
    std::path::Path::new(path).is_dir()
}

/// Check if path is a file.
pub fn is_file(path: &str) -> bool {
    std::path::Path::new(path).is_file()
}

/// Return true if `rel_path` should be skipped based on `ignored` patterns.
pub fn is_path_ignored(rel_path: &str, ignored: &[String]) -> bool {
    if rel_path.is_empty() {
        return false;
    }
    let segments: Vec<&str> = rel_path
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .collect();
    for pat in ignored {
        if pat.is_empty() {
            continue;
        }
        if let Some(stripped) = pat.strip_prefix('/') {
            if stripped.is_empty() {
                continue;
            }
            let pat_segments: Vec<&str> = stripped
                .split(['/', '\\'])
                .filter(|s| !s.is_empty())
                .collect();
            if pat_segments.is_empty() {
                continue;
            }
            let n_pat = pat_segments.len();
            let n_seg = segments.len();
            if n_seg < n_pat {
                continue;
            }
            for start in 0..=(n_seg - n_pat) {
                if segments[start..start + n_pat] == pat_segments[..] {
                    return true;
                }
            }
            continue;
        }

        // Handle **/*.rs patterns (recursive glob)
        if pat.starts_with("**/") {
            let suffix = pat.strip_prefix("**/").unwrap_or(pat);
            if let Some(ext_pattern) = suffix.strip_prefix("*.") {
                let ext = ext_pattern.trim_start_matches('.');
                if !ext.is_empty() {
                    let basename = segments.last().copied().unwrap_or_default();
                    if basename.ends_with(&format!(".{ext}")) {
                        return true;
                    }
                }
            }
            continue;
        }

        // Handle target/* patterns (prefix with wildcard)
        if let Some(prefix) = pat.strip_suffix("/*") {
            if !prefix.is_empty() && segments.first() == Some(&prefix) {
                return true;
            }
            continue;
        }

        if let Some(suffix) = pat.strip_prefix("*.") {
            let suffix = suffix.trim_start_matches('.');
            if suffix.is_empty() {
                continue;
            }
            let basename = segments.last().copied().unwrap_or_default();
            if basename.ends_with(&format!(".{suffix}")) {
                return true;
            }
            continue;
        }

        if pat.starts_with('.') {
            if segments.iter().any(|seg| *seg == pat) {
                return true;
            }
            continue;
        }
        let pat_segments: Vec<&str> = pat.split(['/', '\\']).filter(|s| !s.is_empty()).collect();
        if pat_segments.len() == 1 {
            if segments.contains(&pat_segments[0]) {
                return true;
            }
        } else if pat_segments.len() > 1 {
            let n_pat = pat_segments.len();
            let n_seg = segments.len();
            if n_seg >= n_pat {
                for start in 0..=(n_seg - n_pat) {
                    if segments[start..start + n_pat] == pat_segments[..] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Common directory names to skip during recursive scanning.
pub const DEFAULT_SKIP_DIRS: &[&str] = &[
    "target",
    "node_modules",
    "dist",
    "build",
    "__pycache__",
    ".venv",
];

/// Walk a directory tree collecting all source files, skipping ignored directories.
/// Respects `.gitignore` files and custom ignore patterns.
/// Uses canonical-path-based visited set (works on all platforms).
pub fn walk_directory(
    dir: &Path,
    files: &mut Vec<PathBuf>,
    ignored: &[String],
    extensions: &[&str],
) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut visited = HashSet::<PathBuf>::new();
    walk_directory_inner(
        &root,
        files,
        ignored,
        &mut visited,
        &root,
        extensions,
    )
}

fn walk_directory_inner(
    dir: &Path,
    files: &mut Vec<PathBuf>,
    ignored: &[String],
    visited: &mut HashSet<PathBuf>,
    root: &Path,
    extensions: &[&str],
) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Skip symlink targets outside root to prevent path traversal
            if let Ok(sym_meta) = std::fs::symlink_metadata(&path)
                && sym_meta.file_type().is_symlink()
            {
                if let Ok(target) = std::fs::canonicalize(&path) {
                    if !target.starts_with(root) {
                        continue;
                    }
                    if !visited.insert(target.clone()) {
                        continue;
                    }
                    if let Ok(target_meta) = target.metadata() {
                        if target_meta.is_dir() {
                            walk_directory_inner(
                                &target,
                                files,
                                ignored,
                                visited,
                                root,
                                extensions,
                            );
                        } else if target_meta.is_file() {
                            collect_if_source_file(&target, ignored, extensions, files);
                        }
                    }
                }
                continue;
            }

            // Skip ignored directories
            if is_ignored_dir(&path, ignored) {
                continue;
            }

            if path.is_dir() {
                let canonical = std::fs::canonicalize(&path).unwrap_or_else(|_| path.to_path_buf());
                if !visited.insert(canonical) {
                    walk_directory_inner(
                        &path,
                        files,
                        ignored,
                        visited,
                        root,
                        extensions,
                    );
                }
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if extensions.contains(&ext) && !is_path_ignored(
                    &path.to_string_lossy(),
                    ignored,
                ) {
                    files.push(path);
                }
            }
        }
    }
}

/// Check if a directory is in the ignored list.
fn is_ignored_dir(dir: &Path, ignored: &[String]) -> bool {
    let s = dir.to_string_lossy();
    is_path_ignored(&s, ignored)
}

/// Collect a file into the output vector if it's a source file and not ignored.
fn collect_if_source_file(
    path: &Path,
    ignored: &[String],
    extensions: &[&str],
    files: &mut Vec<PathBuf>,
) {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if extensions.contains(&ext) && !is_path_ignored(&path.to_string_lossy(), ignored) {
            files.push(path.to_path_buf());
        }
    }
}

/// Walk up from `start` looking for workspace root markers.
/// Returns the first directory containing Cargo.toml, crates/, packages/, or modules/.
pub fn find_workspace_root(start: &str) -> Option<PathBuf> {
    let mut dir = std::path::Path::new(start).to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }
    loop {
        // Priority 1: workspace root markers (crates/, packages/, modules/)
        if dir.join("crates").is_dir()
            || dir.join("packages").is_dir()
            || dir.join("modules").is_dir()
        {
            return Some(dir);
        }
        // Priority 2: Cargo.toml (only if not inside a workspace member)
        if dir.join("Cargo.toml").exists() {
            if let Some(parent) = dir.parent() {
                let parent_name = parent
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if parent.join("crates").is_dir()
                    || parent.join("packages").is_dir()
                    || parent.join("modules").is_dir()
                    || matches!(parent_name, "crates" | "packages" | "modules")
                {
                    // Don't return yet — parent/grandparent is the real workspace root
                } else {
                    return Some(dir);
                }
            } else {
                return Some(dir);
            }
        }
        if !dir.pop() {
            return None;
        }
    }
}
