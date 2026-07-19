// PURPOSE: FileCollectorProvider — IScannerProviderPort implementation for collecting source files
// Main implementator of file collection. Wired to other crates via DI through root containers.

use shared::common::contract_scanner_provider_port::IScannerProviderPort;
use shared::common::taxonomy_file_utility::{default_ignored_paths, walk_source_files};
use shared::common::taxonomy_filesystem_error::FileSystemError;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct FileCollectorProvider;

// ─── Block 2: Public Contract (domain port ONLY) ──────────
impl IScannerProviderPort for FileCollectorProvider {
    fn scan_directory(&self, path: &DirectoryPath) -> Result<FilePathList, FileSystemError> {
        let dir = std::path::Path::new(&path.value);
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

pub fn collect_all_source_files(dir: &std::path::Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        let ignored = default_ignored_paths();
        walk_source_files(dir, &mut files, &ignored);
    }
    files
}

pub fn collect_all_source_files_raw(dir: &std::path::Path) -> Vec<FilePath> {
    let mut files = Vec::new();
    if dir.exists() && dir.is_dir() {
        let ignored: Vec<String> = Vec::new();
        walk_source_files(dir, &mut files, &ignored);
    }
    files
}
