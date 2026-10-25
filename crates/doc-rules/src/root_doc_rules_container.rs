// PURPOSE: root container — doc-rules DI composition root
//
// Wires the nine capability seams into the agent behind the contract trait.
// The container is where concrete capability types are named — the agent and
// the capabilities never see each other, only the protocols. No business logic
// lives here.
use crate::agent_doc_orchestrator::DocOrchestrator;
use crate::capabilities_doc_crosslink_checker::CrosslinkChecker;
use crate::capabilities_doc_fr_format_checker::FrFormatChecker;
use crate::capabilities_doc_heading_structure_checker::HeadingStructureChecker;
use crate::capabilities_doc_member_doc_checker::MemberDocChecker;
use crate::capabilities_doc_required_doc_checker::RequiredDocChecker;
use crate::capabilities_doc_section_structure_checker::SectionStructureChecker;
use crate::capabilities_doc_spec_purity_checker::SpecPurityChecker;
use crate::capabilities_doc_template_body_checker::TemplateBodyChecker;
use crate::capabilities_doc_template_conformance_checker::TemplateConformanceChecker;
use shared_doc_rules::contract_doc_aggregate::IDocRunnerAggregate;
use shared_doc_rules::contract_doc_protocol::{
    ICrosslinkProtocol, IDocHeadingProtocol, IFrFormatProtocol, IMemberDocProtocol,
    IRequiredDocProtocol, ISectionStructureProtocol, ISpecPurityProtocol, ITemplateBodyProtocol,
    ITemplateConformanceProtocol,
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
        let template_conformance: Arc<dyn ITemplateConformanceProtocol> =
            Arc::new(TemplateConformanceChecker::new());
        let required_doc: Arc<dyn IRequiredDocProtocol> = Arc::new(RequiredDocChecker::new());
        let member_doc: Arc<dyn IMemberDocProtocol> = Arc::new(MemberDocChecker::new());
        let template_body: Arc<dyn ITemplateBodyProtocol> = Arc::new(TemplateBodyChecker::new());
        Arc::new(DocOrchestrator::new(
            fr_format,
            section_structure,
            spec_purity,
            crosslink,
            doc_heading,
            template_conformance,
            required_doc,
            member_doc,
            template_body,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RootDocRulesContainer::orchestrator creates a valid aggregate without panic.
    #[test]
    fn orchestrator_instantiation_succeeds() {
        let _orch = RootDocRulesContainer::orchestrator();
    }
}
