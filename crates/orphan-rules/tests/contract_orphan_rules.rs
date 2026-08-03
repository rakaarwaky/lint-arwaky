// Contract tests — verify all analyzer capabilities implement their declared protocol traits.
// Compile-time structural checks: each concrete type must satisfy its protocol trait.
use orphan_rules_lint_arwaky::agent_orphan_orchestrator::ArchOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use orphan_rules_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use shared::orphan_rules::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol, IOrphanAggregate,
    ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol, IUtilityOrphanProtocol,
};

// ── Trait-bound helpers ────────────────────────────────────

fn assert_itaxonomy<T: ITaxonomyOrphanProtocol>() {}
fn assert_icontract<T: IContractOrphanProtocol>() {}
fn assert_icapabilities<T: ICapabilitiesOrphanProtocol>() {}
fn assert_iutility<T: IUtilityOrphanProtocol>() {}
fn assert_iagent<T: IAgentOrphanProtocol>() {}
fn assert_isurfaces<T: ISurfacesOrphanProtocol>() {}
fn assert_iaggregate<T: IOrphanAggregate>() {}

// ── Tests ──────────────────────────────────────────────────

#[test]
fn taxonomy_orphan_analyzer_implements_protocol() {
    assert_itaxonomy::<TaxonomyOrphanAnalyzer>();
}

#[test]
fn contract_orphan_analyzer_implements_protocol() {
    assert_icontract::<ContractOrphanAnalyzer>();
}

#[test]
fn capabilities_orphan_analyzer_implements_protocol() {
    assert_icapabilities::<CapabilitiesOrphanAnalyzer>();
}

#[test]
fn utility_orphan_analyzer_implements_protocol() {
    assert_iutility::<UtilityOrphanAnalyzer>();
}

#[test]
fn agent_orphan_analyzer_implements_protocol() {
    assert_iagent::<AgentOrphanAnalyzer>();
}

#[test]
fn surfaces_orphan_analyzer_implements_protocol() {
    assert_isurfaces::<SurfacesOrphanAnalyzer>();
}

#[test]
fn arch_orphan_analyzer_implements_aggregate() {
    assert_iaggregate::<ArchOrphanAnalyzer>();
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
