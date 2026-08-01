// PURPOSE: Core filesystem I/O utilities — centralized replacements for legacy IO scattered
//          across import-rules, role-rules, naming-rules, code-analysis, orphan-detector,
//          external-lint, config-system, and cli-commands.
//
// Replaces: shared::common::utility_file_handler::{read_file_sync, read_file, read_file_safe,
//           walk_source_files, is_ignored_dir, is_path_ignored, find_workspace_root,
//           is_directory, is_file, path_exists, is_source_file}
//           shared::orphan_detector::utility_orphan_io::read_file_safe

use crate::common::taxonomy_path_vo::FilePath;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

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
        path.extension().and_then(|e| e.to_str()),
        Some("rs" | "py" | "ts" | "js" | "tsx" | "jsx")
    )
}

/// Check if a directory is in the ignored list.
pub fn is_ignored_dir(dir: &Path, ignored: &[String]) -> bool {
    let s = dir.to_string_lossy();
    is_path_ignored(&s, ignored)
}

/// Collect a single source file path into the output vector.
pub fn collect_source_file(path: &Path, files: &mut Vec<FilePath>) {
    if let Some(path_str) = path.to_str()
        && let Ok(fp) = FilePath::new(path_str.to_string())
    {
        files.push(fp);
    }
}

/// Walk directory recursively, collecting all source file paths (skipping ignored patterns).
pub fn walk_directory(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let workspace_subdirs = ["crates", "packages", "modules"];
    let found_subdirs: Vec<PathBuf> = workspace_subdirs
        .iter()
        .map(|s| root.join(s))
        .filter(|p| p.is_dir())
        .collect();

    let mut visited = HashSet::<PathBuf>::new();
    if !found_subdirs.is_empty() {
        for sub in found_subdirs {
            walk_directory_inner(&sub, files, ignored, &mut visited);
        }
    } else {
        walk_directory_inner(&root, files, ignored, &mut visited);
    }
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
                            walk_directory_inner(&target, files, ignored, visited);
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
    let workspace_subdirs = ["crates", "packages", "modules"];
    let found_subdirs: Vec<PathBuf> = workspace_subdirs
        .iter()
        .map(|s| root.join(s))
        .filter(|p| p.is_dir())
        .collect();

    let mut visited = HashSet::<PathBuf>::new();
    if !found_subdirs.is_empty() {
        for sub in found_subdirs {
            walk_directory_with_extensions_inner(&sub, files, ignored, extensions, &mut visited);
        }
    } else {
        walk_directory_with_extensions_inner(&root, files, ignored, extensions, &mut visited);
    }
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
                                &target, files, ignored, extensions, visited,
                            );
                        } else if target_meta.is_file()
                            && target.starts_with(dir)
                            && target
                                .extension()
                                .and_then(|e| e.to_str())
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
            } else if path
                .extension()
                .and_then(|e| e.to_str())
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
        // Handle /prefix absolute path patterns
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

        // Handle *.ext patterns (suffix extension match)
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

        // Handle .-prefix patterns (hidden dirs/files)
        if pat.starts_with('.') {
            if segments.iter().any(|seg| *seg == pat) {
                return true;
            }
            continue;
        }

        // Handle multi-segment path patterns
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

/// Scan directory with ignored paths filter (matches shared signature).
/// Returns FilePathList of source files.
pub fn scan_directory_with_ignored(
    path: &crate::common::taxonomy_path_vo::DirectoryPath,
    ignored_paths: &[String],
) -> Result<
    crate::common::taxonomy_paths_vo::FilePathList,
    crate::common::taxonomy_filesystem_error::FileSystemError,
> {
    let dir = std::path::Path::new(&path.value);
    if !dir.exists() || !dir.is_dir() {
        return Ok(crate::common::taxonomy_paths_vo::FilePathList { values: vec![] });
    }
    let files = collect_all_source_files(dir, ignored_paths);
    Ok(crate::common::taxonomy_paths_vo::FilePathList { values: files })
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

/// Write content to a file.
pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> std::io::Result<()> {
    std::fs::write(path, contents)
}

/// Read file content with cache fallback (matches shared::read_file_safe with cache).
pub fn read_file_with_cache<P: AsRef<Path>>(path: P) -> String {
    let path_buf = path.as_ref().to_path_buf();
    if let Some(content) = FILE_CACHE.get(&path_buf) {
        return content.value().clone();
    }
    std::fs::read_to_string(path).unwrap_or_default()
}

/// Read a file for linting. Returns:
/// - Ok(Some(content)) if file is readable and within size limit
/// - Ok(None) if file exceeds size limit (graceful skip)
/// - Err(message) if file is unreadable
pub fn read_lintable_file(path: &str) -> Result<Option<String>, String> {
    // Fast path: check global cache first
    let path_buf = std::path::PathBuf::from(path);
    if let Some(content) = FILE_CACHE.get(&path_buf) {
        return Ok(Some(content.value().clone()));
    }
    // Slow path: direct I/O with size check
    let meta = std::fs::metadata(path).map_err(|e| format!("{}: {}", path, e))?;
    if meta.len() > MAX_LINT_FILE_BYTES {
        return Ok(None);
    }
    std::fs::read_to_string(path)
        .map(Some)
        .map_err(|e| format!("{}: {}", path, e))
}

// ─── Path Helpers ───────────────────────────────────────────

/// Get file basename (filename without directory).
pub fn get_basename(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(path)
}

/// Get file stem (filename without extension).
pub fn get_file_stem(path: &str) -> &str {
    Path::new(path)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or(path)
}

/// Get parent directory path.
pub fn get_parent(path: &str) -> &str {
    Path::new(path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or(path)
}

// ─── is_source_file (str extension overload) ────────────────

/// Check if an extension string is a recognized source file extension.
/// Matches shared::common::utility_file_handler::is_source_file(ext: &str).
pub fn is_source_ext(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

// ─── Workspace-aware Walking ────────────────────────────────

/// Workspace-restricted directories.
const WORKSPACE_DIRS: [&str; 3] = ["crates", "packages", "modules"];

/// Walk source files with workspace restriction.
/// Only walks into workspace member directories (crates/, packages/, modules/).
pub fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let restrict = workspace_restrict(&root);
    let mut visited = HashSet::new();
    walk_source_files_inner(&root, files, ignored, &mut visited, &root, &restrict);
}

fn workspace_restrict(root: &Path) -> Option<HashSet<String>> {
    let mut has_ws = false;
    for d in &WORKSPACE_DIRS {
        if root.join(d).is_dir() {
            has_ws = true;
            break;
        }
    }
    if !has_ws {
        return None;
    }
    // Only allow workspace member directories — not test-workspaces, target, etc.
    let mut set = HashSet::new();
    for d in &WORKSPACE_DIRS {
        if root.join(d).is_dir() {
            set.insert(d.to_string());
        }
    }
    Some(set)
}

fn walk_source_files_inner(
    dir: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
    visited: &mut HashSet<PathBuf>,
    root: &Path,
    restrict: &Option<HashSet<String>>,
) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_ignored_dir(&path, ignored) {
                continue;
            }
            if let Ok(meta) = std::fs::symlink_metadata(&path)
                && meta.file_type().is_symlink()
            {
                if let Ok(target) = std::fs::canonicalize(&path) {
                    if !target.starts_with(root) || !visited.insert(target.clone()) {
                        continue;
                    }
                    if let Ok(tm) = target.metadata() {
                        if tm.is_dir() {
                            walk_source_files_inner(
                                &target, files, ignored, visited, root, restrict,
                            );
                        } else if tm.is_file()
                            && target.starts_with(root)
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
                if !visited.insert(canonical.clone()) {
                    continue;
                }
                // Workspace restriction: only descend into allowed member dirs at root level
                if dir == root
                    && let Some(r) = restrict
                    && let Some(name) = path.file_name().and_then(|n| n.to_str())
                    && !r.contains(name)
                {
                    continue;
                }
                walk_source_files_inner(&path, files, ignored, visited, root, restrict);
            } else if is_source_file(&path) {
                collect_source_file(&path, files);
            }
        }
    }
}

/// Walk only .rs files.
pub fn walk_rs_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut visited = HashSet::new();
    walk_rs_files_inner(&root, files, ignored, &mut visited);
}

fn walk_rs_files_inner(
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
            if path.is_dir() {
                let canonical = std::fs::canonicalize(&path).unwrap_or_else(|_| path.to_path_buf());
                if visited.insert(canonical) {
                    walk_rs_files_inner(&path, files, ignored, visited);
                }
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                collect_source_file(&path, files);
            }
        }
    }
}

/// Collect all source files (no workspace restriction).
pub fn collect_all_source_files(dir: &Path, ignored_paths: &[String]) -> Vec<FilePath> {
    let mut files = Vec::new();
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut visited = HashSet::new();
    collect_all_inner(&root, &mut files, ignored_paths, &mut visited);
    files
}

fn collect_all_inner(
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
            if path.is_dir() {
                let canonical = std::fs::canonicalize(&path).unwrap_or_else(|_| path.to_path_buf());
                if visited.insert(canonical) {
                    collect_all_inner(&path, files, ignored, visited);
                }
            } else if is_source_file(&path) {
                collect_source_file(&path, files);
            }
        }
    }
}

/// Collect all raw source files (no workspace restriction, returns all paths).
pub fn collect_all_source_files_raw(dir: &Path) -> Vec<FilePath> {
    collect_all_source_files(dir, &[])
}

// ─── Orphan Detector IO ─────────────────────────────────────

/// Read file with diagnostic error message.
pub fn read_file_with_diagnostic(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))
}

/// List directory entries as (name, path, is_dir) tuples.
pub fn list_directory_entries(dir_path: &Path) -> Vec<(String, String, bool)> {
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

// ─── External Lint IO ───────────────────────────────────────

/// Canonicalize a path string to absolute.
pub fn canonicalize_path(path_str: &str) -> PathBuf {
    let path = Path::new(path_str);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}

/// Check if directory contains Python files.
pub fn has_python_files(dir_path: &Path) -> bool {
    dir_path
        .read_dir()
        .into_iter()
        .flatten()
        .flatten()
        .any(|e| e.path().extension().and_then(|x| x.to_str()) == Some("py"))
}

/// Check if directory contains a config file.
pub fn has_config_file(dir_path: &Path) -> bool {
    const CONFIG_NAMES: [&str; 6] = [
        ".eslintrc",
        ".prettierrc",
        "tsconfig.json",
        "pyproject.toml",
        "setup.cfg",
        ".flake8",
    ];
    dir_path
        .read_dir()
        .into_iter()
        .flatten()
        .flatten()
        .any(|e| {
            let name = e.file_name();
            CONFIG_NAMES.iter().any(|c| name == *c)
                || name.to_string_lossy().ends_with(".config.js")
                || name.to_string_lossy().ends_with(".config.ts")
        })
}

/// Check if a Cargo.toml exists and return its directory.
pub fn has_cargo_toml(path_str: &str) -> Option<String> {
    let path = Path::new(path_str);
    if path.join("Cargo.toml").exists() {
        Some(path.to_string_lossy().to_string())
    } else {
        None
    }
}

/// Check if a Cargo.lock exists and return its directory.
pub fn has_cargo_lock(path_str: &str) -> Option<String> {
    let path = Path::new(path_str);
    if path.join("Cargo.lock").exists() {
        Some(path.to_string_lossy().to_string())
    } else {
        None
    }
}

/// Check if an executable is in PATH.
pub fn is_executable_in_path(executable: &str) -> bool {
    std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .any(|dir| {
            let p = Path::new(dir).join(executable);
            p.exists() && p.metadata().is_ok_and(|m| m.is_file())
        })
}

/// Check if an executable exists in local bin.
pub fn has_local_bin(working_dir: &Path, executable: &str) -> bool {
    let local_bin = working_dir
        .join("node_modules")
        .join(".bin")
        .join(executable);
    local_bin.exists()
}

// ─── File Content Cache (DashMap) ──────────────────────────
// Global file cache — read once, serve from memory.
// FR-001/FR-002: Cache populated from FileEntry.content after walk.

use crate::filesystem::taxonomy_filesystem_vo::FileEntry;
use dashmap::DashMap;
use rayon::prelude::*;
use std::sync::LazyLock;

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

/// Detect which programming languages are present in a directory by walking
/// the filesystem and checking file extensions (lightweight — no file reading
/// or parsing). Returns `(has_rust, has_python, has_js)`.
///
/// - Rust: `.rs`
/// - Python: `.py`
/// - JS/TS: `.js`, `.jsx`, `.ts`, `.tsx`
///
/// Early-terminates once all three booleans are true. Follows symlinks within
/// the workspace root (directory), skips symlinks pointing outside.
/// Skips common non-source directories: `node_modules`, `target`, `.git`.
pub fn detect_languages(root: &std::path::Path) -> (bool, bool, bool) {
    let mut has_rs = false;
    let mut has_py = false;
    let mut has_js = false;

    fn walk_detect(dir: &std::path::Path, has_rs: &mut bool, has_py: &mut bool, has_js: &mut bool) {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = match path.file_name().and_then(|n| n.to_str()) {
                    Some(n) => n,
                    None => continue,
                };
                if matches!(
                    name,
                    "node_modules" | "target" | ".git" | "Graph-It-Live" | "tests"
                ) {
                    continue;
                }
                walk_detect(&path, has_rs, has_py, has_js);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                match ext {
                    "rs" => *has_rs = true,
                    "py" => *has_py = true,
                    "js" | "ts" | "jsx" | "tsx" => *has_js = true,
                    _ => {}
                }
            }
            if *has_rs && *has_py && *has_js {
                return;
            }
        }
    }

    if root.is_file() {
        if let Some(ext) = root.extension().and_then(|e| e.to_str()) {
            match ext {
                "rs" => has_rs = true,
                "py" => has_py = true,
                "js" | "ts" | "jsx" | "tsx" => has_js = true,
                _ => {}
            }
        }
    } else {
        walk_detect(root, &mut has_rs, &mut has_py, &mut has_js);
    }
    (has_rs, has_py, has_js)
}
