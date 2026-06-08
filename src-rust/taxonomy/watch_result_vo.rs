use crate::taxonomy::{ComplianceStatus, FilePath, Score};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WatchResult {
    pub file: FilePath,
    pub score: Score,
    pub is_passing: ComplianceStatus,
}

impl WatchResult {
    pub fn new(file: FilePath, score: Score) -> Self {
        Self {
            file,
            score,
            is_passing: ComplianceStatus::new(true),
        }
    }
    pub fn score(&self) -> &Score {
        &self.score
    }
    pub fn is_passing(&self) -> &ComplianceStatus {
        &self.is_passing
    }
}
