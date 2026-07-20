// PURPOSE: ICapabilitiesRoleChecker — protocol trait for AES403: capability routing bottlenecks and role audits
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait ICapabilitiesRoleChecker: Send + Sync {
    fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}
