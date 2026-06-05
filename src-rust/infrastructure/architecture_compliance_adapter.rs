/// arch_compliance_adapter — Infrastructure adapter that wraps architectural compliance checking.
use crate::contract::{IArchCompliancePort, ILinterAdapterPort};
use crate::taxonomy::{AdapterError, AdapterName, ComplianceStatus, ErrorMessage, FilePath, LintResultList, ScanError, LinterOperationError};
use std::sync::Arc;

pub struct ArchComplianceAdapter {
    coordinator: Arc<dyn IArchCompliancePort>,
}

impl ArchComplianceAdapter {
    pub fn new(coordinator: Arc<dyn IArchCompliancePort>) -> Self {
        Self { coordinator }
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for ArchComplianceAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::new("architecture").unwrap()
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        Ok(self.coordinator.scan(path).await)
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(self.coordinator.apply_fix(path).await)
    }
}
