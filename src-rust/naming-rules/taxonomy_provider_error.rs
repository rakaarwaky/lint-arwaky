use crate::shared_common::taxonomy_common_error::Cause;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use /* UNKNOWN: ErrorMessage */ crate::shared_common::taxonomy_common_error::ErrorMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct NamingError {
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub cause: Option<Cause>,
}

impl NamingError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            message,
            error_code: None,
            cause: None,
        }
    }
}

impl std::fmt::Display for NamingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error_code {
            Some(code) => write!(f, "Naming Error [{}]: {}", code, self.message),
            None => write!(f, "Naming Error: {}", self.message),
        }
    }
}
