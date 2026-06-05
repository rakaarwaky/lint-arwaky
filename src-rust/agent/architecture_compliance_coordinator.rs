// arch_compliance_coordinator — Coordinator bridging multiple IArchComplianceProtocol orchestrators.
use async_trait::async_trait;
use crate::contract::{ArchCoordinatorAggregate, IArchCompliancePort, IArchComplianceProtocol};
use crate::taxonomy::{AdapterName, ComplianceStatus, FilePath, LintResultList};

pub struct ArchComplianceCoordinator {
    orchestrators: Vec<Box<dyn IArchComplianceProtocol + Send + Sync>>,
}

impl ArchComplianceCoordinator {
    pub fn new(
        compliance_orchestrator: Box<dyn IArchComplianceProtocol + Send + Sync>,
        additional_orchestrators: Option<Vec<Box<dyn IArchComplianceProtocol + Send + Sync>>>,
    ) -> Self {
        let mut orchestrators = vec![compliance_orchestrator];
        if let Some(additional) = additional_orchestrators {
            orchestrators.extend(additional);
        }
        Self { orchestrators }
    }

    pub fn name(&self) -> AdapterName {
        AdapterName::new("architecture").unwrap()
    }
}

#[async_trait]
impl ArchCoordinatorAggregate for ArchComplianceCoordinator {
    async fn check_compliance(&self, path: &FilePath) -> ComplianceStatus {
        let result = self.scan(path).await;
        ComplianceStatus::new(result.values.is_empty())
    }

    async fn scan(&self, path: &FilePath) -> LintResultList {
        let mut results = LintResultList::new(Vec::new());
        for orchestrator in &self.orchestrators {
            let mut partial = orchestrator.execute(path);
            results.values.append(&mut partial.values);
        }
        results
    }

    async fn apply_fix(&self, _path: &FilePath) -> ComplianceStatus {
        // Architecture fixes are not supported automatically yet
        ComplianceStatus::new(false)
    }
}

#[async_trait]
impl IArchCompliancePort for ArchComplianceCoordinator {
    async fn scan(&self, path: &FilePath) -> LintResultList {
        // Delegate to ArchCoordinatorAggregate::scan
        (self as &dyn ArchCoordinatorAggregate).scan(path).await
    }

    async fn apply_fix(&self, path: &FilePath) -> ComplianceStatus {
        (self as &dyn ArchCoordinatorAggregate).apply_fix(path).await
    }
}
