// PURPOSE: IArchLintProtocol — port trait for self-linting the linter's own codebase and formatting reports

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;

pub trait IArchLintProtocol: Send + Sync {
    fn run_self_lint(&self, project_root: &str) -> LintResultList;
    fn run_self_lint_dir(&self, src_dir: &str) -> LintResultList;
    fn run_lint(&self, path: &str) -> Vec<LintResult>;
    fn calc_score(&self, results: &[LintResult]) -> f64;
    fn check_critical(&self, results: &[LintResult]) -> bool;
    fn format_report(&self, results: &LintResultList, project_root: &str) -> String;
}
