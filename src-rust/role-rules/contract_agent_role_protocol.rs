// PURPOSE: Protocol: Contract trait for Agent Role
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IAgentRoleChecker: Send + Sync {
    fn check_container(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_orchestrator(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_lifecycle(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
