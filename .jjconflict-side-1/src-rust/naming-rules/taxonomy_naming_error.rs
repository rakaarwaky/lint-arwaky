use crate::shared_common::taxonomy_common_error::Cause;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct NamingError {
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: ErrorCode,
    #[serde(default)]
    pub cause: Cause,
}

impl NamingError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            message,
            error_code: ErrorCode::default(),
            cause: Cause::default(),
        }
    }
}

impl std::fmt::Display for NamingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code: &str = &self.error_code;
        if code.is_empty() {
            write!(f, "Naming Error: {}", self.message)
        } else {
            write!(f, "Naming Error [{}]: {}", code, self.message)
        }
    }
}
