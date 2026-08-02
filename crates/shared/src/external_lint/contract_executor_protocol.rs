// PURPOSE: Port: ICommandExecutorProtocol — trait for executing shell commands and capturing response
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_response_data_vo::ResponseData;

pub trait ICommandExecutorProtocol: Send + Sync {
    /// Execute a command and return the response.
    fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData>;

    /// Check the health of the execution transport.
    fn health_check(&self) -> anyhow::Result<ResponseData>;
}
