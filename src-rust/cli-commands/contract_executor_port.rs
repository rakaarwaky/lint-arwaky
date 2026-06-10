// PURPOSE: Port: Interface for Executor
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::shared_common::taxonomy_common_vo::PatternList;
use crate::shared_common::taxonomy_duration_vo::Timeout;
/// contract — Port for executing external commands.
use crate::source_parsing::taxonomy_path_vo::FilePath;
6|
#[async_trait::async_trait]
pub trait ICommandExecutorPort: Send + Sync {
    /// Execute a command and return the response.
    async fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData>;
16|
    /// Check the health of the execution transport.
    async fn health_check(&self) -> anyhow::Result<ResponseData>;
}
20|