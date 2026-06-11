// PURPOSE: Re-exports ContentString, SourceContentVO from common + ConfigResult, ConfigSource for config-system
pub use crate::common::taxonomy_source_vo::ContentString;
pub use crate::common::taxonomy_source_vo::SourceContentVO;

use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::source_parsing::taxonomy_path_vo::FilePath;

/// Result type for config loading operations.
pub type ConfigResult = Result<ArchitectureConfig, String>;

/// Represents a configuration source with its language and path.
pub struct ConfigSource {
    pub config: ArchitectureConfig,
    pub language: String,
    pub path: FilePath,
}

impl ConfigSource {
    pub fn new(config: ArchitectureConfig, language: impl Into<String>, path: FilePath) -> Self {
        Self {
            config,
            language: language.into(),
            path,
        }
    }
}