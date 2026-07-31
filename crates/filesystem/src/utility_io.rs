// PURPOSE: Core filesystem I/O utilities — centralized replacements for legacy IO scattered
//          across import-rules, role-rules, naming-rules, code-analysis, orphan-detector,
//          external-lint, config-system, and cli-commands.
//
// Replaces: shared::common::utility_file_handler::{read_file_sync, read_file, read_file_safe,
//           walk_source_files, is_ignored_dir, is_path_ignored, find_workspace_root,
//           is_directory, is_file, path_exists, is_source_file}
//           shared::orphan_detector::utility_orphan_io::read_file_safe

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use shared::common::taxonomy_path_vo::FilePath;

/// Default directories to skip during directory walks.
pub const DEFAULT_SKIP_DIRS: [&str; 7] = [
    "node_modules",
    "target",
    ".git",
    "Graph-It-Live",
    "tests",
    ".venv",
    "__pycache__",
];

/// Maximum allowed file size for full memory reads (2 MiB).
pub const MAX_LINT_FILE_BYTES: u64 = 2 * 1024 * 1024;

// ─── File Reading ─────────────────────────────────────────

/// Read file content synchronously. Returns Ok(content) or Err(io::Error).
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// Read file content, returning empty string on error.
pub fn read_file_safe<P: AsRef<Path>>(path: P) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

// ─── Directory Walking ────────────────────────────────────

/// Check if a path has a source file extension.
pub fn is_source_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|e| e.to_str()),
        Some("rs" | "py" | "ts" | "js" | "tsx" | "jsx")
    )
}

/// Check if a directory is in the ignored list.
pub fn is_ignored_dir(dir: &Path, ignored: &[String]) -> bool {
    let s = dir.to_string_lossy();
    is_path_ignored(&s, ignored)
}

/// Collect a single source file path into the output vector.
fn collect_source_file(path: &Path, files: &mut Vec<FilePath>) {
    if let Some(path_str) = path.to_str()
        && let Ok(fp) = FilePath::new(path_str.to_string())
    {
        files.push(fp);
    }
}

/// Walk directory recursively, collecting all source file paths (skipping ignored patterns).
pub fn walk_directory(
    dir: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut visited = HashSet::<PathBuf>::new();
    walk_directory_inner(&root, files, ignored, &mut visited);
}

fn walk_directory_inner(
    dir: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
    visited: &mut HashSet<PathBuf>,
) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if is_ignored_dir(&path, ignored) {
                continue;
            }
            if let Ok(sym_meta) = std::fs::symlink_metadata(&path)
                && sym_meta.file_type().is_symlink()
            {
                if let Ok(target) = std::fs::canonicalize(&path) {
                    if !target.starts_with(dir) {
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
                            );
                        } else if target_meta.is_file()
                            && target.starts_with(dir)
                            && is_source_file(&target)
                        {
                            collect_source_file(&target, files);
                        }
                    }
                }
                continue;
            }
            if path.is_dir() {
                let canonical = std::fs::canonicalize(&path).unwrap_or_else(|_| path.to_path_buf());
                if !visited.insert(canonical) {
                    continue;
                }
                walk_directory_inner(&path, files, ignored, visited);
            } else if is_source_file(&path) {
                collect_source_file(&path, files);
            }
        }
    }
}

/// Collect source files with extensions filter.
pub fn walk_directory_with_extensions(
    dir: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
    extensions: &[&str],
) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut visited = HashSet::<PathBuf>::new();
    walk_directory_with_extensions_inner(
        &root,
        files,
        ignored,
        extensions,
        &mut visited,
    );
}

fn walk_directory_with_extensions_inner(
    dir: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
    extensions: &[&str],
    visited: &mut HashSet<PathBuf>,
) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if is_ignored_dir(&path, ignored) {
                continue;
            }
            if let Ok(sym_meta) = std::fs::symlink_metadata(&path)
                && sym_meta.file_type().is_symlink()
            {
                if let Ok(target) = std::fs::canonicalize(&path) {
                    if !target.starts_with(dir) {
                        continue;
                    }
                    if !visited.insert(target.clone()) {
                        continue;
                    }
                    if let Ok(target_meta) = target.metadata() {
                        if target_meta.is_dir() {
                            walk_directory_with_extensions_inner(
                                &target,
                                files,
                                ignored,
                                extensions,
                                visited,
                            );
                        } else if target_meta.is_file()
                            && target.starts_with(dir)
                            && target.extension().and_then(|e| e.to_str())
                                .is_some_and(|ext| extensions.contains(&ext))
                        {
                            collect_source_file(&target, files);
                        }
                    }
                }
                continue;
            }
            if path.is_dir() {
                let canonical = std::fs::canonicalize(&path).unwrap_or_else(|_| path.to_path_buf());
                if !visited.insert(canonical) {
                    continue;
                }
                walk_directory_with_extensions_inner(&path, files, ignored, extensions, visited);
            } else if path.extension().and_then(|e| e.to_str())
                .is_some_and(|ext| extensions.contains(&ext))
            {
                collect_source_file(&path, files);
            }
        }
    }
}

// ─── Path Utilities ───────────────────────────────────────

/// Check if path exists.
pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Check if path is a directory.
pub fn is_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

/// Check if path is a file.
pub fn is_file<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
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
        } else {
            // Substring match (e.g., "node_modules" anywhere in path)
            if rel_path.contains(pat) {
                return true;
            }
        }
    }
    false
}

/// Walk up from `start` looking for workspace root markers.
/// Returns the first directory containing Cargo.toml, crates/, packages/, or modules/.
pub fn find_workspace_root(start: &str) -> Option<PathBuf> {
    let mut dir = Path::new(start).to_path_buf();
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

// ─── Directory Scanning ───────────────────────────────────

/// Scan directory entries, returning vector of (file_name, file_path, is_dir) tuples.
pub fn scan_directory(dir_path: &Path) -> Vec<(String, String, bool)> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = dir_path.read_dir() {
        for dir_entry in read_dir.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                let path = dir_entry.path();
                let is_dir = path.is_dir();
                entries.push((name.to_string(), path.to_string_lossy().to_string(), is_dir));
            }
        }
    }
    entries
}

/// Recursively scan directory for files, returning vector of file paths.
/// Skips hidden directories and common heavy dependency/build directories.
pub fn scan_directory_recursive(dir_path: &Path) -> Vec<String> {
    let mut files = Vec::new();
    _scan_directory_recursive(dir_path, &mut files);
    files
}

fn _scan_directory_recursive(dir_path: &Path, files: &mut Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(dir_path) {
        for dir_entry in entries.flatten() {
            if let Some(name) = dir_entry.file_name().to_str() {
                if name.starts_with('.') {
                    continue;
                }

                let path = dir_entry.path();

                if path.is_dir() {
                    if matches!(
                        name,
                        "target" | "node_modules" | "dist" | "build" | "__pycache__" | ".venv"
                    ) {
                        continue;
                    }

                    _scan_directory_recursive(&path, files);
                } else if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
        }
    }
}

