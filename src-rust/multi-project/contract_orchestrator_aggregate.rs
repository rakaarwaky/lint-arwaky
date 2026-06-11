// PURPOSE: MultiProjectOrchestratorAggregate — aggregate trait for multi-project orchestration
use crate::multi_project::taxonomy_summary_vo::AggregatedResults;
use crate::multi_project::taxonomy_summary_vo::ProjectResult;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::shared_common::taxonomy_layer_vo::Identity;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;

#[async_trait]
pub trait MultiProjectOrchestratorAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn analyze_project(&self, path: &FilePath) -> ProjectResult;
    async fn scan_all_projects(
        &self,
        paths: &FilePathList,
        max_concurrency: Count,
    ) -> AggregatedResults;
    fn load_config(&self, config_path: Option<&FilePath>) -> FilePathList;
    fn find_projects(&self, root: &FilePath, config_name: &Identity) -> FilePathList;
}
