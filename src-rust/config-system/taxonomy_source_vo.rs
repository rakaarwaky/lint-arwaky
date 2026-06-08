use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigSource {
    pub language: String,
    pub path: String,
    pub raw_content: String,
}

impl ConfigSource {
    pub fn new(
        language: impl Into<String>,
        path: impl Into<String>,
        raw_content: impl Into<String>,
    ) -> Self {
        Self {
            language: language.into(),
            path: path.into(),
            raw_content: raw_content.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigResult {
    pub config: ArchitectureConfig,
    pub source: ConfigSource,
    pub warnings: Vec<String>,
}

impl ConfigResult {
    pub fn new(config: ArchitectureConfig, source: ConfigSource, warnings: Vec<String>) -> Self {
        Self {
            config,
            source,
            warnings,
        }
    }
}
