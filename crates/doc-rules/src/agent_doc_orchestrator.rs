// PURPOSE: DocOrchestrator — agent that orchestrates doc-invariant checks
//
// The single entry point over the doc-rules feature. Consumers call
// `execute(DocRequest)`; the agent collects the document chain once, builds
// the shared audit context, and fans the request out to the five AES doc-rule
// capabilities through their protocol traits. It holds the capabilities only as
// `Arc<dyn I*Protocol>` trait objects, so the agent never names a concrete
// capability and stays free of capability imports.
use shared_doc_rules::contract_doc_aggregate::IDocRunnerAggregate;
use shared_doc_rules::contract_doc_protocol::{
    ICrosslinkProtocol, IDocHeadingProtocol, IFrFormatProtocol, ISectionStructureProtocol,
    ISpecPurityProtocol,
};
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;
use shared_doc_rules::utility_doc_collector;
use std::sync::Arc;

// ─── Block 1: Struct Definition ────────────────────────────

/// Holds one injected capability per AES doc rule, keyed by the rule each
/// answers. The agent is the only place that knows all five exist.
pub struct DocOrchestrator {
    fr_format: Arc<dyn IFrFormatProtocol>,
    section_structure: Arc<dyn ISectionStructureProtocol>,
    spec_purity: Arc<dyn ISpecPurityProtocol>,
    crosslink: Arc<dyn ICrosslinkProtocol>,
    doc_heading: Arc<dyn IDocHeadingProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IDocRunnerAggregate for DocOrchestrator {
    /// Audit the document chain under the request's root and merge what each
    /// capability reports into one ordered list.
    fn execute(&self, request: DocRequest) -> DocResponse {
        let DocRequest::AuditAll { root } = request;
        // The chain is walked once here rather than once per capability: the
        // five rules read the same documents, so collecting per rule would
        // re-read the same files five times.
        let documents = utility_doc_collector::collect_documents(&root);
        let master = utility_doc_collector::master_text(&documents);
        let context = DocAuditContext::new(&root, documents, master);

        let mut findings = Vec::new();
        findings.extend(self.fr_format.audit_fr_format(&context));
        findings.extend(self.section_structure.audit_section_structure(&context));
        findings.extend(self.spec_purity.audit_spec_purity(&context));
        findings.extend(self.crosslink.audit_crosslinks(&context));
        findings.extend(self.doc_heading.audit_doc_heading(&context));

        DocResponse::Findings {
            findings: utility_doc_collector::sorted(findings),
        }
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

impl DocOrchestrator {
    pub fn new(
        fr_format: Arc<dyn IFrFormatProtocol>,
        section_structure: Arc<dyn ISectionStructureProtocol>,
        spec_purity: Arc<dyn ISpecPurityProtocol>,
        crosslink: Arc<dyn ICrosslinkProtocol>,
        doc_heading: Arc<dyn IDocHeadingProtocol>,
    ) -> Self {
        Self {
            fr_format,
            section_structure,
            spec_purity,
            crosslink,
            doc_heading,
        }
    }
}
