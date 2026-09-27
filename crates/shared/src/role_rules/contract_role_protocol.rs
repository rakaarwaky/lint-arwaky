// PURPOSE: role-domain capability contracts (AES102 `_protocol`).
//
// One file for the role feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::ExternalReferenceMap;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub trait IAgentRoleProtocol: Send + Sync {
    /// AES405: enforce agent type composition.
    /// Rule 1 — >= 1 struct must implement an aggregate trait.
    /// Rule 2 — max 3 types (struct + enum).
    fn check_agent_routing(&self, file: &FileEntry, layer: &str, violations: &mut Vec<LintResult>);
}

pub trait ICapabilitiesRoleProtocol: Send + Sync {
    /// Entry point for every AES403 capability sub-check.
    ///
    /// Runs all sub-checks in sequence; each appends to `violations`
    /// independently, so one file can report several defects.
    fn check_capability_routing(
        &self,
        file: &FileEntry,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );

    /// Every sub-check in sequence, with workspace-wide reference data.
    ///
    /// This is what the orchestrator calls. `references` lets a sub-check
    /// decide whether a `pub` helper is genuinely called from outside its own
    /// file, so it does not flag API that other modules rely on. Checks that
    /// need no reference data take the same call shape for uniformity.
    fn check_capability_routing_with_references(
        &self,
        file: &FileEntry,
        layer: &str,
        references: &ExternalReferenceMap,
        violations: &mut Vec<LintResult>,
    );

    /// Capability must not declare more than 3 types (struct + enum). HIGH.
    fn check_capability_type_budget(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// Capability must implement at least one contract protocol trait. MEDIUM.
    ///
    /// An aggregate trait does not satisfy this — aggregates belong to the
    /// agent layer (AES405).
    fn check_capability_implementor(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// Block 2 (protocol trait impl) must precede Block 3 (inherent impl).
    /// Severity HIGH.
    fn check_capability_block_order(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// Constants belong in `taxonomy_<domain>_constant.rs`, not inline. MEDIUM.
    fn check_capability_constant_placement(
        &self,
        file: &FileEntry,
        violations: &mut Vec<LintResult>,
    );

    /// Test code belongs in `tests/`, not inline in the capability file. LOW.
    fn check_capability_test_placement(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// A Block 3 helper must not be `pub` with nothing outside the file using it.
    /// Severity MEDIUM.
    ///
    /// A helper that other modules call stays `pub` and is not reported. A
    /// helper covered only by an integration test also stays `pub`, because
    /// `pub(crate)` is invisible to a `tests/` target — reporting those would
    /// tell the author to demote visibility and break the suite.
    fn check_capability_helper_visibility(
        &self,
        file: &FileEntry,
        references: &ExternalReferenceMap,
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
