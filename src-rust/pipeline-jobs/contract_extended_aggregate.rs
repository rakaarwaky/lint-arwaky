// PURPOSE: ExtendedPipelineAggregate — aggregate trait for extended pipeline initialization
use crate::file_watch::contract_watch_aggregate::DirectoryWatchAggregate;
use crate::multi_project::contract_project_aggregate::MultiProjectAggregate;
use crate::pipeline_jobs::contract_output_aggregate::PipelineOutputAggregate;
use crate::shared_common::taxonomy_common_vo::BooleanVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
