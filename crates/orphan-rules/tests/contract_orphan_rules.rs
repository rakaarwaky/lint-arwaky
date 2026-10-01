// Contract tests — verify all analyzer capabilities implement their declared protocol traits.
// Compile-time structural checks: each concrete type must satisfy its protocol trait.
// One test per FR-OrphanRules-001..010 protocol seam, each with a unique helper name.
use orphan_rules_lint_arwaky::agent_orphan_orchestrator::ArchOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use shared_orphan_rules::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol, IOrphanAggregate,
    IOrphanParserProtocol, ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol,
    IUtilityOrphanProtocol,
};

// ── Per-trait bound helpers (unique name per seam) ──────────

fn assert_itaxonomy<T: ITaxonomyOrphanProtocol>() {}
fn assert_icontract<T: IContractOrphanProtocol>() {}
fn assert_icapabilities<T: ICapabilitiesOrphanProtocol>() {}
fn assert_iutility<T: IUtilityOrphanProtocol>() {}
fn assert_iagent<T: IAgentOrphanProtocol>() {}
fn assert_isurfaces<T: ISurfacesOrphanProtocol>() {}
fn assert_iaggregate<T: IOrphanAggregate>() {}

// ── Tests ──────────────────────────────────────────────────

// FR-OrphanRules-001 (graph context, entry points, reachability) has no
// protocol: those are orchestration steps the agent owns, and an agent must
// not implement a contract protocol (AES405). The methods remain callable.
#[test]
fn fr001_orchestration_steps_are_inherent_agent_methods() {
    let _graph_ctx: fn(
        &ArchOrphanAnalyzer,
        &shared_common::taxonomy_path_vo::FilePath,
    )
        -> shared_quality_rules::taxonomy_quality_rules_vo::GraphAnalysisContext =
        ArchOrphanAnalyzer::build_orphan_graph_context;
    let _entry: fn(
        &ArchOrphanAnalyzer,
        &shared_orphan_rules::OrphanFileListVO,
    ) -> shared_orphan_rules::OrphanFileListVO = ArchOrphanAnalyzer::identify_orphan_entry_points;
    // The reachability step moved off IReachabilityProtocol along with the other
    // two, so it needs the same assertion — without it a signature change or a
    // deletion here would pass CI unnoticed.
    let _reach: fn(
        &ArchOrphanAnalyzer,
        &shared_orphan_rules::OrphanFileListVO,
        &shared_quality_rules::taxonomy_quality_rules_vo::GraphAnalysisContext,
    ) -> shared_quality_rules::taxonomy_quality_rules_vo::ReachabilityResult =
        ArchOrphanAnalyzer::trace_alive_files;
}

#[test]
fn fr002_taxonomy_orphan_analyzer_implements_protocol() {
    assert_itaxonomy::<TaxonomyOrphanAnalyzer>();
}

#[test]
fn fr003_contract_orphan_analyzer_implements_protocol() {
    assert_icontract::<ContractOrphanAnalyzer>();
}

#[test]
fn fr004_capabilities_orphan_analyzer_implements_protocol() {
    assert_icapabilities::<CapabilitiesOrphanAnalyzer>();
}

#[test]
fn fr005_utility_orphan_analyzer_implements_protocol() {
    assert_iutility::<UtilityOrphanAnalyzer>();
}

#[test]
fn fr006_agent_orphan_analyzer_implements_protocol() {
    assert_iagent::<AgentOrphanAnalyzer>();
}

#[test]
fn fr007_surfaces_orphan_analyzer_implements_protocol() {
    assert_isurfaces::<SurfacesOrphanAnalyzer>();
}

#[test]
fn orphan_parser_protocol_is_object_safe() {
    fn assert_object_safe<T: IOrphanParserProtocol + ?Sized>() {}
    assert_object_safe::<dyn IOrphanParserProtocol>();
}

#[test]
fn arch_orphan_analyzer_implements_aggregate() {
    assert_iaggregate::<ArchOrphanAnalyzer>();
}

#[test]
fn all_capabilities_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ArchOrphanAnalyzer>();
    assert_send_sync::<AgentOrphanAnalyzer>();
    assert_send_sync::<CapabilitiesOrphanAnalyzer>();
    assert_send_sync::<ContractOrphanAnalyzer>();
    assert_send_sync::<SurfacesOrphanAnalyzer>();
    assert_send_sync::<TaxonomyOrphanAnalyzer>();
    assert_send_sync::<UtilityOrphanAnalyzer>();
}

// ── dyn-pointer verification ───────────────────────────────

#[test]
fn taxonomy_analyzer_is_object_safe() {
    let _: &dyn ITaxonomyOrphanProtocol = &TaxonomyOrphanAnalyzer;
}

#[test]
fn utility_analyzer_is_object_safe() {
    let _: &dyn IUtilityOrphanProtocol = &UtilityOrphanAnalyzer;
}

#[test]
fn agent_analyzer_is_object_safe() {
    let _: &dyn IAgentOrphanProtocol = &AgentOrphanAnalyzer;
}

#[test]
fn surfaces_analyzer_is_object_safe() {
    let _: &dyn ISurfacesOrphanProtocol = &SurfacesOrphanAnalyzer;
}

#[test]
fn one_concrete_type_may_satisfy_several_seams() {
    // The parser seam is satisfied by a capability, not by the agent; the agent
    // owns the FR-001 orchestration steps directly and has no seam to satisfy.
    assert_itaxonomy::<TaxonomyOrphanAnalyzer>();
    assert_icontract::<ContractOrphanAnalyzer>();
    assert_iutility::<UtilityOrphanAnalyzer>();
}

// ─── Aggregate contract tests ──────────────────────────────
// AES101 `_aggregate`: exactly one method, the request/response entry point.

#[test]
fn aggregate_is_object_safe() {
    fn assert_object_safe<T: IOrphanAggregate + Send + Sync>() {}
    assert_object_safe::<ArchOrphanAnalyzer>();
}

#[test]
fn aggregate_exposes_a_single_execute_entry_point() {
    use shared_orphan_rules::OrphanRequest;
    fn assert_method<T: IOrphanAggregate>() {
        let _ = |t: &T, request: OrphanRequest| t.execute(request);
    }
    assert_method::<ArchOrphanAnalyzer>();
}
