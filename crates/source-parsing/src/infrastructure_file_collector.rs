// PURPOSE: FileCollector — collects source files recursively with directory exclusion support
use std::fs;
use std::path::{Path, PathBuf};

use shared::config_system::taxonomy_config_vo::default_aes_config;
use shared::file_system::taxonomy_filesystem_error::FileSystemError;
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use shared::source_parsing::taxonomy_file_collector_helper::is_path_ignored;
use shared::source_parsing::taxonomy_path_vo::DirectoryPath;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;

pub struct FileCollectorProvider {}

impl Default for FileCollectorProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FileCollectorProvider {
    pub fn new() -> Self {
        Self {}
    }
}

fn default_ignored_paths() -> Vec<String> {
    let config = default_aes_config();
    config
        .ignored_paths
        .values
        .iter()
        .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
        .collect()
}

pub fn collect_all_source_files(dir: &Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        walk_source_files(dir, &mut files, &[]);
    }
    files
}

impl IScannerProviderPort for FileCollectorProvider {
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

fn is_source_file(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

fn is_ignored_dir(dir: &Path, ignored: &[String]) -> bool {
    let s = dir.to_string_lossy();
    is_path_ignored(&s, ignored)
}

fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if is_ignored_dir(&path, ignored) {
                continue;
            }
            if path.is_dir() {
                walk_source_files(&path, files, ignored);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if is_source_file(ext) {
                    if let Some(path_str) = path.to_str() {
                        if let Ok(fp) = FilePath::new(path_str.to_string()) {
                            files.push(fp);
                        }
                    }
                }
            }
        }
    }
}

/// Walk only .rs files recursively, calling `cb` for each.
pub fn walk_rs_files(dir: &Path, cb: &mut dyn FnMut(PathBuf), ignored: &[String]) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if is_ignored_dir(&p, ignored) {
                continue;
            }
            if p.is_dir() {
                walk_rs_files(&p, cb, ignored);
            } else if matches!(p.extension().and_then(|e| e.to_str()), Some("rs")) {
                cb(p);
            }
        }
    }
}

/// Count total lines in all .rs files under a directory.
pub fn count_loc(path: &str) -> usize {
    let src = Path::new(path);
    let ignored = default_ignored_paths();
    let mut count = 0usize;
    walk_rs_files(
        src,
        &mut |p| {
            if let Ok(c) = fs::read_to_string(&p) {
                count += c.lines().count();
            }
        },
        &ignored,
    );
    count.max(1)
}
