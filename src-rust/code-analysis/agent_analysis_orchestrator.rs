// analysis_execution_orchestrator — Implementation of the analysis orchestration domain contract.
use crate::code_analysis::contract_analysis_aggregate::AnalysisOrchestratorAggregate;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::layer_rules::taxonomy_governance_entity::ArchitectureGovernanceEntity;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use /* UNKNOWN: LintResultList */ crate::output_report::taxonomy_result_vo::LintResultList;
use async_trait::async_trait;

pub struct AnalysisOrchestrator {}

struct DummyContainer {}
impl ServiceContainerAggregate for DummyContainer {
}

#[async_trait]
impl AnalysisOrchestratorAggregate for AnalysisOrchestrator {
    fn container(&self) -> &dyn ServiceContainerAggregate {
        static DUMMY: DummyContainer = DummyContainer {};
        &DUMMY
    }

    async fn run(&self, _path: &FilePath) -> ArchitectureGovernanceEntity {
        ArchitectureGovernanceEntity::default()
    }

    async fn get_complexity(&self, path: &FilePath) -> ArchitectureGovernanceEntity {
        let report = self.run(path).await;
        let source_adapter = crate::shared_common::taxonomy_name_vo::AdapterName::raw("radon");
        self.report_from_list(report.results_by_source(&source_adapter))
    }

    async fn get_duplicates(&self, path: &FilePath) -> ArchitectureGovernanceEntity {
        let report = self.run(path).await;
        let source_adapter = crate::shared_common::taxonomy_name_vo::AdapterName::raw("duplicates");
        self.report_from_list(report.results_by_source(&source_adapter))
    }

    async fn get_trends(&self, path: &FilePath) -> ArchitectureGovernanceEntity {
        let report = self.run(path).await;
        let current_score = crate::output_report::taxonomy_score_constant::compute_score(
            &report
                .results
                .values
                .iter()
                .map(|r| r.clone())
                .collect::<Vec<_>>(),
        );

        // Read history file
        let history_path = std::path::Path::new(&path.value).join(".lint-arwaky-trends.json");
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

                        let trend_result = crate::output_report::taxonomy_result_vo::LintResult {
                            file: path.clone(),
                            line: crate::shared_common::taxonomy_common_vo::LineNumber::new(0),
                            column: crate::shared_common::taxonomy_common_vo::ColumnNumber::new(0),
                            code: crate::shared_common::taxonomy_error_vo::ErrorCode::raw("TREND001"),
                            message: crate::shared_common::taxonomy_message_vo::LintMessage::new(format!(
                                "Quality score: {:.1}, delta: {:.1}, trend: {}",
                                current_score, delta, trend
                            )),
                            source: Some(crate::shared_common::taxonomy_name_vo::AdapterName::raw("trends")),
                            severity: crate::output_report::taxonomy_severity_vo::Severity::INFO,
                            enclosing_scope: Default::default(),
                            related_locations: Default::default(),
                        };
                        return ArchitectureGovernanceEntity {
                            id: Default::default(),
                            results: crate::output_report::taxonomy_result_vo::LintResultList::new(vec![trend_result]),
                            score: crate::shared_common::taxonomy_common_vo::Score::new(current_score),
                            is_passing: crate::shared_common::taxonomy_message_vo::ComplianceStatus::new(true),
                        };
                    }
                }
            }
        }

        let no_history = crate::output_report::taxonomy_result_vo::LintResult {
            file: path.clone(),
            line: crate::shared_common::taxonomy_common_vo::LineNumber::new(0),
            column: crate::shared_common::taxonomy_common_vo::ColumnNumber::new(0),
            code: crate::shared_common::taxonomy_error_vo::ErrorCode::raw("TREND002"),
            message: crate::shared_common::taxonomy_message_vo::LintMessage::new(
                "No trend history yet — first run".to_string(),
            ),
            source: Some(crate::shared_common::taxonomy_name_vo::AdapterName::raw("trends")),
            severity: crate::output_report::taxonomy_severity_vo::Severity::INFO,
            enclosing_scope: Default::default(),
            related_locations: Default::default(),
        };

        let mut entity = ArchitectureGovernanceEntity::new();
        entity.add_result(no_history);
        entity
    }

    async fn get_dependencies(&self, path: &FilePath) -> ArchitectureGovernanceEntity {
        let report = self.run(path).await;
        let source_adapter = crate::shared_common::taxonomy_name_vo::AdapterName::raw("pip-audit");
        self.report_from_list(report.results_by_source(&source_adapter))
    }
}

impl AnalysisOrchestrator {
    fn report_from_list(&self, list: LintResultList) -> ArchitectureGovernanceEntity {
        let mut report = ArchitectureGovernanceEntity::new();
        for result in list.values {
            report.add_result(result);
        }
        report
    }
}
