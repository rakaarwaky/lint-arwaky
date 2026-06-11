// PURPOSE: DispatcherAggregate — aggregate trait for dispatching pipeline jobs to executor ports
use pipeline_jobs::taxonomy_action_vo::ActionArgs;
use pipeline_jobs::taxonomy_action_vo::ActionName;
use pipeline_jobs::taxonomy_job_vo::ResponseData;
use pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait PipelineActionDispatcherAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn dispatch(&self, action: &ActionName, args: ActionArgs) -> ResponseData;
    fn validate_action(&self, action: &ActionName) -> SuccessStatus;
}
