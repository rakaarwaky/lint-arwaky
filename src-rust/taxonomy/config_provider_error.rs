use crate::taxonomy::{ActualValue, BooleanVO, ConfigKey, ErrorMessage, ExpectedValue, FilePath};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct ConfigError {
    pub key: ConfigKey,
    pub message: ErrorMessage,
    #[serde(default)]
    pub expected: Option<ExpectedValue>,
    #[serde(default)]
    pub actual: Option<ActualValue>,
    #[serde(default)]
    pub config_file: Option<FilePath>,
}

impl ConfigError {
    pub fn new(key: ConfigKey, message: ErrorMessage) -> Self {
        Self {
            key,
            message,
            expected: None,
            actual: None,
            config_file: None,
        }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_info = self
            .config_file
            .as_ref()
            .map(|p| format!(" in {}", p))
            .unwrap_or_default();
        write!(
            f,
            "Config error on '{}'{}: {}",
            self.key, file_info, self.message
        )
    }
}
