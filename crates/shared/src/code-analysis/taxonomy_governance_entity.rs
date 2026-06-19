// PURPOSE: ArchitectureGovernanceEntity — domain entity for architecture governance (scores, issues, dates)
use serde::{Deserialize, Serialize};

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_message_vo::ComplianceStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchitectureGovernanceEntity {
    #[serde(default)]
    pub id: Identity,
    #[serde(default)]
    pub results: LintResultList,
    #[serde(default = "default_score")]
    pub score: Score,
    #[serde(default = "default_compliance")]
    pub is_passing: ComplianceStatus,
}

fn default_score() -> Score {
    Score::new(100.0)
}
fn default_compliance() -> ComplianceStatus {
    ComplianceStatus::new(true)
}

impl ArchitectureGovernanceEntity {
    pub fn new() -> Self {
        Self {
            id: Identity::new("default"),
            results: LintResultList::default(),
            score: Score::new(100.0),
            is_passing: ComplianceStatus::new(true),
        }
    }
    pub fn add_result(&mut self, result: LintResult) {
        self.score = self.score.deduct(&result.severity);
        self.results.push(result);
    }
    pub fn update_compliance(&mut self, threshold: &Score) {
        let is_p = self.score.value >= threshold.value;
        let has_critical = self
            .results
            .values
            .iter()
            .any(|r| r.severity == Severity::CRITICAL);
        self.is_passing = ComplianceStatus::new(is_p && !has_critical);
    }
    pub fn results_by_source(&self, source: &AdapterName) -> LintResultList {
        LintResultList {
            values: self
                .results
                .values
                .iter()
                .filter(|r| r.source.as_ref() == Some(source))
                .cloned()
                .collect(),
        }
    }
    pub fn violation_count(&self) -> Count {
        Count::new(
            self.results
                .values
                .iter()
                .filter(|r| r.severity.score_impact() > 0.0)
                .count() as i64,
        )
    }
}

impl Default for ArchitectureGovernanceEntity {
    fn default() -> Self {
        Self::new()
    }
}
