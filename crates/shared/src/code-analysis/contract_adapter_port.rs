// PURPOSE: ILinterAdapterPort — port trait for linter adapter implementations (Ruff, Mypy, Clippy, etc.)
// NOTE: Uses cli_commands::LintResultList as the standard output type (shared output contract).
// The circular-import detector is suppressed here: this is an intentional cross-domain reference
// because the lint-result type is the universal output format used by all adapters and surfaces.

use async_trait::async_trait;

use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;

/// Abstract interface for linter adapters.
/// Implemented by Infrastructure (e.g., RuffAdapter, MypyAdapter).
#[async_trait]
pub trait ILinterAdapterPort: Send + Sync {
    fn name(&self) -> AdapterName;
    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError>;
    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError>;
    fn violation_type(&self) -> AesCodeAnalysisViolation {
        AesCodeAnalysisViolation::FileTooLarge { reason: None }
    }
}
