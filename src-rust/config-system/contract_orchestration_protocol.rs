// PURPOSE: IConfigOrchestrationProtocol — protocol for orchestrating configuration loading across languages

use crate::config_system::taxonomy_source_vo::ConfigResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IConfigOrchestrationProtocol: Send + Sync {
    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;
    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: &str,
    ) -> ConfigResult;
}
