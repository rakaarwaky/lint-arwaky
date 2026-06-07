use crate::contract::directory_watch_aggregate::DirectoryWatchAggregate;
use crate::contract::multi_project_aggregate::MultiProjectAggregate;
use crate::contract::PipelineOutputAggregate;
use crate::taxonomy::BooleanVO;
use crate::taxonomy::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait PipelineExtendedOrchestratorAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn execute_multi_project(
        &self,
        request: MultiProjectAggregate,
        use_retry: Option<BooleanVO>,
        config_path: Option<&FilePath>,
    ) -> Box<dyn PipelineOutputAggregate>;
    async fn execute_watch(
        &self,
        request: DirectoryWatchAggregate,
    ) -> Box<dyn PipelineOutputAggregate>;
}
