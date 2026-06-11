// PURPOSE: ReportCommandsAggregate — aggregate trait for report and security operations
use cli_transport::contract_executor_port::ICommandExecutorPort;
use output_report::taxonomy_score_vo::FileFormat;
use source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

pub type ReportExecutorPort = Box<dyn ICommandExecutorPort>;

#[async_trait]
pub trait ReportCommandsAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn report(&self, path: &FilePath, output_format: &FileFormat);
    async fn security(&self, path: &FilePath);
}
