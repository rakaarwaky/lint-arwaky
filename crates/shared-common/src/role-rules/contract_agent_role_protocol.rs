// PURPOSE: IAgentRoleChecker — port trait for AES0305: agent role audits (container, orchestrator, lifecycle, file size, any type)
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::config_system::taxonomy_source_vo::SourceContentVO;

pub trait IAgentRoleChecker: Send + Sync {
    fn check_container(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_orchestrator(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_lifecycle(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_file_size_limit(
        &self,
        source: &SourceContentVO,
        max_lines: usize,
        violations: &mut Vec<LintResult>,
    );
    fn check_any_type_annotation(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
