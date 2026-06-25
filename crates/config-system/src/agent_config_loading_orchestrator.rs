// PURPOSE: ConfigLoadingOrchestrator — orchestrates config discovery, loading, parsing across languages
use async_trait::async_trait;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
use shared::config_system::contract_reader_port::IConfigReaderPort;
use shared::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
use shared::config_system::taxonomy_config_vo::default_config_for_language;
use shared::config_system::taxonomy_config_vo::parse_config_yaml;
use shared::config_system::taxonomy_source_vo::ConfigResult;
use shared::config_system::taxonomy_source_vo::ConfigSource;
use std::sync::Arc;

pub struct ConfigLoadingOrchestrator {
    workspace_detector: Arc<dyn IWorkspaceDetectorPort>,
    config_reader: Arc<dyn IConfigReaderPort>,
}

impl ConfigLoadingOrchestrator {
    pub fn new(
        workspace_detector: Arc<dyn IWorkspaceDetectorPort>,
        config_reader: Arc<dyn IConfigReaderPort>,
    ) -> Self {
        Self {
            workspace_detector,
            config_reader,
        }
    }
}

#[async_trait]
impl IConfigOrchestrationAggregate for ConfigLoadingOrchestrator {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorPort> {
        self.workspace_detector.clone()
    }

    fn config_reader(&self) -> Arc<dyn IConfigReaderPort> {
        self.config_reader.clone()
    }

    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult {
        let ws_type = self.workspace_detector.detect(project_root);
        let language = ws_type.as_str().to_string();
        self.load_config_for_language(project_root, &language).await
    }

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: &str,
    ) -> ConfigResult {
        match self.config_reader.read_config(project_root, language).await {
            Some(source) => {
                let parsed = parse_config_yaml(&source.raw_content);
                if !parsed.layers.is_empty() {
                    ConfigResult::new(parsed, source, Vec::new())
                } else {
                    let warnings = vec![
                        "Config file had no architecture layers, using built-in defaults"
                            .to_string(),
                    ];
                    let config = default_config_for_language(language);
                    ConfigResult::new(config, source, warnings)
                }
            }
            None => {
                let warnings = vec!["No config file found, using built-in defaults".to_string()];
                let config = default_config_for_language(language);
                let source = ConfigSource::new(language, "embedded", "");
                ConfigResult::new(config, source, warnings)
            }
        }
    }
}
