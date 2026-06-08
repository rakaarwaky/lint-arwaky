use crate::taxonomy::{BooleanVO, ComplianceStatus, Count, Duration, FilePath, Score, Severity, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanCompleted {
    pub path: FilePath,
    pub score: Score,
    pub worst_severity: Severity,
    pub violation_count: Count,
    pub duration_ms: Duration,
    #[serde(default)]
    pub is_passing: ComplianceStatus,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl ScanCompleted {
    pub fn new(
        path: FilePath,
        score: Score,
        worst_severity: Severity,
        violation_count: Count,
        duration_ms: Duration,
    ) -> Self {
        Self {
            path,
            score,
            worst_severity,
            violation_count,
            duration_ms,
            is_passing: ComplianceStatus::new(true),
            timestamp: Timestamp::default(),
        }
    }
}
