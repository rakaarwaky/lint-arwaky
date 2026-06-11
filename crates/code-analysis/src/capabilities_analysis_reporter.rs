// PURPOSE: AnalysisReporter — IAnalysisProtocol implementation for tracking analysis history and trend data

use async_trait::async_trait;

use crate::IAnalysisProtocol;
use crate::ArchitectureGovernanceEntity;
use shared::taxonomy_result_vo::{LintResult, LintResultList};
use shared::taxonomy_score_vo::compute_score;
use shared::taxonomy_severity_vo::Severity;
use shared::AdapterName;
use shared::{ColumnNumber, LineNumber, Score};
use shared::ErrorCode;
use shared::{ComplianceStatus, LintMessage};
use shared::FilePath;

pub struct AnalysisReporter {
    history_path: String,
}

impl Default for AnalysisReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisReporter {
    pub fn new() -> Self {
        Self {
            history_path: ".lint-arwaky-trends.json".to_string(),
        }
    }

    pub fn with_history_path(history_path: &str) -> Self {
        Self {
            history_path: history_path.to_string(),
        }
    }
}

#[async_trait]
impl IAnalysisProtocol for AnalysisReporter {
    async fn run(&self, _path: &FilePath) -> ArchitectureGovernanceEntity {
        ArchitectureGovernanceEntity::default()
    }

    async fn get_complexity(&self, path: &FilePath) -> ArchitectureGovernanceEntity {
        let report = self.run(path).await;
        let source_adapter = AdapterName::raw("radon");
        self.report_from_list(report.results_by_source(&source_adapter))
    }

    async fn get_duplicates(&self, path: &FilePath) -> ArchitectureGovernanceEntity {
        let report = self.run(path).await;
        let source_adapter = AdapterName::raw("duplicates");
        self.report_from_list(report.results_by_source(&source_adapter))
    }

    async fn get_trends(&self, path: &FilePath) -> ArchitectureGovernanceEntity {
        let report = self.run(path).await;
        let current_score = compute_score(&report.results.values.to_vec());

        let history_path = std::path::Path::new(&path.value).join(&self.history_path);
        if history_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&history_path) {
                let history: Vec<serde_json::Value> = content
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
                    .collect();

                if let Some(prev) = history.last() {
                    if let Some(prev_score) = prev.get("score").and_then(|s| s.as_f64()) {
                        let delta = current_score - prev_score;
                        let trend = if delta > 1.0 {
                            "improving"
                        } else if delta < -1.0 {
                            "declining"
                        } else {
                            "stable"
                        };

                        let trend_result = LintResult {
                            file: path.clone(),
                            line: LineNumber::new(0),
                            column: ColumnNumber::new(0),
                            code: ErrorCode::raw("TREND001"),
                            message: LintMessage::new(format!(
                                "Quality score: {:.1}, delta: {:.1}, trend: {}",
                                current_score, delta, trend
                            )),
                            source: Some(AdapterName::raw("trends")),
                            severity: Severity::INFO,
                            enclosing_scope: Default::default(),
                            related_locations: Default::default(),
                        };
                        return ArchitectureGovernanceEntity {
                            id: Default::default(),
                            results: LintResultList::new(vec![trend_result]),
                            score: Score::new(current_score),
                            is_passing: ComplianceStatus::new(true),
                        };
                    }
                }
            }
        }

        let no_history = LintResult {
            file: path.clone(),
            line: LineNumber::new(0),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw("TREND002"),
            message: LintMessage::new("No trend history yet — first run".to_string()),
            source: Some(AdapterName::raw("trends")),
            severity: Severity::INFO,
            enclosing_scope: Default::default(),
            related_locations: Default::default(),
        };

        let mut entity = ArchitectureGovernanceEntity::new();
        entity.add_result(no_history);
        entity
    }

    async fn get_dependencies(&self, path: &FilePath) -> ArchitectureGovernanceEntity {
        let report = self.run(path).await;
        let source_adapter = AdapterName::raw("pip-audit");
        self.report_from_list(report.results_by_source(&source_adapter))
    }
}

impl AnalysisReporter {
    fn report_from_list(&self, list: LintResultList) -> ArchitectureGovernanceEntity {
        let mut report = ArchitectureGovernanceEntity::new();
        for result in list.values {
            report.add_result(result);
        }
        report
    }
}
