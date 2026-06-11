// PURPOSE: HookInstalled — domain event published when a git hook is installed
use crate::common::taxonomy_common_vo::Timestamp;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HookInstalled {
    pub path: FilePath,
    pub executable: FilePath,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl HookInstalled {
    pub fn new(path: FilePath, executable: FilePath) -> Self {
        Self {
            path,
            executable,
            timestamp: Timestamp::default(),
        }
    }
}
