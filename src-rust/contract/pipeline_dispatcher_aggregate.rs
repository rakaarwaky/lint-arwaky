use crate::taxonomy::source_path_vo::FilePath;
use crate::taxonomy::{ActionArgs, ActionName, ResponseData, SuccessStatus};
use async_trait::async_trait;

#[async_trait]
pub trait PipelineActionDispatcherAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn dispatch(&self, action: &ActionName, args: ActionArgs) -> ResponseData;
    fn validate_action(&self, action: &ActionName) -> SuccessStatus;
}
