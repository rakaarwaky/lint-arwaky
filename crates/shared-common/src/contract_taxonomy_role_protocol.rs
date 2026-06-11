// PURPOSE: ITaxonomyRoleChecker — port trait for AES0301: taxonomy role audits (VO, entity, error, event, constant)
use output_report::taxonomy_result_vo::LintResult;
use shared_common::taxonomy_source_vo::SourceContentVO;

pub trait ITaxonomyRoleChecker: Send + Sync {
    fn check_vo(&self) -> Vec<LintResult>;
    fn check_entity(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_error(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_event(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_constant(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
