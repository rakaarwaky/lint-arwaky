// PURPOSE: IDeadInheritanceProtocol — protocol trait for AES303 sub-check 2: detect empty struct/impl blocks
use crate::cli_commands::taxonomy_result_vo::LintResult;

pub trait IDeadInheritanceProtocol: Send + Sync {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
