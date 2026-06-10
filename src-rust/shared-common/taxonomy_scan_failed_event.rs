// PURPOSE: ScanFailed — domain event published when a project scan fails
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::Timestamp;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::source_parsing::taxonomy_path_vo::FilePath;
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
