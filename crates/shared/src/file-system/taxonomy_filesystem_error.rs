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
        let code = if self.error_code.code().is_empty() {
            String::new()
        } else {
            format!(" [{}]", self.error_code.code())
        };
        write!(
            f,
            "FS Error during {} on {}{}: {}",
            self.operation, self.path, code, self.message
        )
    }
}

/// Wrap a `FileSystemError` in a newtype variant and forward its `Display`.
/// Use `[$name, $op, $msg_prefix]` form when the newtype should override the
/// operation label (e.g. `read`/`access`) and produce a custom prefix when
/// displayed (e.g. `"Path not found: "`/`"Access denied: "`).
macro_rules! fs_error_newtype {
    ($name:ident, $op:expr, $msg_prefix:literal) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
        pub struct $name {
            #[serde(flatten)]
            pub base: FileSystemError,
        }

        impl $name {
            pub fn new(path: FilePath, message: ErrorMessage) -> Self {
                Self {
                    base: FileSystemError::new(path, message, ActionName::new($op)),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}{} ({})",
                    $msg_prefix, self.base.path, self.base.message
                )
            }
        }
    };
}

fs_error_newtype!(PathNotFoundError, "read", "Path not found: ");
fs_error_newtype!(AccessDeniedError, "access", "Access denied: ");
