use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::contract_reader_protocol::IConfigReaderProtocol;
use crate::config_system::contract_workspace_detector_protocol::IWorkspaceDetectorProtocol;
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IConfigOrchestratorAggregate: Send + Sync {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorProtocol>;
    fn config_reader(&self) -> Arc<dyn IConfigReaderProtocol>;

    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;
    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: &str,
    ) -> ConfigResult;

    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo>;
}
