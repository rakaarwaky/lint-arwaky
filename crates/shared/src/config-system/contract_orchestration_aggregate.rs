// PURPOSE: IConfigOrchestrationAggregate — aggregate contract for orchestrating configuration loading across languages

use crate::config_system::contract_reader_port::IConfigReaderPort;
use crate::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IConfigOrchestrationAggregate: Send + Sync {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorPort>;
    fn config_reader(&self) -> Arc<dyn IConfigReaderPort>;

    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;
    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: &str,
    ) -> ConfigResult;
}
