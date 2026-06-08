/// linter_operation_error — Unified error type for linter adapter operations.
/* UNKNOWN: ErrorMessage */ use crate::shared_common::taxonomy_common_error::ErrorMessage;
use /* UNKNOWN: lint_adapter_error::AdapterError */ crate::shared_common::taxonomy_adapter_error::AdapterError;
use /* UNKNOWN: lint_adapter_error::ScanError */ crate::shared_common::taxonomy_adapter_error::ScanError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum LinterOperationError {
    #[error("Scan error: {0}")]
    Scan(ScanError),

    #[error("Adapter error: {0}")]
    Adapter(AdapterError),
}

impl LinterOperationError {
    pub fn message(&self) -> ErrorMessage {
        ErrorMessage::new(self.to_string())
    }
}

impl From<ScanError> for LinterOperationError {
    fn from(e: ScanError) -> Self {
        LinterOperationError::Scan(e)
    }
}

impl From<AdapterError> for LinterOperationError {
    fn from(e: AdapterError) -> Self {
        LinterOperationError::Adapter(e)
    }
}
