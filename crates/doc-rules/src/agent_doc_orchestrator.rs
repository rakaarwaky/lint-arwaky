// PURPOSE: DocOrchestrator — agent that orchestrates doc-invariant checks
//
// The single entry point over the doc-rules feature. Consumers call
// `execute(DocRequest)`; the agent collects the document chain once, builds
// the shared audit context, and fans the request out to the nine AES doc-rule
// capabilities through their protocol traits. It holds the capabilities only as
// `Arc<dyn I*Protocol>` trait objects, so the agent never names a concrete
// capability and stays free of capability imports.
use shared_doc_rules::contract_doc_aggregate::IDocRunnerAggregate;
use shared_doc_rules::contract_doc_protocol::{
    ICrosslinkProtocol, IDocHeadingProtocol, IFrFormatProtocol, IMemberDocProtocol,
    IPrFolderOwnershipProtocol, IRequiredDocProtocol, ISectionStructureProtocol,
    ISpecPurityProtocol, ITemplateBodyProtocol, ITemplateConformanceProtocol,
};
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;
use shared_doc_rules::utility_doc_collector;
use std::sync::Arc;

// ─── Block 1: Struct Definition ──────────────────────────────────────────

/// Holds one injected capability per AES doc rule, keyed by the rule each
/// answers. The agent is the only place that knows all ten exist.
pub struct DocOrchestrator {
    fr_format: Arc<dyn IFrFormatProtocol>,
    section_structure: Arc<dyn ISectionStructureProtocol>,
    spec_purity: Arc<dyn ISpecPurityProtocol>,
    crosslink: Arc<dyn ICrosslinkProtocol>,
    doc_heading: Arc<dyn IDocHeadingProtocol>,
    template_conformance: Arc<dyn ITemplateConformanceProtocol>,
    required_doc: Arc<dyn IRequiredDocProtocol>,
    member_doc: Arc<dyn IMemberDocProtocol>,
    template_body: Arc<dyn ITemplateBodyProtocol>,
    pr_folder_ownership: Arc<dyn IPrFolderOwnershipProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ──────────────────────────────

impl IDocRunnerAggregate for DocOrchestrator {
    /// Audit the document chain under the request's root and merge what each
    /// capability reports into one ordered list.
    fn execute(&self, request: DocRequest) -> DocResponse {
        let DocRequest::AuditAll {
            root,
            config,
            strict_override,
        } = request;
        // The chain is walked once here rather than once per capability: the
        // rules read the same documents, so collecting per rule would
        // re-read the same files multiple times.
        let documents = utility_doc_collector::collect_documents(&root);
        let master = utility_doc_collector::master_text(&documents);
        let members = utility_doc_collector::collect_members(&root);
        let context =
            DocAuditContext::new(&root, documents, master, members, config, strict_override);

        let mut findings = Vec::new();
        findings.extend(self.fr_format.audit_fr_format(&context));
        findings.extend(self.section_structure.audit_section_structure(&context));
        findings.extend(self.spec_purity.audit_spec_purity(&context));
        findings.extend(self.crosslink.audit_crosslinks(&context));
        findings.extend(self.doc_heading.audit_doc_heading(&context));
        findings.extend(
            self.template_conformance
                .audit_template_conformance(&context),
        );
        findings.extend(self.required_doc.audit_required_docs(&context));
        findings.extend(self.member_doc.audit_member_docs(&context));
        findings.extend(self.template_body.audit_template_body(&context));
        findings.extend(self.pr_folder_ownership.audit_pr_folder_ownership(&context));

        DocResponse::Findings {
            findings: utility_doc_collector::sorted(findings),
        }
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ──────────────────────────

impl DocOrchestrator {
    pub fn new(
        fr_format: Arc<dyn IFrFormatProtocol>,
        section_structure: Arc<dyn ISectionStructureProtocol>,
        spec_purity: Arc<dyn ISpecPurityProtocol>,
        crosslink: Arc<dyn ICrosslinkProtocol>,
        doc_heading: Arc<dyn IDocHeadingProtocol>,
        template_conformance: Arc<dyn ITemplateConformanceProtocol>,
        required_doc: Arc<dyn IRequiredDocProtocol>,
        member_doc: Arc<dyn IMemberDocProtocol>,
        template_body: Arc<dyn ITemplateBodyProtocol>,
        pr_folder_ownership: Arc<dyn IPrFolderOwnershipProtocol>,
    ) -> Self {
        Self {
            fr_format,
            section_structure,
            spec_purity,
            crosslink,
            doc_heading,
            template_conformance,
            required_doc,
            member_doc,
            template_body,
            pr_folder_ownership,
        }
    }
}
