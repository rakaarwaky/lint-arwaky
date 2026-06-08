use crate::taxonomy::{BooleanVO, FilePath, Timestamp};
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
