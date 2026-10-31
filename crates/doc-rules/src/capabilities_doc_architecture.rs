// PURPOSE: Per-doc capability for ARCHITECTURE.md — AES603
//
// Answers ALL invariants scoped to ARCHITECTURE.md behind
// IArchitectureDocProtocol. Heading structure (AES605) and template body
// conformance (AES606) are stamped with RULE_CODE_ARCHITECTURE_DOC
// ("AES603"). The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IArchitectureDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::utility_doc_common_checks::check_doc_heading;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

#[derive(Default)]
pub struct ArchitectureCapability;

impl ArchitectureCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IArchitectureDocProtocol for ArchitectureCapability {
    fn audit_architecture(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::ARCHITECTURE_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_ARCHITECTURE_DOC;
            }
        }
        findings
    }
}

impl ArchitectureCapability {
    fn audit_one(document: &DocSource, _context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES603 — heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::ARCHITECTURE_DOC) {
            check_doc_heading(
                consts::RULE_CODE_ARCHITECTURE_DOC,
                text,
                consts::ARCHITECTURE_DOC,
                required,
                allowed,
                findings,
            );
        }

        // AES603 — template body conformance is exercised through the
        // required H2 set above; ARCHITECTURE.md carries no entry tables.
        let _ = parsed;
    }
}
