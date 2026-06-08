use /* UNKNOWN: AggregatedResults */ crate::multi_project::taxonomy_summary_vo::AggregatedResults;
use crate::shared_common::taxonomy_common_vo::Count;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use /* UNKNOWN: FilePathList */ crate::source_parsing::taxonomy_paths_vo::FilePathList;
use crate::shared_common::taxonomy_layer_vo::Identity;
use /* UNKNOWN: ProjectResult */ crate::multi_project::taxonomy_summary_vo::ProjectResult;
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
    fn load_config(config_path: Option<&FilePath>) -> FilePathList;
    fn find_projects(root: &FilePath, config_name: &Identity) -> FilePathList;
}
