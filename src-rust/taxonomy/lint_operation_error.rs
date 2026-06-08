/// linter_operation_error — Unified error type for linter adapter operations.
use crate::taxonomy::BooleanVO;
use crate::taxonomy::{lint_adapter_error::AdapterError, lint_adapter_error::ScanError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum LinterOperationError {
    #[error("Scan error: {0}")]
    Scan(ScanError),

    #[error("Adapter error: {0}")]
    Adapter(AdapterError),
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
