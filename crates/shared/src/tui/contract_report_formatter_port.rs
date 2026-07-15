use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use crate::project_setup::taxonomy_doctor_vo::{DependencyReport, ToolchainDiagnostics};
use crate::tui::taxonomy_lint_result_vo::LintExecutionResult;

pub trait IReportFormatterPort: Send + Sync {
    fn format_results(&self, results: &LintResultList) -> String;
    fn format_doctor_report(&self, diagnostics: &ToolchainDiagnostics) -> LintExecutionResult;
    fn format_dependency_report(
        &self,
        path: &str,
        report: &DependencyReport,
    ) -> LintExecutionResult;
    fn format_config_result(&self, result: &ConfigResult) -> LintExecutionResult;
}
