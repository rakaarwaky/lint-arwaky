use crate::taxonomy::{AdapterName, FilePath, Timestamp};
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
