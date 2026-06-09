use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_error::Cause;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct PluginError {
    pub message: ErrorMessage,
    pub error_code: ErrorCode,
    pub cause: Cause,
}

impl PluginError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            message,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code_str = self.error_code.to_string();
        let code = if code_str.is_empty() {
            String::new()
        } else {
            format!(" [{}]", code_str)
        };
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
    pub adapter_name: AdapterName,
}

impl RegistrationError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            base: PluginError::new(message),
            adapter_name: AdapterName::default(),
        }
    }
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name_str = self.adapter_name.to_string();
        let target = if name_str.is_empty() {
            String::new()
        } else {
            format!(" for '{}'", name_str)
        };
        write!(f, "Registration Error{}: {}", target, self.base.message)
    }
}
