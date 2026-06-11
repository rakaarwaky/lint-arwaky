// PURPOSE: ConfigYamlReader — reads and parses lint-arwaky YAML config files from disk
use crate::config_system::contract_reader_port::IConfigReaderPort;
use crate::config_system::taxonomy_source_vo::ConfigSource;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ConfigYamlReader {
    _path_norm: Arc<dyn IPathNormalizationPort>,
}

impl ConfigYamlReader {
    pub fn new(path_norm: Arc<dyn IPathNormalizationPort>) -> Self {
        Self {
            _path_norm: path_norm,
        }
    }

    fn config_filename(language: &str) -> String {
        format!("lint_arwaky.config.{}.yaml", language)
    }
}

#[async_trait]
impl IConfigReaderPort for ConfigYamlReader {
    async fn read_config(&self, project_root: &FilePath, language: &str) -> Option<ConfigSource> {
        let filename = Self::config_filename(language);
        let mut path = std::path::PathBuf::from(&project_root.value).join(&filename);

        if !path.exists() {
            // Try parent directory
            if let Some(parent) = std::path::Path::new(&project_root.value).parent() {
                path = parent.join(&filename);
            }
        }

        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(content) => Some(ConfigSource::new(
                    language,
                    path.to_string_lossy().to_string(),
                    content,
                )),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    async fn list_config_files(&self, project_root: &FilePath) -> Vec<(String, String)> {
        let mut found = Vec::new();
        for lang in &["rust", "python", "javascript"] {
            if let Some(config) = self.read_config(project_root, lang).await {
                found.push((lang.to_string(), config.path));
            }
        }
        found
    }
}
