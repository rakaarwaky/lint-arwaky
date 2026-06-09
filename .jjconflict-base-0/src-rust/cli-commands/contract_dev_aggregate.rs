use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
use crate::output_report::taxonomy_score_vo::FileFormat;
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::shared_common::taxonomy_layer_vo::Identity;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

pub type DevExecutorPort = Box<dyn ICommandExecutorPort>;

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
