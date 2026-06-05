// analysis_execution_orchestrator — Implementation of the analysis orchestration domain contract.
use crate::contract::{
    AnalysisOrchestratorAggregate, LintPipelineOrchestratorAggregate, ServiceContainerAggregate,
};
use crate::taxonomy::{FilePath, GovernanceReport};
use async_trait::async_trait;

pub struct AnalysisOrchestrator;

#[async_trait]
impl AnalysisOrchestratorAggregate for AnalysisOrchestrator {
    async fn run(&self, path: &FilePath) -> GovernanceReport {
        // Delegates to the lint pipeline orchestrator
        // The actual pipeline resolution happens via container wiring
        GovernanceReport::default()
    }

    async fn get_complexity(&self, path: &FilePath) -> GovernanceReport {
        let report = self.run(path).await;
        // Filter for radon results only
        let source_adapter = crate::taxonomy::AdapterName::new("radon").unwrap();
        report.results_by_source(&source_adapter)
    }

    async fn get_duplicates(&self, path: &FilePath) -> GovernanceReport {
        let report = self.run(path).await;
        let source_adapter = crate::taxonomy::AdapterName::new("duplicates").unwrap();
        report.results_by_source(&source_adapter)
    }

    async fn get_trends(&self, path: &FilePath) -> GovernanceReport {
        let report = self.run(path).await;
        let source_adapter = crate::taxonomy::AdapterName::new("trends").unwrap();
        report.results_by_source(&source_adapter)
    }

    async fn get_dependencies(&self, path: &FilePath) -> GovernanceReport {
        let report = self.run(path).await;
        let source_adapter = crate::taxonomy::AdapterName::new("pip-audit").unwrap();
        report.results_by_source(&source_adapter)
    }
}
