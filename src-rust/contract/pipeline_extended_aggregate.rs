use crate::taxonomy::{FilePath, PipelineOutputAggregate};
use crate::contract::{crate::contract::multi_project_aggregate::MultiProjectAggregate, crate::contract::directory_watch_aggregate::DirectoryWatchAggregate};
use async_trait::async_trait;

#[async_trait]
pub trait PipelineExtendedOrchestratorAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn execute_multi_project(&self, request: MultiProjectAggregate, use_retry: Option<bool>, config_path: Option<&FilePath>) -> PipelineOutputAggregate;
    async fn execute_watch(&self, request: DirectoryWatchAggregate) -> PipelineOutputAggregate;
}
