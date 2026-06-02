use serde::{Serialize, Deserialize};
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WatchResult {
    pub file: FilePath,
    pub report: GovernanceReport,
}

impl WatchResult {
    pub fn new(file: FilePath, report: GovernanceReport) -> Self {
        Self { file, report }
    }
    pub fn score(&self) -> &Score {
        &self.report.score
    }
    pub fn is_passing(&self) -> &ComplianceStatus {
        &self.report.is_passing
    }
}
