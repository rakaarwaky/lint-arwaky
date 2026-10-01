// PURPOSE: role-domain capability contracts (AES102 `_protocol`).
//
// One file for the role feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs. One trait per FR-RoleRules-001..007.

use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_lint_result_vo::LintResult;
use shared_filesystem::taxonomy_filesystem_vo::ExternalReferenceMap;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;

/// FR-RoleRules-001: classify each file by its filename prefix to determine its
/// AES layer, then dispatch to the layer-specific role checker. The prefix is
/// the first `_`-separated segment of the file stem; `root` and unrecognised
/// prefixes are skipped.
///
/// The sole implementer is the `RoleClassifier` capability. The role-rules agent
/// holds one as a dependency and delegates the classification rather than
/// implementing this trait itself — an agent must never implement a contract
/// protocol (AES405).
pub trait IClassificationProtocol: Send + Sync {
    /// Resolve the AES layer name for a file from its filename prefix.
    /// Returns `None` when the prefix maps to no layer, so the file is skipped.
    fn classify_layer(&self, file: &FileEntry) -> Option<LayerNameVO>;
}

/// FR-RoleRules-002 (AES401): taxonomy purity and constant placement.
pub trait ITaxonomyRoleProtocol: Send + Sync {
    fn check_entity(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_error(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_event(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_constant(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
}

/// FR-RoleRules-003 (AES402): contract primitive restriction.
pub trait IContractRoleProtocol: Send + Sync {
    /// Entry point that runs all contract sub-checks in sequence.
    ///
    /// This is what the orchestrator calls. Each sub-check appends to
    /// `violations` independently, so one file can report several defects.
    fn check_contract_routing(
        &self,
        file: &FileEntry,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );

    /// Protocol or aggregate trait method must not have a default body. HIGH.
    fn check_contract_default_body(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// An aggregate trait must declare exactly one method (execute). HIGH.
    fn check_contract_aggregate_method_count(
        &self,
        file: &FileEntry,
        violations: &mut Vec<LintResult>,
    );

    /// An `execute(op: &str, ...)` or param-bag signature in a capability seam
    /// is a dispatch anti-pattern. MEDIUM.
    fn check_contract_dispatch_bag(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// A multi-variant response enum where every variant holds only primitives
    /// (no VO-wrapped fields) is an untyped return. MEDIUM.
    fn check_contract_untyped_return(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// Every sub-check in sequence, collecting the violations into a fresh list.
    ///
    /// A `_protocol` seam uses `layer = "contract"`. This wrapper exists for
    /// call sites that only need the result list and do not already hold a
    /// `violations` buffer.
    fn check_protocol(&self, file: &FileEntry) -> Vec<LintResult>;

    /// Same as `check_protocol`, for callers dispatching an `_aggregate` seam.
    ///
    /// The sub-checks read the file path themselves, so the aggregate seam is
    /// covered by the same sequence.
    fn check_aggregate(&self, file: &FileEntry) -> Vec<LintResult>;
}

/// FR-RoleRules-004 (AES403): capability protocol implementation.
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

    /// Exactly one contract protocol per capability file. MEDIUM.
    ///
    /// A capability file that implements two or more protocols should be
    /// split into separate files, one per capability. Shared helper functions
    /// that belong to more than one capability belong in a `utility_*` file.
    fn check_capability_single_protocol(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

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

/// FR-RoleRules-005 (AES404): utility purity.
pub trait IUtilityRoleProtocol: Send + Sync {
    fn check_utility_convention(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
}

/// FR-RoleRules-006 (AES405): agent orchestrator composition.
pub trait IAgentRoleProtocol: Send + Sync {
    /// AES405: the composition sub-checks, in the order the orchestrator
    /// calls them.
    ///
    /// Runs the implementor, type-budget, and Any-annotation rules only. The
    /// remaining sub-checks are called individually by the orchestrator, each
    /// against its own rule code, so a caller can run a specific rule without
    /// paying for the rest. `check_agent_routing` is the historical entry
    /// point and keeps its original three rules.
    fn check_agent_routing(&self, file: &FileEntry, layer: &str, violations: &mut Vec<LintResult>);

    /// Rule 1 — at least 1 type implements an aggregate trait. MEDIUM.
    fn check_agent_implementor(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// Rule 2 — at most 3 type declarations. HIGH.
    fn check_agent_type_budget(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// No `Any` / `any` type annotations. MEDIUM.
    fn check_agent_any_annotation(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// Block 2 (aggregate impl) must precede Block 3 (inherent impl). HIGH.
    fn check_agent_block_order(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// An agent file implements no contract protocol. HIGH.
    ///
    /// A flat prohibition, not a budget: no number of implementations is
    /// acceptable, and one alongside the aggregate is already a violation. An
    /// agent is the feature's composition root: it implements the feature
    /// aggregate and coordinates injected seams, so implementing a contract
    /// protocol as well duplicates a capability's job in the orchestration
    /// layer. A std trait impl (`Default`, `Display`, `Clone`) and the
    /// aggregate trait itself are not contract protocols and are reported as
    /// nothing.
    fn check_agent_protocol_forbidden(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// No block marker above 3. MEDIUM.
    ///
    /// A ceiling, not a sequence check: any `Block <n>:` banner with n > 3 is
    /// reported, and nothing else about the markers is examined. Block ordering
    /// and presence are a different check — `check_agent_block_order` handles
    /// Block 2 preceding Block 3.
    ///
    /// The 3-block structure is a readability contract: Block 1 types and
    /// injected deps, Block 2 the aggregate, Block 3 constructors and
    /// helpers. A fourth marker means the file has grown past the shape the
    /// HOW-TO documents, so the reader loses the block map. Markers are the
    /// `─── Block N:` banner comments; a file that carries none is not
    /// reported, since the marker is a convention rather than a requirement.
    fn check_agent_block_markers(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// No forbidden I/O operations in the agent file. MEDIUM.
    fn check_agent_io_forbidden(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// No file-level constants — policy constants belong in taxonomy. MEDIUM.
    fn check_agent_constant_placement(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// Agent must coordinate at least 2 injected protocol subsystems. LOW.
    ///
    /// `feature_protocol_count` is the number of protocol traits the feature's
    /// shared module declares. A feature that declares exactly one is a
    /// genuine single-subsystem feature, and the check is skipped for it:
    /// forcing a second protocol to satisfy a count would add a seam that does
    /// no distinct job. A feature that declares none (a test fixture with no
    /// shared module) is not skipped.
    fn check_agent_subsystem_count(
        &self,
        file: &FileEntry,
        feature_protocol_count: usize,
        violations: &mut Vec<LintResult>,
    );

    /// No computation (`.sum()`, `.fold()`, `.reduce()`, arithmetic). MEDIUM.
    fn check_agent_computation(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// No `@abstractmethod` in the agent file — an agent delegates, it does
    /// not declare abstract methods. MEDIUM.
    fn check_agent_abstract_method(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// No state mutation outside the constructor. MEDIUM.
    ///
    /// Direct reassignment (`self.x = ...`, `this.x = ...`, `self.x += ...`,
    /// `&mut self`) is reported. Appending to a collection field that the
    /// constructor initialised is a results buffer, not stored state, and is
    /// allowed.
    fn check_agent_stateless(&self, file: &FileEntry, violations: &mut Vec<LintResult>);

    /// No module-level free functions — they belong in a `*_utility.*` file. LOW.
    fn check_agent_free_fn(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
}

/// FR-RoleRules-007 (AES406): surface passive role.
pub trait ISurfaceRoleProtocol: Send + Sync {
    fn check_smart_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_utility_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_passive_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_fn_count_limit(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
}
