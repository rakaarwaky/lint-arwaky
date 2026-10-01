// PURPOSE: ExternalLintRequest — request payload for the external_lint aggregate

use crate::taxonomy_external_lint_vo::ExternalLintContext;
use shared_common::taxonomy_path_vo::FilePath;

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
