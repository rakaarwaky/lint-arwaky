// PURPOSE: IGitCommandPort — port trait for executing git commands
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

pub struct GitCommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

#[async_trait]
pub trait IGitCommandPort: Send + Sync {
    async fn run_git(&self, args: &[&str], dir: &FilePath) -> GitCommandOutput;
    async fn symbolic_ref(&self, dir: &FilePath) -> Option<String>;
    async fn diff_name_only(&self, range: &str, dir: &FilePath) -> Vec<String>;
    async fn ls_files_modified(&self, dir: &FilePath) -> Vec<String>;
}
