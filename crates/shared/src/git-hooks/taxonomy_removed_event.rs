// PURPOSE: HookRemoved — domain event published when a git hook is removed
use crate::common::taxonomy_common_vo::Timestamp;
use crate::common::taxonomy_path_vo::FilePath;
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
