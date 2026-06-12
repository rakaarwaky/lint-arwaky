// PURPOSE: MultiProjectOrchestratorAggregate — contract trait for multi-project orchestration
use async_trait::async_trait;
use crate::multi_project::taxonomy_summary_vo::{AggregatedResults, ProjectResult};
use crate::taxonomy_common_vo::Count;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::taxonomy_layer_vo::Identity;

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
