/// linter_adapter_port — Port interface for external linting tools.
///
/// Infrastructure implements this. Capabilities consume it via DI.

use crate::taxonomy::{
    AdapterName, AdapterError, ComplianceStatus, FilePath, LintResultList, ScanError,
};

/// Port interface for external linting tools.
#[async_trait::async_trait]
pub trait ILinterAdapterPort: Send + Sync {
    /// Scan the given path and return a list of LintResult.
    async fn scan(&self, path: FilePath) -> Result<LintResultList, AdapterErrorOrScanError>;

    /// Apply automatic fixes to the given path.
    async fn apply_fix(&self, path: FilePath) -> Result<ComplianceStatus, AdapterError>;

    /// Return the name of the tool (e.g., 'ruff').
    fn name(&self) -> AdapterName;
}

// Define a combined error type for scan operation (since it can return either ScanError or AdapterError)
#[derive(Debug, thiserror::Error)]
pub enum AdapterErrorOrScanError {
    #[error("Adapter error: {0}")]
    AdapterError(#[from] AdapterError),

    #[error("Scan error: {0}")]
    ScanError(#[from] ScanError),
}
