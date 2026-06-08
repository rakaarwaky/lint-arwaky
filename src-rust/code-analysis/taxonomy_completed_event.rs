use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::shared_common::taxonomy_common_vo::Count;
use /* UNKNOWN: Duration */ crate::shared_common::taxonomy_duration_vo::Duration;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use /* UNKNOWN: Score */ crate::shared_common::taxonomy_common_vo::Score;
use crate::output_report::taxonomy_severity_vo::Severity;
use /* UNKNOWN: Timestamp */ crate::shared_common::taxonomy_common_vo::Timestamp;
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
