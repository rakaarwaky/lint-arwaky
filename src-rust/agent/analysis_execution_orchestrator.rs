// analysis_execution_orchestrator — Implementation of the analysis orchestration domain contract.
use crate::contract::{
    AnalysisOrchestratorAggregate, ServiceContainerAggregate,
};
use crate::taxonomy::{FilePath, GovernanceReport, LintResultList};
use async_trait::async_trait;

pub struct AnalysisOrchestrator;

struct DummyContainer;
impl ServiceContainerAggregate for DummyContainer {}

#[async_trait]
impl AnalysisOrchestratorAggregate for AnalysisOrchestrator {
    fn container(&self) -> &dyn ServiceContainerAggregate {
        static DUMMY: DummyContainer = DummyContainer;
        &DUMMY
    }

    async fn run(&self, _path: &FilePath) -> GovernanceReport {
        GovernanceReport::default()
    }

    async fn get_complexity(&self, path: &FilePath) -> GovernanceReport {
        let report = self.run(path).await;
        let source_adapter = crate::taxonomy::AdapterName::new("radon").unwrap();
        self.report_from_list(report.results_by_source(&source_adapter))
    }

    async fn get_duplicates(&self, path: &FilePath) -> GovernanceReport {
        let report = self.run(path).await;
        let source_adapter = crate::taxonomy::AdapterName::new("duplicates").unwrap();
        self.report_from_list(report.results_by_source(&source_adapter))
    }

    async fn get_trends(&self, path: &FilePath) -> GovernanceReport {
        let report = self.run(path).await;
        let source_adapter = crate::taxonomy::AdapterName::new("trends").unwrap();
        self.report_from_list(report.results_by_source(&source_adapter))
    }

    async fn get_dependencies(&self, path: &FilePath) -> GovernanceReport {
        let report = self.run(path).await;
        let source_adapter = crate::taxonomy::AdapterName::new("pip-audit").unwrap();
        self.report_from_list(report.results_by_source(&source_adapter))
    }
}

impl AnalysisOrchestrator {
    fn report_from_list(&self, list: LintResultList) -> GovernanceReport {
        let mut report = GovernanceReport::new();
        for result in list.values {
            report.add_result(result);
        }
        report
    }
}
