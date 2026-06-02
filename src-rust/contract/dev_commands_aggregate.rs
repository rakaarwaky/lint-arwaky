use async_trait::async_trait;
use super::*;

#[async_trait]
pub trait DevCommandsAggregate: Send + Sync {
    async fn diff(&self, path1: FilePath, path2: FilePath, output_format: FileFormat);
    async fn suggest(&self, path: FilePath, ai: bool);
    async fn ignore(&self, rule: &str, remove: bool, path: Option<FilePath>);
    async fn config(&self, action: &str, path: Option<FilePath>);
    async fn export(&self, output_format: FileFormat, output: Option<FilePath>);
    async fn init(&self, path: Option<FilePath>);
    async fn install_hook(&self, path: Option<FilePath>);
    async fn uninstall_hook(&self, path: Option<FilePath>);
}
