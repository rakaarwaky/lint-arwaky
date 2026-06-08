use crate::source_parsing::taxonomy_path_vo::FilePath;
use /* UNKNOWN: Timestamp */ crate::shared_common::taxonomy_common_vo::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HookRemoved {
    pub path: FilePath,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl HookRemoved {
    pub fn new(path: FilePath) -> Self {
        Self {
            path,
            timestamp: Timestamp::default(),
        }
    }
}
