use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IConstantPurityProtocol: Send + Sync {
    fn check_constant_purity(&self, file: &str, violations: &mut Vec<LintResult>);
}
