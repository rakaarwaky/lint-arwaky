// linter_adapter_port — Abstract port for linter adapter operations (Contract Layer).
use async_trait::async_trait;

use crate::taxonomy::adapter_collection_vo::AdapterClassMap;
use crate::taxonomy::config_setting_vo::ActualValue;
use crate::taxonomy::config_setting_vo::AdapterEntry;
use crate::taxonomy::job_action_vo::ActionArgs;
use crate::taxonomy::job_action_vo::ActionName;
use crate::taxonomy::lint_adapter_error::AdapterError;
use crate::taxonomy::lint_status_vo::AdapterMetadata;
use crate::taxonomy::source_system_error::AccessDeniedError;

/// Abstract interface for linter adapters.
/// Implemented by Infrastructure (e.g., RuffAdapter, MypyAdapter).
#[async_trait]
pub trait ILinterAdapterPort: Send + Sync {
    fn name(&self) -> AdapterName;
    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError>;
    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError>;
}
