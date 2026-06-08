use crate::contract::ServiceContainerAggregate;
use crate::taxonomy::BooleanVO;
use crate::taxonomy::FileFormat;
use crate::taxonomy::FilePath;
use crate::taxonomy::Identity;
use async_trait::async_trait;

#[async_trait]
pub trait DevCommandsAggregate: Send + Sync {
    async fn diff(&self, path1: FilePath, path2: FilePath, output_format: FileFormat);
    async fn suggest(&self, path: FilePath, ai: BooleanVO);
    async fn ignore(&self, rule: &Identity, remove: BooleanVO, path: Option<FilePath>);
    async fn config(&self, action: &Identity, path: Option<FilePath>);
    async fn export(&self, output_format: FileFormat, output: Option<FilePath>);
    async fn init(&self, path: Option<FilePath>);
    async fn install_hook(&self, path: Option<FilePath>);
    async fn uninstall_hook(&self, path: Option<FilePath>);
}
