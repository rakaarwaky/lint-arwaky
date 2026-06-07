use crate::taxonomy::{AggregatedResults, Count, FilePath, FilePathList, Identity, ProjectResult};
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
