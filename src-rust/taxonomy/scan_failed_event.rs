use crate::taxonomy::{AdapterName, BooleanVO, ErrorCode, ErrorMessage, FilePath, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanFailed {
    pub path: FilePath,
    pub adapter: AdapterName,
    pub error_message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl ScanFailed {
    pub fn new(path: FilePath, adapter: AdapterName, error_message: ErrorMessage) -> Self {
        Self {
            path,
            adapter,
            error_message,
            error_code: None,
            timestamp: Timestamp::default(),
        }
    }
}
