// PURPOSE: File & workspace utility — pure logic + I/O, free functions only
// Single source of truth for file walking, ignored path matching, source file detection,
// and workspace root detection.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::common::taxonomy_filesystem_error::FileSystemError;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::default_aes_config;

/// Check if a file extension is a known source file.
pub fn is_source_file(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

/// Check if a directory is in the ignored list.
pub fn is_ignored_dir(dir: &Path, ignored: &[String]) -> bool {
    let s = dir.to_string_lossy();
    is_path_ignored(&s, ignored)
}

/// Collect a single source file path into the output vector.
pub fn collect_source_file(path: &Path, files: &mut Vec<FilePath>) {
    if let Some(path_str) = path.to_str() {
        if let Ok(fp) = FilePath::new(path_str.to_string()) {
            files.push(fp);
        }
    }
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
        if pat.starts_with("*.") || (pat.starts_with('.') && pat.contains('.')) {
            let suffix = if let Some(s) = pat.strip_prefix('*') {
                s.trim_start_matches('.')
            } else {
                pat.trim_start_matches('.')
            };
            if suffix.is_empty() {
                continue;
            }
            let basename = segments.last().copied().unwrap_or_default();
            if basename.ends_with(suffix) {
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

/// Build default ignored paths from config.
pub fn default_ignored_paths() -> Vec<String> {
    let config = default_aes_config();
    config
        .ignored_paths
        .values
        .iter()
        .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
        .collect()
}

#[cfg(unix)]
fn get_inode(meta: &std::fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    meta.ino()
}

#[cfg(not(unix))]
fn get_inode(_meta: &std::fs::Metadata) -> u64 {
    0
}

/// Collect all lintable source files from a directory tree.
pub fn collect_all_source_files(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        let ignored = default_ignored_paths();
        walk_source_files(dir, &mut files, &ignored);
    }
    files
}

/// Collect all lintable source files without applying default ignores.
pub fn collect_all_source_files_raw(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        let ignored: Vec<String> = Vec::new();
        walk_source_files(dir, &mut files, &ignored);
    }
    files
}

/// Scan a directory and return files as FilePathList (replaces IScannerProviderPort).
pub fn scan_directory(path: &DirectoryPath) -> Result<FilePathList, FileSystemError> {
    let dir = std::path::Path::new(&path.value);
    if !dir.exists() || !dir.is_dir() {
        return Ok(FilePathList { values: vec![] });
    }
    let files = collect_all_source_files(dir);
    Ok(FilePathList { values: files })
}

/// Walk a directory tree collecting all source files, skipping ignored directories.
pub fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    let mut visited = HashSet::new();
    walk_source_files_inner(dir, files, ignored, &mut visited)
}

fn walk_source_files_inner(
    dir: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
    visited: &mut HashSet<u64>,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_ignored_dir(&path, ignored) {
                continue;
            }
            if let Ok(sym_meta) = std::fs::symlink_metadata(&path) {
                if sym_meta.file_type().is_symlink() {
                    if let Ok(target) = std::fs::canonicalize(&path) {
                        if let Ok(target_meta) = target.metadata() {
                            let inode = get_inode(&target_meta);
                            if !visited.insert(inode) {
                                continue;
                            }
                            if target_meta.is_dir() {
                                walk_source_files_inner(&target, files, ignored, visited);
                            } else if target_meta.is_file() {
                                collect_source_file(&target, files);
                            }
                        }
                    }
                    continue;
                }
            }
            if path.is_dir() {
                let dir_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                if dir_name == "tests" {
                    continue;
                }
                if let Ok(meta) = fs::metadata(&path) {
                    let inode = get_inode(&meta);
                    if !visited.insert(inode) {
                        continue;
                    }
                }
                walk_source_files_inner(&path, files, ignored, visited);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if is_source_file(ext) {
                    collect_source_file(&path, files);
                }
            }
        }
    }
}

/// Walk a directory tree collecting all .rs files.
pub fn walk_rs_files(dir: &Path, cb: &mut dyn FnMut(PathBuf), ignored: &[String]) {
    let mut visited = HashSet::new();
    walk_rs_files_inner(dir, cb, ignored, &mut visited)
}

fn walk_rs_files_inner(
    dir: &Path,
    cb: &mut dyn FnMut(PathBuf),
    ignored: &[String],
    visited: &mut HashSet<u64>,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if is_ignored_dir(&p, ignored) {
                continue;
            }
            if let Ok(sym_meta) = std::fs::symlink_metadata(&p) {
                if sym_meta.file_type().is_symlink() {
                    if let Ok(target) = std::fs::canonicalize(&p) {
                        if let Ok(target_meta) = target.metadata() {
                            let inode = get_inode(&target_meta);
                            if !visited.insert(inode) {
                                continue;
                            }
                            if target_meta.is_dir() {
                                walk_rs_files_inner(&target, cb, ignored, visited);
                            } else if target_meta.is_file()
                                && matches!(target.extension().and_then(|e| e.to_str()), Some("rs"))
                            {
                                cb(target);
                            }
                        }
                    }
                    continue;
                }
            }
            if p.is_dir() {
                if let Ok(meta) = fs::metadata(&p) {
                    let inode = get_inode(&meta);
                    if !visited.insert(inode) {
                        continue;
                    }
                }
                walk_rs_files_inner(&p, cb, ignored, visited);
            } else if matches!(p.extension().and_then(|e| e.to_str()), Some("rs")) {
                cb(p);
            }
        }
    }
}

/// Walk up from `start` looking for workspace root markers.
/// Returns the first directory containing Cargo.toml, crates/, packages/, or modules/.
pub fn find_workspace_root(start: &str) -> Option<std::path::PathBuf> {
    let mut dir = std::path::Path::new(start).to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }
    loop {
        if dir.join("Cargo.toml").exists()
            || dir.join("crates").is_dir()
            || dir.join("packages").is_dir()
            || dir.join("modules").is_dir()
        {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}
