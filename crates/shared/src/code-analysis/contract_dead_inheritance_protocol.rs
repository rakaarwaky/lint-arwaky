// PURPOSE: IDeadInheritanceProtocol — protocol trait for AES303 sub-check 2: detect empty struct/impl blocks
use crate::cli_commands::taxonomy_result_vo::LintResult;

/// Protocol for detecting dead (empty) struct and impl blocks.
///
/// AES303 requires that every struct and impl block contain at least one
/// meaningful item. This protocol checks for violations and appends them
/// to the provided violations vector.
pub trait IDeadInheritanceProtocol: Send + Sync {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
