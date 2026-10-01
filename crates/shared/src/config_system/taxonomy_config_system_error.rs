// PURPOSE: ConfigError, ConfigErrorKind — structured error types for configuration loading failures
use crate::taxonomy_config_system_vo::ActualValue;
use crate::taxonomy_config_system_vo::ConfigKey;
use crate::taxonomy_config_system_vo::ExpectedValue;
use serde::{Deserialize, Serialize};
use shared_common::taxonomy_common_error::ErrorMessage;
use shared_common::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct ConfigError {
    pub key: ConfigKey,
    pub message: ErrorMessage,
    pub expected: ExpectedValue,
    pub actual: ActualValue,
    pub config_file: FilePath,
    #[serde(default)]
    pub error_id: shared_common::taxonomy_error_vo::ErrorId,
}

impl ConfigError {
    pub fn new(key: ConfigKey, message: ErrorMessage) -> Self {
        Self {
            key,
            message,
            expected: ExpectedValue::default(),
            actual: ActualValue::default(),
            config_file: FilePath::default(),
            error_id: shared_common::taxonomy_error_vo::ErrorId::raw(1001),
        }
    }

    /// Stable numeric id for machine branching.
    pub fn error_id(&self) -> u16 {
        self.error_id.value()
    }

    /// Human-readable description derived from the error id and fields.
    pub fn message(&self) -> String {
        format!(
            "{}: config key '{}': expected '{}' got '{}'",
            self.error_id(),
            self.key,
            self.expected,
            self.actual
        )
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_str = self.config_file.to_string();
        let file_info = if file_str.is_empty() {
            String::new()
        } else {
            format!(" in {}", file_str)
        };
        write!(
            f,
            "Config error on '{}'{}: {}",
            self.key, file_info, self.message
        )
    }
}
