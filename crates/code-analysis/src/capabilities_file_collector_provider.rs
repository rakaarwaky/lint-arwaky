// PURPOSE: FileCollectorProvider — IScannerProviderProtocol implementation for collecting source files
// Infrastructure layer: has I/O (fs::read_dir, symlink handling).
// Uses taxonomy utilities for pure logic (is_source_file, is_ignored_dir, etc.).

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use shared::common::contract_scanner_provider_protocol::IScannerProviderProtocol;
use shared::common::utility_file::{
    collect_source_file, is_ignored_dir, is_path_ignored, is_source_file,
};
use shared::common::taxonomy_filesystem_error::FileSystemError;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::default_aes_config;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct FileCollectorProvider;

// ─── Block 2: Public Contract (domain port ONLY) ──────────
impl IScannerProviderProtocol for FileCollectorProvider {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError> {
        let dir = Path::new(&path.value);
        let mut files = Vec::new();
        if !dir.exists() || !dir.is_dir() {
            return Ok(FilePathList { values: files });
        }
        let ignored = default_ignored_paths();
        walk_source_files(dir, &mut files, &ignored);
        Ok(FilePathList { values: files })
    }

    fn get_ignored_files(&self) -> FilePathList {
        FilePathList { values: vec![] }
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for FileCollectorProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FileCollectorProvider {
    pub fn new() -> Self {
        Self
    }
}

pub fn collect_all_source_files(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        let ignored = default_ignored_paths();
        walk_source_files(dir, &mut files, &ignored);
    }
    files
}

pub fn collect_all_source_files_raw(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        let ignored: Vec<String> = Vec::new();
        walk_source_files(dir, &mut files, &ignored);
    }
    files
}

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
