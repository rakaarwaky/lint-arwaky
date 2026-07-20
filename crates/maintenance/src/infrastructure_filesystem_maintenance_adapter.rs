// PURPOSE: FileSystemMaintenanceAdapter — IFileSystemMaintenanceProtocol implementation for maintenance filesystem ops
use std::fs;
use std::path::Path;

use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_filesystem_maintenance_protocol::{
    FileEntry, IFileSystemMaintenanceProtocol,
};

// Block 1: struct Definition
pub struct FileSystemMaintenanceAdapter;

// Block 2: impl Port for Struct (Public Contract)
#[async_trait::async_trait]
impl IFileSystemMaintenanceProtocol for FileSystemMaintenanceAdapter {
    async fn read_file(&self, path: &FilePath) -> Result<String, String> {
        fs::read_to_string(path.value()).map_err(|e| e.to_string())
    }

    async fn write_file(&self, path: &FilePath, content: &str) -> Result<(), String> {
        fs::write(path.value(), content).map_err(|e| e.to_string())
    }

    async fn create_dir_all(&self, path: &FilePath) -> Result<(), String> {
        fs::create_dir_all(path.value()).map_err(|e| e.to_string())
    }

    async fn path_exists(&self, path: &FilePath) -> bool {
        Path::new(path.value()).exists()
    }

    async fn file_exists(&self, path: &FilePath) -> bool {
        Path::new(path.value()).exists()
    }

    async fn walk_py_files(&self, dir: &FilePath) -> Vec<FilePath> {
        let mut files = Vec::new();
        walk_py_files_inner(Path::new(dir.value()), &mut files);
        files
    }

    async fn find_cache_dirs(&self, dir: &FilePath, cache_names: &[&str]) -> Vec<FilePath> {
        let mut found = Vec::new();
        find_cache_dirs_inner(Path::new(dir.value()), cache_names, &mut found);
        found
    }

    async fn remove_dir_all(&self, path: &FilePath) -> Result<(), String> {
        fs::remove_dir_all(path.value()).map_err(|e| e.to_string())
    }

    async fn list_dir(&self, dir: &FilePath) -> Vec<FileEntry> {
        let mut entries = Vec::new();
        if let Ok(read_dir) = fs::read_dir(dir.value()) {
            for entry in read_dir.flatten() {
                let path = entry.path();
                let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap_or_default();
                entries.push(FileEntry {
                    path: fp,
                    is_dir: path.is_dir(),
                });
            }
        }
        entries
    }
}

// Block 3: constructors & helpers
impl FileSystemMaintenanceAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileSystemMaintenanceAdapter {
    fn default() -> Self {
        Self::new()
    }
}

fn walk_py_files_inner(dir: &Path, files: &mut Vec<FilePath>) {
    if let Ok(entries) = fs::read_dir(dir) {
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
            } else if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("py") {
                if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                    files.push(fp);
                }
            }
        }
    }
}

fn find_cache_dirs_inner(dir: &Path, cache_names: &[&str], found_dirs: &mut Vec<FilePath>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if cache_names.contains(&name) {
                    if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                        found_dirs.push(fp);
                    }
                } else if name != "target" && name != ".git" && name != "node_modules" {
                    find_cache_dirs_inner(&path, cache_names, found_dirs);
                }
            }
        }
    }
}
