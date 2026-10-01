// PURPOSE: root container — doc-rules DI composition root
//
// Wires the five capability seams into the agent behind the contract trait.
// The container is where concrete capability types are named — the agent and
// the capabilities never see each other, only the protocols. No business logic
// lives here.
use crate::agent_doc_orchestrator::DocOrchestrator;
use crate::capabilities_doc_crosslink_checker::CrosslinkChecker;
use crate::capabilities_doc_fr_format_checker::FrFormatChecker;
use crate::capabilities_doc_heading_structure_checker::HeadingStructureChecker;
use crate::capabilities_doc_section_structure_checker::SectionStructureChecker;
use crate::capabilities_doc_spec_purity_checker::SpecPurityChecker;
use shared_doc_rules::contract_doc_aggregate::IDocRunnerAggregate;
use shared_doc_rules::contract_doc_protocol::{
    ICrosslinkProtocol, IDocHeadingProtocol, IFrFormatProtocol, ISectionStructureProtocol,
    ISpecPurityProtocol,
};
use std::sync::Arc;

/// Composition root for the doc-rules feature.
pub struct RootDocRulesContainer;

impl RootDocRulesContainer {
    /// Return a fresh orchestrator for this workspace.
    pub fn orchestrator() -> Arc<dyn IDocRunnerAggregate> {
        let fr_format: Arc<dyn IFrFormatProtocol> = Arc::new(FrFormatChecker::new());
        let section_structure: Arc<dyn ISectionStructureProtocol> =
            Arc::new(SectionStructureChecker::new());
        let spec_purity: Arc<dyn ISpecPurityProtocol> = Arc::new(SpecPurityChecker::new());
        let crosslink: Arc<dyn ICrosslinkProtocol> = Arc::new(CrosslinkChecker::new());
        let doc_heading: Arc<dyn IDocHeadingProtocol> = Arc::new(HeadingStructureChecker::new());
        Arc::new(DocOrchestrator::new(
            fr_format,
            section_structure,
            spec_purity,
            crosslink,
            doc_heading,
        ))
    }
}
