// PURPOSE: IGitFileCheckPort — port trait for file/directory existence checks
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IGitFileCheckPort: Send + Sync {
    async fn path_exists(&self, path: &FilePath) -> bool;
    async fn is_file(&self, path: &FilePath) -> bool;
    async fn is_dir(&self, path: &FilePath) -> bool;
}
