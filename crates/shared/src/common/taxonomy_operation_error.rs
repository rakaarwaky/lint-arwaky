// PURPOSE: LinterOperationError — structured error type for linter operation failures (scan, fix, report)
use crate::common::taxonomy_adapter_error::AdapterError;
use crate::common::taxonomy_adapter_error::ScanError;
/// linter_operation_error — Unified error type for linter adapter operations.
/* UNKNOWN: ErrorMessage */
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::LineNumber;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum LinterOperationError {
    #[error("Scan error: {0}")]
    Scan(ScanError),

    #[error("Adapter error: {0}")]
    Adapter(AdapterError),
}

impl LinterOperationError {
    /// Stable numeric id for machine branching.
    pub fn error_id(&self) -> u16 {
        match self {
            Self::Scan(_) => 4,
            Self::Adapter(_) => 5,
        }
    }

    pub fn message(&self) -> ErrorMessage {
        let _ = &LineNumber::default();
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
