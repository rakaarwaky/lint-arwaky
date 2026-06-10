// PURPOSE: ConfigLoadingOrchestrator — orchestrates config discovery, loading, parsing across languages
use crate::config_system::contract_detector_port::ILanguageDetectorPort;
use crate::config_system::contract_orchestration_protocol::IConfigOrchestrationProtocol;
use crate::config_system::contract_reader_port::IConfigReaderPort;
use crate::config_system::taxonomy_config_vo::default_config_for_language;
use crate::config_system::taxonomy_config_vo::parse_config_yaml;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use crate::config_system::taxonomy_source_vo::ConfigSource;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;
use std::sync::Arc;

pub type ServiceContainerAggregateRef = Box<dyn ServiceContainerAggregate>;

pub struct ConfigLoadingOrchestrator {
    language_detector: Arc<dyn ILanguageDetectorPort>,
    config_reader: Arc<dyn IConfigReaderPort>,
}

impl ConfigLoadingOrchestrator {
    pub fn new(
        language_detector: Arc<dyn ILanguageDetectorPort>,
        config_reader: Arc<dyn IConfigReaderPort>,
    ) -> Self {
        Self {
            language_detector,
            config_reader,
        }
    }
}

#[async_trait]
impl IConfigOrchestrationProtocol for ConfigLoadingOrchestrator {
    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult {
        let lang_source = self.language_detector.detect_language(project_root).await;
        self.load_config_for_language(project_root, &lang_source.language)
            .await
    }

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: &str,
    ) -> ConfigResult {
        match self.config_reader.read_config(project_root, language).await {
            Some(source) => {
                // Read file content and parse through the proper YAML pipeline
                // (with suffix array → policy/allowed/forbidden transformation)
                let config = match std::fs::read_to_string(&source.path) {
                    Ok(content) => {
                        let parsed = parse_config_yaml(&content);
                        if !parsed.layers.is_empty() {
                            parsed
                        } else {
                            default_config_for_language(language)
                        }
                    }
                    Err(_) => default_config_for_language(language),
                };
                if config.layers.is_empty() {
                    let warnings = vec![
                        "Config file had no architecture layers, using built-in defaults"
                            .to_string(),
                    ];
                    let source = ConfigSource::new(language, "embedded", "");
                    ConfigResult::new(config, source, warnings)
                } else {
                    ConfigResult::new(config, source, Vec::new())
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
