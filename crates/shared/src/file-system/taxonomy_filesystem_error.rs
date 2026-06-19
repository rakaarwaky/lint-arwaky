// PURPOSE: FileSystemError — structured error type for filesystem operation failures
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::mcp_server::taxonomy_action_vo::ActionName;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct FileSystemError {
    pub path: FilePath,
    pub message: ErrorMessage,
    pub operation: ActionName,
    #[serde(default)]
    pub error_code: ErrorCode,
    #[serde(default)]
    pub cause: Cause,
}

impl FileSystemError {
    pub fn new(path: FilePath, message: ErrorMessage, operation: ActionName) -> Self {
        Self {
            path,
            message,
            operation,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = {
            let c: &str = &self.error_code;
            if c.is_empty() {
                String::new()
            } else {
                format!(" [{}]", c)
            }
        };
        write!(
            f,
            "FS Error during {} on {}{}: {}",
            self.operation, self.path, code, self.message
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct PathNotFoundError {
    #[serde(flatten)]
    pub base: FileSystemError,
}

impl PathNotFoundError {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            base: FileSystemError::new(path, message, ActionName::new("read")),
        }
    }
}

impl std::fmt::Display for PathNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Path not found: {} ({})",
            self.base.path, self.base.message
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct AccessDeniedError {
    #[serde(flatten)]
    pub base: FileSystemError,
}

impl AccessDeniedError {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            base: FileSystemError::new(path, message, ActionName::new("access")),
        }
    }
}

impl std::fmt::Display for AccessDeniedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Access denied: {} ({})",
            self.base.path, self.base.message
        )
    }
}
