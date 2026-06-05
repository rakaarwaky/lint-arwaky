// linter_adapter_port — Abstract port for linter adapter operations (Contract Layer).
use async_trait::async_trait;

use crate::taxonomy::{AdapterName, ComplianceStatus, FilePath, LintResultList, LinterOperationError};


/// Abstract interface for linter adapters.
/// Implemented by Infrastructure (e.g., RuffAdapter, MypyAdapter).
#[async_trait]
pub trait ILinterAdapterPort: Send + Sync {
    fn name(&self) -> AdapterName;
    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError>;
    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError>;
}
