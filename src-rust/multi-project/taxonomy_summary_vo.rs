use serde::{Deserialize, Serialize};

use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::shared_common::taxonomy_common_vo::Count;
use /* UNKNOWN: ErrorMessage */ crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use /* UNKNOWN: PatternList */ crate::shared_common::taxonomy_common_vo::PatternList;
use /* UNKNOWN: Score */ crate::shared_common::taxonomy_common_vo::Score;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AggregatedResults {
    pub projects: Vec<ProjectResult>,
    pub total_projects: Count,
    pub passing_projects: Count,
    pub failing_projects: Count,
    pub average_score: Score,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectResult {
    pub path: FilePath,
    pub score: Score,
    pub is_passing: ComplianceStatus,
    pub issues: Vec<std::collections::HashMap<String, serde_json::Value>>,
    pub adapters: PatternList,
    pub error: ErrorMessage,
}

impl AggregatedResults {
    pub fn new(
        projects: Vec<ProjectResult>,
        total_projects: Count,
        passing_projects: Count,
        failing_projects: Count,
        average_score: Score,
    ) -> Self {
        Self {
            projects,
            total_projects,
            passing_projects,
            failing_projects,
            average_score,
        }
    }
}

impl ProjectResult {
    pub fn new(
        path: FilePath,
        score: Score,
        is_passing: ComplianceStatus,
        issues: Vec<std::collections::HashMap<String, serde_json::Value>>,
        adapters: PatternList,
        error: ErrorMessage,
    ) -> Self {
        Self {
            path,
            score,
            is_passing,
            issues,
            adapters,
            error,
        }
    }
}
