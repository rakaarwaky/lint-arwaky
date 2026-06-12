// PURPOSE: ExtendedPipelineAggregate — aggregate trait for extended pipeline initialization
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::file_watch::taxonomy_watch_vo::DirectoryWatchVO;
use crate::multi_project::taxonomy_multi_project_vo::MultiProjectVO;
use crate::pipeline_jobs::contract_pipeline_output_aggregate::PipelineOutputAggregate;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait PipelineExtendedOrchestratorAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn execute_multi_project(
        &self,
        request: MultiProjectVO,
        use_retry: Option<BooleanVO>,
        config_path: Option<&FilePath>,
    ) -> Box<dyn PipelineOutputAggregate>;
    async fn execute_watch(&self, request: DirectoryWatchVO) -> Box<dyn PipelineOutputAggregate>;
}
