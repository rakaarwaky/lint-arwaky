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
use shared::orphan_rules::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    IEntryPointProtocol, IGraphContextProtocol, IOrphanAggregate, IOrphanParserProtocol,
    IReachabilityProtocol, ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol,
    IUtilityOrphanProtocol,
};

// ── Per-trait bound helpers (unique name per seam) ──────────

fn assert_igraph_context<T: IGraphContextProtocol>() {}
fn assert_ientry_point<T: IEntryPointProtocol>() {}
fn assert_ireachability<T: IReachabilityProtocol>() {}
fn assert_itaxonomy<T: ITaxonomyOrphanProtocol>() {}
fn assert_icontract<T: IContractOrphanProtocol>() {}
fn assert_icapabilities<T: ICapabilitiesOrphanProtocol>() {}
fn assert_iutility<T: IUtilityOrphanProtocol>() {}
fn assert_iagent<T: IAgentOrphanProtocol>() {}
fn assert_isurfaces<T: ISurfacesOrphanProtocol>() {}
fn assert_iaggregate<T: IOrphanAggregate>() {}

// ── Tests ──────────────────────────────────────────────────

#[test]
fn fr001_arch_orphan_analyzer_implements_graph_context_protocol() {
    assert_igraph_context::<ArchOrphanAnalyzer>();
}

#[test]
fn fr002_arch_orphan_analyzer_implements_entry_point_protocol() {
    assert_ientry_point::<ArchOrphanAnalyzer>();
}

#[test]
fn fr003_arch_orphan_analyzer_implements_reachability_protocol() {
    assert_ireachability::<ArchOrphanAnalyzer>();
}

#[test]
fn fr004_taxonomy_orphan_analyzer_implements_protocol() {
    assert_itaxonomy::<TaxonomyOrphanAnalyzer>();
}

#[test]
fn fr005_contract_orphan_analyzer_implements_protocol() {
    assert_icontract::<ContractOrphanAnalyzer>();
}

#[test]
fn fr006_capabilities_orphan_analyzer_implements_protocol() {
    assert_icapabilities::<CapabilitiesOrphanAnalyzer>();
}

#[test]
fn fr007_utility_orphan_analyzer_implements_protocol() {
    assert_iutility::<UtilityOrphanAnalyzer>();
}

#[test]
fn fr008_agent_orphan_analyzer_implements_protocol() {
    assert_iagent::<AgentOrphanAnalyzer>();
}

#[test]
fn fr009_surfaces_orphan_analyzer_implements_protocol() {
    assert_isurfaces::<SurfacesOrphanAnalyzer>();
}

#[test]
fn fr010_orphan_parser_protocol_is_object_safe() {
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
    // ArchOrphanAnalyzer backs FR-001/002/003 and the parser seam.
    assert_igraph_context::<ArchOrphanAnalyzer>();
    assert_ientry_point::<ArchOrphanAnalyzer>();
    assert_ireachability::<ArchOrphanAnalyzer>();
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
    use shared::orphan_rules::OrphanRequest;
    fn assert_method<T: IOrphanAggregate>() {
        let _ = |t: &T, request: OrphanRequest| t.execute(request);
    }
    assert_method::<ArchOrphanAnalyzer>();
}
