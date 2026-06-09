use crate::shared_common::taxonomy_common_error::Cause;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct WatchServiceError {
    pub path: FilePath,
    pub message: ErrorMessage,
    pub error_code: ErrorCode,
    pub cause: Cause,
}

impl WatchServiceError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            path: FilePath::default(),
            message,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for WatchServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path_str = self.path.to_string();
        let target = if path_str.is_empty() {
            String::new()
        } else {
            format!(" on {}", path_str)
        };
        let code_str = self.error_code.to_string();
        let code = if code_str.is_empty() {
            String::new()
        } else {
            format!(" [{}]", code_str)
        };
        write!(f, "Watch Error{}{}: {}", target, code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct WatchSubscriptionError {
    #[serde(flatten)]
    pub base: WatchServiceError,
}

impl WatchSubscriptionError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            base: WatchServiceError::new(message),
        }
    }
}

impl std::fmt::Display for WatchSubscriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct WatchEventError {
    #[serde(flatten)]
    pub base: WatchServiceError,
}

impl WatchEventError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            base: WatchServiceError::new(message),
        }
    }
}

impl std::fmt::Display for WatchEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}
