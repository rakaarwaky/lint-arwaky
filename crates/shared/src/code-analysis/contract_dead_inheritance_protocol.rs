// PURPOSE: IDeadInheritanceProtocol — port trait for AES024: detect empty struct/impl blocks
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IDeadInheritanceProtocol: Send + Sync {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
