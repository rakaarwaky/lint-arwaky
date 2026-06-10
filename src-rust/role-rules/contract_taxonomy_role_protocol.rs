// PURPOSE: Protocol: Contract trait for Taxonomy Role
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait ITaxonomyRoleChecker: Send + Sync {
    fn check_vo(&self) -> Vec<LintResult>;
    fn check_entity(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_error(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_event(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_constant(&self, file: &str, violations: &mut Vec<LintResult>);
}
