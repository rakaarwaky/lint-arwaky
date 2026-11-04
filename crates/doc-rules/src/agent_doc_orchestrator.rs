// doc-rules — DocOrchestrator: agent that orchestrates doc-invariant checks
//
// The single entry point over the doc-rules feature. Consumers call
// `execute(DocRequest)`; the agent collects the document chain once, builds
// the shared audit context, and fans the request out to the ten per-doc
// capabilities through their protocol traits. It holds the capabilities only
// as `Arc<dyn I*Protocol>` trait objects, so the agent never names a concrete
// capability and stays free of capability imports.
use shared_doc_rules::contract_doc_aggregate::IDocRunnerAggregate;
use shared_doc_rules::contract_doc_protocol::{
    IAgentsDocProtocol, IArchitectureDocProtocol, IBacklogDocProtocol, IContributingDocProtocol,
    IDataDocProtocol, IDesignDocProtocol, IFrdDocProtocol, IPrdDocProtocol, IReadmeDocProtocol,
    IRoadmapDocProtocol,
};
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;
use shared_doc_rules::utility_doc_collector;
use std::sync::Arc;

// ─── Block 1: Struct Definition ──────────────────────────────────────────

/// Holds one injected capability per AES document type, keyed by the doc each
/// answers. The agent is the only place that knows all ten exist.
pub struct DocOrchestrator {
    agents: Arc<dyn IAgentsDocProtocol>,
    architecture: Arc<dyn IArchitectureDocProtocol>,
    backlog: Arc<dyn IBacklogDocProtocol>,
    contributing: Arc<dyn IContributingDocProtocol>,
    data: Arc<dyn IDataDocProtocol>,
    design: Arc<dyn IDesignDocProtocol>,
    frd: Arc<dyn IFrdDocProtocol>,
    prd: Arc<dyn IPrdDocProtocol>,
    readme: Arc<dyn IReadmeDocProtocol>,
    roadmap: Arc<dyn IRoadmapDocProtocol>,
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
        findings.extend(self.agents.audit_agents(&context));
        findings.extend(self.architecture.audit_architecture(&context));
        findings.extend(self.backlog.audit_backlog(&context));
        findings.extend(self.contributing.audit_contributing(&context));
        findings.extend(self.data.audit_data(&context));
        findings.extend(self.design.audit_design(&context));
        findings.extend(self.frd.audit_frd(&context));
        findings.extend(self.prd.audit_prd(&context));
        findings.extend(self.readme.audit_readme(&context));
        findings.extend(self.roadmap.audit_roadmap(&context));

        DocResponse::Findings {
            findings: utility_doc_collector::sorted(findings),
        }
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ──────────────────────────

impl DocOrchestrator {
    pub fn new(
        agents: Arc<dyn IAgentsDocProtocol>,
        architecture: Arc<dyn IArchitectureDocProtocol>,
        backlog: Arc<dyn IBacklogDocProtocol>,
        contributing: Arc<dyn IContributingDocProtocol>,
        data: Arc<dyn IDataDocProtocol>,
        design: Arc<dyn IDesignDocProtocol>,
        frd: Arc<dyn IFrdDocProtocol>,
        prd: Arc<dyn IPrdDocProtocol>,
        readme: Arc<dyn IReadmeDocProtocol>,
        roadmap: Arc<dyn IRoadmapDocProtocol>,
    ) -> Self {
        Self {
            agents,
            architecture,
            backlog,
            contributing,
            data,
            design,
            frd,
            prd,
            readme,
            roadmap,
        }
    }
}
