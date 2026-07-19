// PURPOSE: IGitFileCheckPort — port trait for file/directory existence checks
use async_trait::async_trait;

#[async_trait]
pub trait IGitFileCheckPort: Send + Sync {
    async fn path_exists(&self, path: &str) -> bool;
    async fn is_file(&self, path: &str) -> bool;
    async fn is_dir(&self, path: &str) -> bool;
}
