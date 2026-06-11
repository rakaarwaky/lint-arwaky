// PURPOSE: IFixProtocol — protocol trait for auto-fix operations (capabilities layer)
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IFixProtocol: Send + Sync {
    /// Execute auto-fix on a file, returns fix result
    fn execute(&self, path: &FilePath) -> FixResult;

    /// Check if a specific violation is fixable
    fn is_fixable(&self, violation: &LintResult) -> bool;

    /// Get list of fixable violation codes
    fn fixable_codes(&self) -> &[&str];

    /// Report violations that cannot be auto-fixed
    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<String>;
}
