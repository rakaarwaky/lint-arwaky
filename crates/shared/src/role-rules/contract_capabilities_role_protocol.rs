// PURPOSE: ICapabilitiesRoleChecker — port trait for AES0303: capability routing bottlenecks and role audits
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::config_system::taxonomy_source_vo::SourceContentVO;

pub trait ICapabilitiesRoleChecker: Send + Sync {
    fn check_capability_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}
