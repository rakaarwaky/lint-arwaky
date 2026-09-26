// PURPOSE: ExternalLintRequest — request payload for the external_lint aggregate

use crate::common::taxonomy_path_vo::FilePath;
use crate::external_lint::taxonomy_external_lint_vo::ExternalLintContext;

pub enum ExternalLintRequest {
    /// Scan a path, letting the aggregate build its own default context.
    ScanAll { path: FilePath },
    /// Scan a path with a pre-computed context (zero I/O — surface supplies all data).
    ScanAllWithContext {
        path: FilePath,
        context: ExternalLintContext,
    },
    /// Report the names of the registered adapters.
    AdapterNames,
}

impl ExternalLintRequest {
    pub fn scan_all(path: &FilePath) -> Self {
        Self::ScanAll { path: path.clone() }
    }

    pub fn scan_all_with_context(path: &FilePath, context: &ExternalLintContext) -> Self {
        Self::ScanAllWithContext {
            path: path.clone(),
            context: context.clone(),
        }
    }

    pub fn adapter_names() -> Self {
        Self::AdapterNames
    }
}
