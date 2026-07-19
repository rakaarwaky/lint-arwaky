// PURPOSE: FileSystemMaintenanceAdapter — infrastructure adapter for filesystem operations
use async_trait::async_trait;
use shared::project_setup::contract_filesystem_maintenance_port::{
    FileEntry, IFileSystemMaintenancePort,
};

// ─── Block 1: Struct Definition ───────────────────────────
pub struct FileSystemMaintenanceAdapter;

// ─── Block 2: Public Contract (domain port ONLY) ──────────
#[async_trait]
impl IFileSystemMaintenancePort for FileSystemMaintenanceAdapter {
    async fn read_file(&self, path: &str) -> Result<String, String> {
        std::fs::read_to_string(path).map_err(|e| e.to_string())
    }

    async fn write_file(&self, path: &str, content: &str) -> Result<(), String> {
        std::fs::write(path, content).map_err(|e| e.to_string())
    }

    async fn create_dir_all(&self, path: &str) -> Result<(), String> {
        std::fs::create_dir_all(path).map_err(|e| e.to_string())
    }

    async fn path_exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    async fn file_exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    async fn walk_py_files(&self, dir: &str) -> Vec<String> {
        let mut results = Vec::new();
        self.walk_dir_recursive(dir, &mut results, 0, 5);
        results
    }

    async fn find_cache_dirs(&self, dir: &str, cache_names: &[&str]) -> Vec<String> {
        let mut results = Vec::new();
        self.find_cache_dirs_recursive(dir, cache_names, &mut results);
        results
    }

    async fn remove_dir_all(&self, path: &str) -> Result<(), String> {
        std::fs::remove_dir_all(path).map_err(|e| e.to_string())
    }

    async fn list_dir(&self, dir: &str) -> Vec<FileEntry> {
        let mut entries = Vec::new();
        if let Ok(read_dir) = std::fs::read_dir(dir) {
            for entry in read_dir.flatten() {
                let path = entry.path();
                let is_dir = path.is_dir();
                if let Some(_name) = path.file_name().and_then(|n| n.to_str()) {
                    entries.push(FileEntry {
                        path: path.to_string_lossy().to_string(),
                        is_dir,
                    });
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
        results: &mut Vec<String>,
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
                        results.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    fn find_cache_dirs_recursive(
        &self,
        dir: &str,
        cache_names: &[&str],
        results: &mut Vec<String>,
    ) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if cache_names.contains(&name.as_ref()) {
                        results.push(path.to_string_lossy().to_string());
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
