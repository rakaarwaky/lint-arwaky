use crate::layer_rules::contract_compliance_port::IArchCompliancePort;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

pub type CoordCompliancePort = Box<dyn IArchCompliancePort>;

#[async_trait]
pub trait ArchCoordinatorAggregate: Send + Sync {
    async fn check_compliance(&self, path: &FilePath) -> ComplianceStatus;
    async fn scan(&self, path: &FilePath) -> LintResultList;
    async fn apply_fix(&self, path: &FilePath) -> ComplianceStatus;
}
