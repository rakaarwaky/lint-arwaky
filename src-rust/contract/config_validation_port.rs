// config_validation_port — Interface for configuration validation and orchestration.
use crate::taxonomy::{AppConfig, ConfigError, FilePath};
use async_trait::async_trait;

#[async_trait]
pub trait IConfigValidationPort: Send + Sync {
    /// Load or reload configuration. Returns AppConfig.
    async fn load_config(
        &self,
        env_path: Option<FilePath>,
        yaml_path: Option<FilePath>,
    ) -> Result<AppConfig, ConfigError>;

    /// Get the current configuration.
    async fn get_config(&self) -> Result<AppConfig, ConfigError>;

    /// Reset the configuration.
    async fn reset_config(&self);
}
