// PURPOSE: ICapabilitiesRoleChecker — port trait for AES0303: capability routing bottlenecks and role audits
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait ICapabilitiesRoleChecker: Send + Sync {
    fn check_capability_routing(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}
