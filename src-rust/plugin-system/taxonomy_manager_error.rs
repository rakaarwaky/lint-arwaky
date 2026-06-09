use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_error::Cause;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct PluginError {
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub cause: Option<Cause>,
}

impl PluginError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            message,
            error_code: None,
            cause: None,
        }
    }
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self
            .error_code
            .as_ref()
            .map(|c| format!(" [{}]", c))
            .unwrap_or_default();
        write!(f, "Plugin Error{}: {}", code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct DiscoveryError {
    #[serde(flatten)]
    pub base: PluginError,
}

impl DiscoveryError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            base: PluginError::new(message),
        }
    }
}

impl std::fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct RegistrationError {
    #[serde(flatten)]
    pub base: PluginError,
    #[serde(default)]
    pub adapter_name: Option<AdapterName>,
}

impl RegistrationError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            base: PluginError::new(message),
            adapter_name: None,
        }
    }
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let target = self
            .adapter_name
            .as_ref()
            .map(|a| format!(" for '{}'", a))
            .unwrap_or_default();
        write!(f, "Registration Error{}: {}", target, self.base.message)
    }
}
