use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IAgentRoleChecker: Send + Sync {
    fn check_container(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_orchestrator(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_coordinator(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_registry(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_manager(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_mixin(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_state(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
