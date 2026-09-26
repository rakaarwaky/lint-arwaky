// PURPOSE: role-domain capability contracts (AES102 `_protocol`).
//
// One file for the role feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub trait IAgentRoleProtocol: Send + Sync {
    /// AES405: enforce agent type composition.
    /// Rule 1 — >= 1 struct must implement an aggregate trait.
    /// Rule 2 — max 3 types (struct + enum).
    fn check_agent_routing(&self, file: &FileEntry, layer: &str, violations: &mut Vec<LintResult>);
}

pub trait ICapabilitiesRoleProtocol: Send + Sync {
    fn check_capability_routing(
        &self,
        file: &FileEntry,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}

pub trait IContractRoleProtocol: Send + Sync {
    fn check_protocol(&self, file: &FileEntry) -> Vec<LintResult>;
    fn check_aggregate(&self, file: &FileEntry) -> Vec<LintResult>;
}

pub trait ISurfaceRoleProtocol: Send + Sync {
    fn check_smart_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_utility_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_passive_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_fn_count_limit(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
}

pub trait ITaxonomyRoleProtocol: Send + Sync {
    fn check_entity(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_error(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_event(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_constant(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
}

pub trait IUtilityRoleProtocol: Send + Sync {
    fn check_utility_convention(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
}
