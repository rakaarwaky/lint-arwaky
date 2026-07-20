// PURPOSE: IFileSystemMaintenanceProtocol — protocol trait for filesystem operations needed by maintenance
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

pub struct FileEntry {
    pub path: FilePath,
    pub is_dir: bool,
}

#[async_trait]
pub trait IFileSystemMaintenanceProtocol: Send + Sync {
    async fn read_file(&self, path: &FilePath) -> Result<String, String>;
    async fn write_file(&self, path: &FilePath, content: &str) -> Result<(), String>;
    async fn create_dir_all(&self, path: &FilePath) -> Result<(), String>;
    async fn path_exists(&self, path: &FilePath) -> bool;
    async fn file_exists(&self, path: &FilePath) -> bool;
    async fn walk_py_files(&self, dir: &FilePath) -> Vec<FilePath>;
    async fn find_cache_dirs(&self, dir: &FilePath, cache_names: &[&str]) -> Vec<FilePath>;
    async fn remove_dir_all(&self, path: &FilePath) -> Result<(), String>;
    async fn list_dir(&self, dir: &FilePath) -> Vec<FileEntry>;
}
