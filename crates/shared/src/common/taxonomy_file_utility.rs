// PURPOSE: Stateless utility functions for file collection and walking
// Domain-agnostic, no &self, no business logic — belongs in shared/taxonomy

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_vo::default_aes_config;

/// Check if a file extension is a known source file.
pub fn is_source_file(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

/// Check if a directory is in the ignored list.
pub fn is_ignored_dir(dir: &Path, ignored: &[String]) -> bool {
    let s = dir.to_string_lossy();
    if let Ok(fp) = FilePath::new(s.to_string()) {
        fp.is_ignored(ignored)
    } else {
        false
    }
}

#[cfg(unix)]
pub fn get_inode(meta: &std::fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    meta.ino()
}

#[cfg(not(unix))]
pub fn get_inode(_meta: &std::fs::Metadata) -> u64 {
    0
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

/// Collect a single source file path into the output vector.
pub fn collect_source_file(path: &Path, files: &mut Vec<FilePath>) {
    if let Some(path_str) = path.to_str() {
        if let Ok(fp) = FilePath::new(path_str.to_string()) {
            files.push(fp);
        }
    }
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
