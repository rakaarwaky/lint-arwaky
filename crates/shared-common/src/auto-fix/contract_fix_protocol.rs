// PURPOSE: IFixProtocol — protocol trait for auto-fix operations (capabilities layer)
use auto_fix::taxonomy_fix_vo::FixResult;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IFixProtocol: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
    fn fix_bypass_comments(&self, file_path: &str, line: u32) -> bool;
    fn fix_unused_import(&self, file_path: &str, line: u32) -> bool;
    fn emit_fix_event(&self, path: &FilePath, error_code: &str, changes: usize);
    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<String>;
    fn is_fixable(&self, violation: &LintResult) -> bool;
    fn fixable_codes(&self) -> &[&str];
}
