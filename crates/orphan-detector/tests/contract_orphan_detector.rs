// PURPOSE: Verify all trait implementations exist for orphan-detector structs.
// Layer: Contract
// Speed: ms

use orphan_detector_lint_arwaky::agent_orphan_orchestrator::ArchOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use orphan_detector_lint_arwaky::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use orphan_detector_lint_arwaky::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer;
use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;

use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use shared::orphan_detector::contract_orphan_protocol::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol, IUtilityOrphanProtocol,
};

// ─── IOrphanAggregate ─────────────────────────────────────

#[test]
fn arch_orphan_analyzer_implements_i_orphan_aggregate() {
    fn assert_trait<T: IOrphanAggregate>() {}
    assert_trait::<ArchOrphanAnalyzer>();
}

// ─── IOrphanGraphResolverProtocol ─────────────────────────

#[test]
fn orphan_graph_resolver_implements_i_orphan_graph_resolver_protocol() {
    fn assert_trait<T: IOrphanGraphResolverProtocol>() {}
    assert_trait::<OrphanGraphResolver>();
}

// ─── ITaxonomyOrphanProtocol ──────────────────────────────

#[test]
fn taxonomy_orphan_analyzer_implements_i_taxonomy_orphan_protocol() {
    fn assert_trait<T: ITaxonomyOrphanProtocol>() {}
    assert_trait::<TaxonomyOrphanAnalyzer>();
}

// ─── IContractOrphanProtocol ──────────────────────────────

#[test]
fn contract_orphan_analyzer_implements_i_contract_orphan_protocol() {
    fn assert_trait<T: IContractOrphanProtocol>() {}
    assert_trait::<ContractOrphanAnalyzer>();
}

// ─── ICapabilitiesOrphanProtocol ──────────────────────────

#[test]
fn capabilities_orphan_analyzer_implements_i_capabilities_orphan_protocol() {
    fn assert_trait<T: ICapabilitiesOrphanProtocol>() {}
    assert_trait::<CapabilitiesOrphanAnalyzer>();
}

// ─── IUtilityOrphanProtocol ───────────────────────────────

#[test]
fn utility_orphan_analyzer_implements_i_utility_orphan_protocol() {
    fn assert_trait<T: IUtilityOrphanProtocol>() {}
    assert_trait::<UtilityOrphanAnalyzer>();
}

// ─── IAgentOrphanProtocol ─────────────────────────────────

#[test]
fn agent_orphan_analyzer_implements_i_agent_orphan_protocol() {
    fn assert_trait<T: IAgentOrphanProtocol>() {}
    assert_trait::<AgentOrphanAnalyzer>();
}

// ─── ISurfacesOrphanProtocol ──────────────────────────────

#[test]
fn surfaces_orphan_analyzer_implements_i_surfaces_orphan_protocol() {
    fn assert_trait<T: ISurfacesOrphanProtocol>() {}
    assert_trait::<SurfacesOrphanAnalyzer>();
}

// ─── Send + Sync bounds ───────────────────────────────────

#[test]
fn all_analyzers_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ArchOrphanAnalyzer>();
    assert_send_sync::<OrphanGraphResolver>();
    assert_send_sync::<TaxonomyOrphanAnalyzer>();
    assert_send_sync::<ContractOrphanAnalyzer>();
    assert_send_sync::<CapabilitiesOrphanAnalyzer>();
    assert_send_sync::<UtilityOrphanAnalyzer>();
    assert_send_sync::<AgentOrphanAnalyzer>();
    assert_send_sync::<SurfacesOrphanAnalyzer>();
    assert_send_sync::<OrphanContainer>();
}
