// PURPOSE: ConfigOrchestrator — capabilities implementation of IConfigOrchestrationProtocol for project config loading and language discovery
use crate::config_system::contract_orchestration_protocol::IConfigOrchestrationProtocol;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::config_system::agent_config_loading_orchestrator::ConfigLoadingOrchestrator;
use crate::config_system::infrastructure_detector_provider::LanguageDetectorProvider;
use crate::config_system::infrastructure_yaml_reader::ConfigYamlReader;
use crate::source_parsing::infrastructure_path_provider::PathNormalizationProvider;
use std::sync::Arc;
use async_trait::async_trait;

pub struct ConfigOrchestrator {
    inner: ConfigLoadingOrchestrator,
}

impl Default for ConfigOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigOrchestrator {
    pub fn new() -> Self {
        let detector = Arc::new(LanguageDetectorProvider::new());
        let path_norm = Arc::new(PathNormalizationProvider {});
        let reader = Arc::new(ConfigYamlReader::new(path_norm));
        let inner = ConfigLoadingOrchestrator::new(detector, reader);
        Self { inner }
    }
}

#[async_trait]
impl IConfigOrchestrationProtocol for ConfigOrchestrator {
    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult {
        self.inner.load_project_config(project_root).await
    }

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: &str,
    ) -> ConfigResult {
        self.inner.load_config_for_language(project_root, language).await
    }
}
