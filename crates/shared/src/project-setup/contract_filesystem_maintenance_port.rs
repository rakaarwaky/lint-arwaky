// PURPOSE: IFileSystemMaintenancePort — port trait for filesystem operations needed by maintenance
use async_trait::async_trait;

pub struct FileEntry {
    pub path: String,
    pub is_dir: bool,
}

#[async_trait]
pub trait IFileSystemMaintenancePort: Send + Sync {
    async fn read_file(&self, path: &str) -> Result<String, String>;
    async fn write_file(&self, path: &str, content: &str) -> Result<(), String>;
    async fn create_dir_all(&self, path: &str) -> Result<(), String>;
    async fn path_exists(&self, path: &str) -> bool;
    async fn file_exists(&self, path: &str) -> bool;
    async fn walk_py_files(&self, dir: &str) -> Vec<String>;
    async fn find_cache_dirs(&self, dir: &str, cache_names: &[&str]) -> Vec<String>;
    async fn remove_dir_all(&self, path: &str) -> Result<(), String>;
    async fn list_dir(&self, dir: &str) -> Vec<FileEntry>;
}
