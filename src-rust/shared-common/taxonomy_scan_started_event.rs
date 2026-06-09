use crate::shared_common::taxonomy_common_vo::Timestamp;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanStarted {
    pub path: FilePath,
    pub adapters: Vec<AdapterName>,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl ScanStarted {
    pub fn new(path: FilePath, adapters: Vec<AdapterName>) -> Self {
        Self {
            path,
            adapters,
            timestamp: Timestamp::default(),
        }
    }
}
