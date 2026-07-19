// PURPOSE: IGitCommandPort — port trait for executing git commands
use async_trait::async_trait;

pub struct GitCommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

#[async_trait]
pub trait IGitCommandPort: Send + Sync {
    async fn run_git(&self, args: &[&str], dir: &str) -> GitCommandOutput;
    async fn symbolic_ref(&self, dir: &str) -> Option<String>;
    async fn diff_name_only(&self, range: &str, dir: &str) -> Vec<String>;
    async fn ls_files_modified(&self, dir: &str) -> Vec<String>;
}
