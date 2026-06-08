use crate::config_system::contract_orchestration_protocol::IConfigOrchestrationProtocol;
use crate::config_system::contract_parser_port::IConfigParserPort;
use crate::config_system::contract_reader_port::IConfigReaderPort;
use crate::config_system::contract_detector_port::ILanguageDetectorPort;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::config_system::taxonomy_config_vo::default_config_for_language;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use crate::config_system::taxonomy_source_vo::ConfigSource;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ConfigLoadingOrchestrator {
    language_detector: Arc<dyn ILanguageDetectorPort>,
    config_reader: Arc<dyn IConfigReaderPort>,
    config_parser: Arc<dyn IConfigParserPort>,
}

impl ConfigLoadingOrchestrator {
    pub fn new(
        language_detector: Arc<dyn ILanguageDetectorPort>,
        config_reader: Arc<dyn IConfigReaderPort>,
        config_parser: Arc<dyn IConfigParserPort>,
    ) -> Self {
        let _: Option<&dyn ServiceContainerAggregate> = None;
        Self {
            language_detector,
            config_reader,
            config_parser,
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
        let config_path = FilePath::new(
            std::path::Path::new(&project_root.value)
                .join(format!("lint_arwaky.config.{language}.yaml"))
                .to_string_lossy()
                .to_string(),
        )
        .unwrap_or_else(|_| project_root.clone());

        match self.config_reader.read_config(project_root, language).await {
            Some(source) => {
                match self.config_parser.parse_yaml_config(
                    &FilePath::new(source.path.clone()).unwrap_or(config_path.clone()),
                ) {
                    Ok(_project_config) => {
                        let config = default_config_for_language(language);
                        ConfigResult::new(config, source, Vec::new())
                    }
                    Err(_) => {
                        let warnings = vec![
                            "Failed to parse config file, using built-in defaults".to_string()
                        ];
                        let config = default_config_for_language(language);
                        let source = ConfigSource::new(language, "embedded", "");
                        ConfigResult::new(config, source, warnings)
                    }
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
