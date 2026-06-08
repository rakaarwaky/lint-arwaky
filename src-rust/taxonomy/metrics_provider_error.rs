use crate::taxonomy::{BooleanVO, Cause, ErrorCode, ErrorMessage, FilePath};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct MetricsError {
    #[serde(default)]
    pub path: Option<FilePath>,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub cause: Option<Cause>,
}

impl MetricsError {
    pub fn new(message: ErrorMessage) -> Self {
        Self {
            path: None,
            message,
            error_code: None,
            cause: None,
        }
    }
}

impl std::fmt::Display for MetricsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let target = self
            .path
            .as_ref()
            .map(|p| format!(" for {}", p))
            .unwrap_or_default();
        let code = self
            .error_code
            .as_ref()
            .map(|c| format!(" [{}]", c))
            .unwrap_or_default();
        write!(f, "Metrics Error{}{}: {}", target, code, self.message)
    }
}
