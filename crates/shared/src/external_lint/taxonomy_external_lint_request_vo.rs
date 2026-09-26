// PURPOSE: ExternalLintRequestVO / ExternalLintResponseVO — aggregate request/response for external_lint
use crate::common::taxonomy_adapter_list_vo::AdapterNameList;
use crate::common::taxonomy_lint_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::external_lint::taxonomy_external_lint_vo::ExternalLintContext;

/// Consumer verb carried by the external-lint aggregate's single entry point.
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

/// Result of an external-lint aggregate request.
pub enum ExternalLintResponse {
    /// Violations reported by the adapters.
    Scan { violations: LintResultList },
    /// Names of the registered adapters.
    AdapterNames { names: AdapterNameList },
}

impl ExternalLintResponse {
    pub fn into_violations(self) -> LintResultList {
        match self {
            Self::Scan { violations } => violations,
            Self::AdapterNames { .. } => LintResultList::default(),
        }
    }

    pub fn into_adapter_names(self) -> AdapterNameList {
        match self {
            Self::AdapterNames { names } => names,
            Self::Scan { .. } => AdapterNameList::default(),
        }
    }
}
