// PURPOSE: IToolExecutorProtocol — protocol trait for executing external tools and capturing output
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

pub struct ToolOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

#[async_trait]
pub trait IToolExecutorProtocol: Send + Sync {
    async fn run_tool(&self, name: &str, args: &[&str]) -> ToolOutput;
    async fn run_tool_in_dir(&self, name: &str, args: &[&str], dir: &FilePath) -> ToolOutput;
    async fn tool_exists(&self, name: &str) -> bool;
    async fn get_binary_path(&self) -> FilePath;
}
