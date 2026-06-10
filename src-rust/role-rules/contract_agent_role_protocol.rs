// PURPOSE: IAgentRoleChecker — port trait for AES0305: agent role audits (container, orchestrator, lifecycle, file size, any type)
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IAgentRoleChecker: Send + Sync {
    fn check_container(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_orchestrator(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_lifecycle(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_file_size_limit(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_any_type_annotation(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
