// PURPOSE: FileSystemMaintenanceAdapter — infrastructure adapter for filesystem operations
use async_trait::async_trait;
use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_filesystem_maintenance_protocol::{
    FileEntry, IFileSystemMaintenanceProtocol,
};

// ─── Block 1: Struct Definition ───────────────────────────
pub struct FileSystemMaintenanceAdapter;

// ─── Block 2: Public Contract (domain port ONLY) ──────────
#[async_trait]
impl IFileSystemMaintenanceProtocol for FileSystemMaintenanceAdapter {
    async fn read_file(&self, path: &FilePath) -> Result<String, String> {
        std::fs::read_to_string(path.value()).map_err(|e| e.to_string())
    }

    async fn write_file(&self, path: &FilePath, content: &str) -> Result<(), String> {
        std::fs::write(path.value(), content).map_err(|e| e.to_string())
    }

    async fn create_dir_all(&self, path: &FilePath) -> Result<(), String> {
        std::fs::create_dir_all(path.value()).map_err(|e| e.to_string())
    }

    async fn path_exists(&self, path: &FilePath) -> bool {
        std::path::Path::new(path.value()).exists()
    }

    async fn file_exists(&self, path: &FilePath) -> bool {
        std::path::Path::new(path.value()).exists()
    }

    async fn walk_py_files(&self, dir: &FilePath) -> Vec<FilePath> {
        let mut results = Vec::new();
        self.walk_dir_recursive(dir.value(), &mut results, 0, 5);
        results
    }

    async fn find_cache_dirs(&self, dir: &FilePath, cache_names: &[&str]) -> Vec<FilePath> {
        let mut results = Vec::new();
        self.find_cache_dirs_recursive(dir.value(), cache_names, &mut results);
        results
    }

    async fn remove_dir_all(&self, path: &FilePath) -> Result<(), String> {
        std::fs::remove_dir_all(path.value()).map_err(|e| e.to_string())
    }

    async fn list_dir(&self, dir: &FilePath) -> Vec<FileEntry> {
        let mut entries = Vec::new();
        if let Ok(read_dir) = std::fs::read_dir(dir.value()) {
            for entry in read_dir.flatten() {
                let path = entry.path();
                let is_dir = path.is_dir();
                if let Some(_name) = path.file_name().and_then(|n| n.to_str()) {
                    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap_or_default();
                    entries.push(FileEntry { path: fp, is_dir });
                }
            }
        }
        entries
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl FileSystemMaintenanceAdapter {
    pub fn new() -> Self {
        Self
    }

    fn walk_dir_recursive(
        &self,
        dir: &str,
        results: &mut Vec<FilePath>,
        depth: usize,
        max_depth: usize,
    ) {
        if depth > max_depth {
            return;
        }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if !name.starts_with('.') && name != "target" && name != "node_modules" {
                        self.walk_dir_recursive(
                            &path.to_string_lossy(),
                            results,
                            depth + 1,
                            max_depth,
                        );
                    }
                } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext == "py" {
                        if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                            results.push(fp);
                        }
                    }
                }
            }
        }
    }

    fn find_cache_dirs_recursive(
        &self,
        dir: &str,
        cache_names: &[&str],
        results: &mut Vec<FilePath>,
    ) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if cache_names.contains(&name.as_ref()) {
                        if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                            results.push(fp);
                        }
                    } else if !name.starts_with('.') && name != "target" && name != "node_modules" {
                        self.find_cache_dirs_recursive(
                            &path.to_string_lossy(),
                            cache_names,
                            results,
                        );
                    }
                }
            }
        }
    }
}

impl Default for FileSystemMaintenanceAdapter {
    fn default() -> Self {
        Self::new()
    }
}
