// PURPOSE: IAgentRoleChecker — contract trait for AES405: agent type composition
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub trait IAgentRoleChecker: Send + Sync {
    /// AES405: enforce agent type composition.
    /// Rule 1 — >= 1 struct must implement an aggregate trait.
    /// Rule 2 — max 3 types (struct + enum).
    fn check_agent_routing(&self, file: &FileEntry, layer: &str, violations: &mut Vec<LintResult>);
}
