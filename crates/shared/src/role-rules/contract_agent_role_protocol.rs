// PURPOSE: IAgentRoleChecker — contract trait for AES405: agent type composition
//          and any-type annotation checks.
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait IAgentRoleChecker: Send + Sync {
    /// AES405: enforce agent type composition.
    /// Rule 1 — internal helper types allowed.
    /// Rule 2 — ≥ 1 struct must implement an aggregate trait.
    /// Rule 3 — max 3 types (struct + enum).
    fn check_agent_routing(
        &self,
        source: &SourceContentVO,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}
