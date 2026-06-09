// linter_adapter_port — Abstract port for linter adapter operations (Contract Layer).
use async_trait::async_trait;

use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::shared_common::taxonomy_operation_error::LinterOperationError;
use crate::source_parsing::taxonomy_path_vo::FilePath;

/// Abstract interface for linter adapters.
/// Implemented by Infrastructure (e.g., RuffAdapter, MypyAdapter).
#[async_trait]
pub trait ILinterAdapterPort: Send + Sync {
    fn name(&self) -> AdapterName;
    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError>;
    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError>;
}
