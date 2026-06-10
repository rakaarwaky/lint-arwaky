// PURPOSE: Aggregate: Report aggregation and wiring
use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
use crate::output_report::taxonomy_score_vo::FileFormat;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;
5|
pub type ReportExecutorPort = Box<dyn ICommandExecutorPort>;
7|
#[async_trait]
pub trait ReportCommandsAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn report(&self, path: &FilePath, output_format: &FileFormat);
    async fn security(&self, path: &FilePath);
}
14|