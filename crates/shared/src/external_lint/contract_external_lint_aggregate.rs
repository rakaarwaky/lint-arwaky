// PURPOSE: IExternalLintAggregate — contract for running external linter adapters
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_adapter_list_vo::AdapterNameList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::external_lint::taxonomy_external_lint_context::ExternalLintContext;

pub trait IExternalLintAggregate: Send + Sync {
    /// Legacy scan — delegates to `scan_all_with_context` with a default context.
    fn scan_all(&self, path: &FilePath) -> LintResultList;
    /// Scan with pre-computed context (zero I/O — all data provided by surface).
    fn scan_all_with_context(
        &self,
        path: &FilePath,
        context: &ExternalLintContext,
    ) -> LintResultList;
    fn adapter_names(&self) -> AdapterNameList;
}
