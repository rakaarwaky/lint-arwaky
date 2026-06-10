// PURPOSE: DispatcherAggregate — aggregate trait for dispatching pipeline jobs to executor ports
use crate::pipeline_jobs::taxonomy_action_vo::ActionArgs;
use crate::pipeline_jobs::taxonomy_action_vo::ActionName;
use crate::pipeline_jobs::taxonomy_job_vo::ResponseData;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::cli_commands::contract_executor_port::ICommandExecutorPort as _;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait PipelineActionDispatcherAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn dispatch(&self, action: &ActionName, args: ActionArgs) -> ResponseData;
    fn validate_action(&self, action: &ActionName) -> SuccessStatus;
}
