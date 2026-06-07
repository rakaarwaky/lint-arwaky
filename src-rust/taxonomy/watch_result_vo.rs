use crate::taxonomy::{ArchitectureGovernanceEntity, ComplianceStatus, FilePath, Score};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WatchResult {
    pub file: FilePath,
    pub report: ArchitectureGovernanceEntity,
}

impl WatchResult {
    pub fn new(file: FilePath, report: ArchitectureGovernanceEntity) -> Self {
        Self { file, report }
    }
    pub fn score(&self) -> &Score {
        &self.report.score
    }
    pub fn is_passing(&self) -> &ComplianceStatus {
        &self.report.is_passing
    }
}
