// PURPOSE: Core filesystem I/O utilities — centralized replacements for legacy IO scattered
//          across import-rules, role-rules, naming-rules, code-analysis, orphan-detector,
//          external-lint, config-system, and cli-commands.
//
// Replaces: shared::common::utility_file_handler::{read_file_sync, read_file, read_file_safe,
//           walk_source_files, is_ignored_dir, is_path_ignored, find_workspace_root,
//           is_directory, is_file, path_exists, is_source_file}
//           shared::orphan_detector::utility_orphan_io::read_file_safe

use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

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
    path: &shared::common::taxonomy_path_vo::DirectoryPath,
    ignored_paths: &[String],
) -> Result<
    shared::common::taxonomy_paths_vo::FilePathList,
    shared::common::taxonomy_filesystem_error::FileSystemError,
> {
    let dir = std::path::Path::new(&path.value);
    if !dir.exists() || !dir.is_dir() {
        return Ok(shared::common::taxonomy_paths_vo::FilePathList { values: vec![] });
    }
    let files = collect_all_source_files(dir, ignored_paths);
    Ok(shared::common::taxonomy_paths_vo::FilePathList { values: files })
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

/// Filter FilePathList to only include source files.
pub fn filter_source_files(
    files: &shared::common::taxonomy_paths_vo::FilePathList,
) -> shared::common::taxonomy_paths_vo::FilePathList {
    let filtered: Vec<shared::common::taxonomy_path_vo::FilePath> = files
        .values
        .iter()
        .filter(|f| {
            let path = Path::new(&f.value);
            path.extension()
                .and_then(|e| e.to_str())
                .map(|ext| is_source_ext(ext))
                .unwrap_or(false)
        })
        .cloned()
        .collect();
    shared::common::taxonomy_paths_vo::FilePathList::new(filtered)
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

use dashmap::DashMap;
use rayon::prelude::*;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
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

// ─── Directory Mutation ────────────────────────────────────

/// Create a directory and all missing parent directories.
pub fn create_dir_all(path: &FilePath) -> Result<(), String> {
    std::fs::create_dir_all(path.value()).map_err(|e| e.to_string())
}

/// Remove a directory and all its contents recursively.
pub fn remove_dir_all(path: &FilePath) -> Result<(), String> {
    std::fs::remove_dir_all(path.value()).map_err(|e| e.to_string())
}

/// Walk directory recursively collecting only `.py` files.
/// Skips `target`, `.git`, `node_modules`, `.venv`.
pub fn walk_py_files(dir: &FilePath) -> Vec<FilePath> {
    let mut files = Vec::new();
    walk_py_files_inner(Path::new(dir.value()), &mut files);
    files
}

fn walk_py_files_inner(dir: &Path, files: &mut Vec<FilePath>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if name != "target" && name != ".git" && name != "node_modules" && name != ".venv" {
                    walk_py_files_inner(&path, files);
                }
            } else if path.is_file()
                && path.extension().and_then(|e| e.to_str()) == Some("py")
                && let Ok(fp) = FilePath::new(path.to_string_lossy().to_string())
            {
                files.push(fp);
            }
        }
    }
}

/// Find directories whose names match any of the given cache names.
pub fn find_cache_dirs(dir: &FilePath, cache_names: &[&str]) -> Vec<FilePath> {
    let mut found = Vec::new();
    find_cache_dirs_inner(Path::new(dir.value()), cache_names, &mut found);
    found
}

fn find_cache_dirs_inner(dir: &Path, cache_names: &[&str], found: &mut Vec<FilePath>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if cache_names.contains(&name) {
                    if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                        found.push(fp);
                    }
                } else if name != "target" && name != ".git" && name != "node_modules" {
                    find_cache_dirs_inner(&path, cache_names, found);
                }
            }
        }
    }
}

// ─── Async File I/O ─────────────────────────────────────────

/// Maximum config file size (1 MiB).
pub const MAX_CONFIG_FILE_SIZE: u64 = 1 << 20;

/// Async read file to string.
pub async fn read_file_async<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    tokio::fs::read_to_string(path).await
}

/// Read a file within the canonical root, enforcing path confinement and max file size.
pub async fn read_text_within_canonical_root<P: AsRef<Path>>(
    path: P,
    canonical_root: &Path,
) -> std::io::Result<String> {
    let path = path.as_ref();
    let canonical_path = tokio::fs::canonicalize(path).await?;
    if !canonical_path.starts_with(canonical_root) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "config path escapes allowed root",
        ));
    }
    let meta = tokio::fs::metadata(&canonical_path).await?;
    if !is_file(&canonical_path) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "config path is not a regular file",
        ));
    }
    if meta.len() > MAX_CONFIG_FILE_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "config file exceeds maximum allowed size",
        ));
    }
    tokio::fs::read_to_string(&canonical_path).await
}

// ─── Target Resolution ──────────────────────────────────────

/// Resolve target path: normalize "crates" to parent, keep "." as-is, etc.
pub fn resolve_target(path: Option<String>) -> String {
    match path {
        Some(p) => p,
        None => ".".to_string(),
    }
}

/// Detect source directory from project root (packages/, crates/, modules/).
/// If the path itself contains source files, return it directly.
pub fn detect_source_dir(project_root: &Path) -> PathBuf {
    if has_source_files(project_root) {
        return project_root.to_path_buf();
    }
    for name in &["packages", "crates", "modules"] {
        let candidate = project_root.join(name);
        if candidate.is_dir() {
            return candidate;
        }
    }
    project_root.to_path_buf()
}

/// Check if a directory contains source files directly (not in subdirectories).
fn has_source_files(dir: &Path) -> bool {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str()
                && (name.ends_with(".rs")
                    || name.ends_with(".py")
                    || name.ends_with(".ts")
                    || name.ends_with(".js"))
            {
                return true;
            }
        }
    }
    false
}

/// Collect source files (.rs, .py, .ts, .js, .tsx, .jsx) from a directory tree or single file.
pub fn collect_source_files(
    root_dir: &Path,
    _dir_path: &shared::common::taxonomy_path_vo::DirectoryPath,
    ignored: &[String],
) -> Vec<FilePath> {
    let mut files = Vec::new();
    if root_dir.is_dir() {
        walk_source_files(root_dir, &mut files, ignored);
    } else if root_dir.is_file() {
        if let Some(ext) = root_dir.extension().and_then(|e| e.to_str())
            && is_source_ext(ext)
        {
            let rel_path = root_dir.to_string_lossy();
            if !is_path_ignored(&rel_path, ignored)
                && let Ok(fp) = FilePath::new(rel_path.to_string())
            {
                files.push(fp);
            }
        }
    }
    files
}

// ─── Path Resolution (Member Detection) ─────────────────────

/// Detect if a path is a member directory (not a workspace root).
/// Returns true if the path is a single crate/module/package member:
/// - Rust: Cargo.toml without [workspace]
/// - Python: __init__.py or pyproject.toml present
/// - TypeScript: package.json present
pub fn is_member_path(path: &str) -> bool {
    let p = Path::new(path);

    // Rust: Cargo.toml without [workspace] means single crate member
    let cargo_toml = p.join("Cargo.toml");
    if cargo_toml.exists() {
        if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
            return !content.contains("[workspace]");
        }
        return true;
    }

    // Python: __init__.py or pyproject.toml means module member
    if p.join("__init__.py").exists() || p.join("pyproject.toml").exists() {
        return true;
    }

    // TypeScript: package.json means package member
    if p.join("package.json").exists() {
        return true;
    }

    false
}

/// Detect if a path is a leaf member directory (not a workspace root and not a group of members).
/// A leaf member has a marker file AND does NOT contain subdirectories that are also members.
/// Skips common source directories (src, lib, bin, tests, benches, examples) to avoid
/// false negatives when a member src/ contains __init__.py.
pub fn is_leaf_member_path(path: &str) -> bool {
    if !is_member_path(path) {
        return false;
    }
    let skip_dirs: &[&str] = &["src", "lib", "bin", "tests", "benches", "examples"];
    let p = Path::new(path);
    if let Ok(entries) = std::fs::read_dir(p) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let dir_name = entry.file_name().to_string_lossy().to_string();
                if skip_dirs.contains(&dir_name.as_str()) {
                    continue;
                }
                let sub_path = entry.path();
                if is_member_path(&sub_path.to_string_lossy()) {
                    return false;
                }
            }
        }
    }
    true
}

// ─── String-based Cache (for code-analysis compatibility) ───
// Separate from the PathBuf-based FILE_CACHE above.

static STRING_CACHE: LazyLock<DashMap<String, String>> = LazyLock::new(DashMap::new);

/// Populate the string-keyed file cache (for code-analysis compatibility).
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

// ─── File Metadata ────────────────────────────────────────

/// Get file/directory metadata.
pub fn metadata<P: AsRef<Path>>(path: P) -> std::io::Result<std::fs::Metadata> {
    std::fs::metadata(path)
}

/// Check if path is a symlink.
pub fn is_symlink<P: AsRef<Path>>(path: P) -> bool {
    std::fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

/// Set file permissions (Unix mode bits).
pub fn set_permissions<P: AsRef<Path>>(path: P, mode: u32) -> std::io::Result<()> {
    let mut perms = std::fs::metadata(&path)?.permissions();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        perms.set_mode(mode);
    }
    std::fs::set_permissions(path, perms)
}

/// Remove a file.
pub fn remove_file<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    std::fs::remove_file(path)
}

/// Read directory entries, returning Vec<FilePath>.
pub fn read_dir_entries(dir_path: &FilePath) -> Vec<FilePath> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = std::fs::read_dir(dir_path.value()) {
        for entry in read_dir.flatten() {
            if let Some(path_str) = entry.path().to_str() {
                if let Ok(fp) = FilePath::new(path_str.to_string()) {
                    entries.push(fp);
                }
            }
        }
    }
    entries
}

/// Read file content, returning empty string on error.
pub fn read_file_safe_str(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

// ─── Generic Read Dir ──────────────────────────────────────

/// Read directory entries (generic, returns Vec<String> of path strings).
pub fn read_dir_generic<P: AsRef<Path>>(dir: P) -> Vec<String> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = std::fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            if let Some(path_str) = entry.path().to_str() {
                entries.push(path_str.to_string());
            }
        }
    }
    entries
}

/// Generic create_dir_all (works with any AsRef<Path>).
pub fn create_dir_all_generic<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

// ─── Relocated Utility Functions ─────────────────────────────

/// Check if a file extension is lintable (supported: Python, JS, TS, Rust).
pub fn is_lintable_file(path: &FilePath) -> bool {
    matches!(
        path.extension().as_str(),
        "py" | "js" | "jsx" | "mjs" | "cjs" | "ts" | "tsx" | "mts" | "cts" | "rs"
    )
}

/// Collect file entries: (PathBuf, content_string) for each lintable file.
pub fn collect_file_entries(files: &[String]) -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    for file_str in files {
        let fp = match FilePath::new(file_str.clone()) {
            Ok(f) => f,
            Err(_) => continue,
        };
        if !is_lintable_file(&fp) {
            continue;
        }
        let content = match cache_get_by_str(&fp.value).map_or_else(|| read_file(&fp.value), Ok) {
            Ok(c) => c,
            Err(_) => continue,
        };
        out.push((PathBuf::from(&fp.value), content));
    }
    out
}

/// Detect ConfigLanguage from a file system path by checking for workspace type markers in the path.
pub fn detect_language_from_path(path: &str) -> ConfigLanguage {
    let path_buf = std::path::PathBuf::from(path);

    if path_exists(path_buf.join("Cargo.toml")) || path_contains_component(&path_buf, "crates") {
        return ConfigLanguage::Rust;
    }
    if path_exists(path_buf.join("package.json")) || path_contains_component(&path_buf, "packages")
    {
        return ConfigLanguage::TypeScript;
    }
    if path_exists(path_buf.join("pyproject.toml"))
        || path_exists(path_buf.join("setup.py"))
        || path_exists(path_buf.join("requirements.txt"))
        || path_contains_component(&path_buf, "modules")
    {
        return ConfigLanguage::Python;
    }

    ConfigLanguage::Rust
}

fn path_contains_component(path: &std::path::Path, component: &str) -> bool {
    path.components()
        .any(|c| matches!(c, std::path::Component::Normal(name) if name == component))
}

/// Read a file synchronously. Returns Ok(content) or Err(io::Error).
pub fn read_dependency_file(path: &std::path::Path) -> Result<String, std::io::Error> {
    read_file(path)
}

const MAX_CACHE_ENTRIES: usize = 20_000;

static FILE_CACHE_MAP: OnceLock<Mutex<std::collections::HashMap<String, String>>> = OnceLock::new();

fn file_cache_map() -> &'static Mutex<std::collections::HashMap<String, String>> {
    FILE_CACHE_MAP.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

/// Read file content using internal bounded cache.
pub fn read_cached(path: &FilePath) -> ContentString {
    let mut cache = file_cache_map().lock().unwrap_or_else(|e| e.into_inner());

    if let Some(content) = cache.get(path.value()) {
        return ContentString::new(content.clone());
    }

    let content = cache_get_by_str(path.value()).unwrap_or_else(|| read_file_safe(path.value()));

    if cache.len() < MAX_CACHE_ENTRIES {
        cache.insert(path.value().to_string(), content.clone());
    }

    ContentString::new(content)
}

/// Read directory entries as FilePath list.
pub fn read_dir(dir_path: &FilePath) -> Vec<FilePath> {
    let mut entries = Vec::new();
    for entry_str in read_dir_generic(dir_path.value()) {
        if let Ok(fp) = FilePath::new(entry_str) {
            entries.push(fp);
        }
    }
    entries
}

/// Clear bounded file cache.
pub fn clear_file_cache() {
    let mut cache = file_cache_map().lock().unwrap_or_else(|e| e.into_inner());
    cache.clear();
}

// ─── Migrated from utility_external_lint ─────────────────────

/// Canonicalize a path string to absolute, returning as String.
pub fn canonicalize_path_str(path_str: &str) -> String {
    canonicalize_path(path_str).to_string_lossy().to_string()
}

/// Create a default `"."` working directory, falling back to the given path if it fails.
pub fn default_working_dir(path: &FilePath) -> FilePath {
    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}

/// No-op apply_fix for linters that cannot auto-fix (scanners, type-checkers).
pub async fn noop_apply_fix() -> Result<
    shared::common::taxonomy_message_vo::ComplianceStatus,
    shared::code_analysis::taxonomy_operation_error::LinterOperationError,
> {
    Ok(shared::common::taxonomy_message_vo::ComplianceStatus::new(
        false,
    ))
}

/// Return true if the given path contains any Python (`.py`) files (recursive).
pub fn has_python_files_recursive(path: &FilePath) -> bool {
    let p = std::path::Path::new(&path.value);
    if !path_exists(p) {
        return p.extension().map(|e| e == "py").unwrap_or(false);
    }
    if is_file(p) {
        return p.extension().map(|e| e == "py").unwrap_or(false);
    }
    has_py_in_dir_recursive(p)
}

fn has_py_in_dir_recursive(dir: &std::path::Path) -> bool {
    for entry_path_str in read_dir_generic(dir) {
        let path = std::path::PathBuf::from(&entry_path_str);
        if is_dir(&path) {
            if has_py_in_dir_recursive(&path) {
                return true;
            }
        } else if path.extension().map(|e| e == "py").unwrap_or(false) {
            return true;
        }
    }
    false
}

/// Resolve the executable command for a JS tool (eslint, prettier, tsc).
/// Only uses local binary from node_modules/.bin — never falls back to npx/bunx.
/// Returns None if the tool is not installed locally.
pub fn resolve_js_cmd(
    executable: &str,
    args: Vec<String>,
    working_dir: &str,
) -> Option<Vec<String>> {
    let local_bin = Path::new(working_dir)
        .join("node_modules")
        .join(".bin")
        .join(executable);
    if path_exists(&local_bin) {
        let mut cmd = vec![local_bin.to_string_lossy().to_string()];
        cmd.extend(args);
        return Some(cmd);
    }
    None
}

/// Walk up from the given path to find the JS project root.
pub fn resolve_js_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    let abs_path = canonicalize_path(path_str);
    let mut current = if is_file(&abs_path) {
        abs_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."))
    } else {
        abs_path.clone()
    };
    for _ in 0..10 {
        if is_file(&current.join("lint_arwaky.config.yaml"))
            || is_file(&current.join("lint_arwaky.config.python.yaml"))
            || is_file(&current.join("package.json"))
            || is_dir(&current.join(".git"))
        {
            return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
        }
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => break,
        }
    }
    FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default()
}

/// Find parent dir with Cargo.toml (for cargo fmt, cargo clippy).
pub fn resolve_cargo_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }
    let current = Path::new(path_str);
    if is_dir(current) {
        if path_exists(&current.join("Cargo.toml")) {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if path_exists(&parent.join("Cargo.toml")) {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
        if let Some(grandparent) = parent.parent()
            && path_exists(&grandparent.join("Cargo.toml"))
        {
            return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
    }
    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}

/// Find parent dir with Cargo.lock (for cargo-audit).
pub fn resolve_cargo_lock_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if path_str.is_empty() {
        return path.clone();
    }
    let current = Path::new(path_str);
    if is_dir(current) {
        if path_exists(&current.join("Cargo.lock")) {
            return path.clone();
        }
    } else if let Some(parent) = current.parent() {
        if path_exists(&parent.join("Cargo.lock")) {
            return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
        if let Some(grandparent) = parent.parent()
            && path_exists(&grandparent.join("Cargo.lock"))
        {
            return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.clone());
        }
    }
    FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
}

// ─── Migrated from utility_workspace_scanner ──────────────────

/// Walk parent directories from `start` to locate the workspace root:
/// a directory that holds a member dir (crates/packages/modules) AND a
/// manifest (Cargo.toml / package.json / pyproject.toml).
pub fn find_workspace_root_from_path(
    start: &std::path::Path,
) -> Result<std::path::PathBuf, std::io::Error> {
    let member_dirs = ["crates", "packages", "modules"];
    let mut current = start.to_path_buf();
    loop {
        let has_cargo = path_exists(&current.join("Cargo.toml"));
        let has_package_json = path_exists(&current.join("package.json"));
        let has_pyproject = path_exists(&current.join("pyproject.toml"));
        let has_member_dir = member_dirs.iter().any(|d| is_dir(&current.join(d)));

        if has_member_dir && (has_cargo || has_package_json || has_pyproject) {
            return Ok(current);
        }

        if !current.pop() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "workspace root not found",
            ));
        }
    }
}

/// Returns true if any container/entry file under the workspace root references
/// one of `identifiers`.
pub fn check_wired_in_container(workspace_root: &std::path::Path, identifiers: &[String]) -> bool {
    for dir_name in &["crates", "packages", "modules"] {
        let dir = workspace_root.join(dir_name);
        if is_dir(&dir) && check_dir_containers(&dir, identifiers) {
            return true;
        }
    }
    false
}

fn check_dir_containers(dir: &std::path::Path, identifiers: &[String]) -> bool {
    if let Ok(fp) = FilePath::new(dir.to_str().unwrap_or("")) {
        let entries = read_dir(&fp);
        for entry_path in &entries {
            let path = std::path::Path::new(entry_path.value());
            if is_dir(&path) {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if matches!(
                    name,
                    "target"
                        | ".git"
                        | "node_modules"
                        | "dist"
                        | "build"
                        | "__pycache__"
                        | ".venv"
                        | "tests"
                ) {
                    continue;
                }

                if check_dir_containers(path, identifiers) {
                    return true;
                }
            } else if let Some(name) = path.file_name().and_then(|n| n.to_str())
                && (name.ends_with("_container.rs")
                    || name.ends_with("_container.py")
                    || name.ends_with("_container.ts")
                    || name.ends_with("_container.js")
                    || name.ends_with("_entry.rs")
                    || name.ends_with("_entry.py")
                    || name.ends_with("_entry.ts")
                    || name.ends_with("_entry.js"))
            {
                let fp = FilePath {
                    value: entry_path.value.clone(),
                };
                let content = read_cached(&fp).value;
                for id in identifiers {
                    if content.contains(id) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Walk directory and collect paths of all source files (*.rs, *.py, *.ts, *.js, etc.)
pub fn collect_source_files_from_path(dir: &std::path::Path, files: &mut Vec<String>) {
    if let Ok(fp) = FilePath::new(dir.to_str().unwrap_or("")) {
        let entries = read_dir(&fp);
        for entry_path in &entries {
            let path = std::path::Path::new(entry_path.value());
            if is_dir(&path) {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "target" || name == ".git" || name == "node_modules" || name == "tests" {
                    continue;
                }
                collect_source_files_from_path(path, files);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str())
                && matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
            {
                files.push(entry_path.value().to_string());
            }
        }
    }
}

// ─── Migrated from utility_git_io ────────────────────────────

/// Execute a git command and return stdout/stderr/success status.
pub fn run_git_command(args: &[&str], dir: &str) -> (String, String, bool) {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(dir)
        .output();

    match output {
        Ok(o) => (
            String::from_utf8_lossy(&o.stdout).to_string(),
            String::from_utf8_lossy(&o.stderr).to_string(),
            o.status.success(),
        ),
        Err(e) => (
            String::new(),
            format!("Failed to execute git: {}", e),
            false,
        ),
    }
}

/// Execute a git command asynchronously and return stdout/stderr/success status.
pub async fn run_git_command_async(args: &[&str], dir: &str) -> (String, String, bool) {
    let output = tokio::process::Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .await;

    match output {
        Ok(o) => (
            String::from_utf8_lossy(&o.stdout).to_string(),
            String::from_utf8_lossy(&o.stderr).to_string(),
            o.status.success(),
        ),
        Err(e) => (
            String::new(),
            format!("Failed to execute git: {}", e),
            false,
        ),
    }
}

/// Parse successful command output into trimmed non-empty lines.
pub fn parse_output_lines(output: &str) -> Vec<String> {
    output
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

// ─── Migrated from utility_dependency_io ──────────────────────

/// Execute an external command and return stdout/stderr/success status.
pub fn run_external_command(name: &str, args: &[&str]) -> (String, String, bool) {
    shared::common::utility_command_runner::run_command(name, args)
}

/// Execute an external command with a working directory and return stdout/stderr/success.
pub fn run_external_command_in(
    name: &str,
    args: &[&str],
    current_dir: &str,
) -> (String, String, bool) {
    shared::common::utility_command_runner::run_command_in_dir(name, args, Some(current_dir))
}

// ─── Migrated from utility_tui_io ────────────────────────────

/// Write text content to a file at the given path.
/// Returns Ok(()) on success, Err with OS error message on failure.
pub fn write_text_to_file(path: &std::path::Path, text: &str) -> Result<(), String> {
    write_file(path, text.as_bytes()).map_err(|e| format!("Failed to write file: {e}"))
}

/// Check if a binary is available in the system PATH.
pub fn is_binary_available(bin_name: &str) -> bool {
    if bin_name.is_empty()
        || bin_name
            .chars()
            .any(|c| !c.is_alphanumeric() && c != '_' && c != '-')
    {
        return false;
    }

    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
        .is_none_or(|dir| {
            let path = dir.join(bin_name);
            path_exists(path) || find_in_path(bin_name)
        })
}

fn find_in_path(bin_name: &str) -> bool {
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let path = dir.join(bin_name);
            if path_exists(path) {
                return true;
            }
        }
    }
    false
}

// ─── Migrated from utility_orphan_path ────────────────────────

/// Normalize a path lexically (resolve `.` and `..` without touching the filesystem).
pub fn normalize_lexical(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

/// Confine a candidate path under a root directory, canonicalizing both.
/// Returns None if the candidate escapes the root.
pub fn confine_under_root(root: &Path, candidate: &Path) -> Option<PathBuf> {
    let canonical_root = std::fs::canonicalize(root).ok()?;

    let absolute = if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        canonical_root.join(candidate)
    };

    if let Ok(canonical_candidate) = std::fs::canonicalize(&absolute) {
        return canonical_candidate
            .starts_with(&canonical_root)
            .then_some(canonical_candidate);
    }

    let parent = absolute.parent()?;
    let file_name = absolute.file_name()?;

    let canonical_parent = std::fs::canonicalize(parent).ok()?;
    let canonical_candidate = canonical_parent.join(file_name);

    canonical_candidate
        .starts_with(&canonical_root)
        .then_some(canonical_candidate)
}

/// Resolve a module path relative to a base directory, confined under root.
pub fn resolve_orphan_module_path(
    root: &Path,
    base_dir: &Path,
    module_path: &str,
) -> Option<PathBuf> {
    let candidate = if Path::new(module_path).is_absolute() {
        PathBuf::from(module_path)
    } else {
        base_dir.join(module_path)
    };
    confine_under_root(root, &candidate)
}

// ─── Migrated from utility_setup_io ───────────────────────────

/// Read directory entries, returning vector of PathBufs.
pub fn read_dir_entries_as_pathbuf(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut entries = Vec::new();
    for e in std::fs::read_dir(dir)?.flatten() {
        entries.push(e.path());
    }
    Ok(entries)
}
