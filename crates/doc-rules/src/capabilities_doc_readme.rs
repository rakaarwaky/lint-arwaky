// PURPOSE: Per-doc capability for README.md — AES602
//
// Answers ALL invariants scoped to README.md behind IReadmeDocProtocol.
// Crosslinks (none; placeholder table), heading structure (AES605),
// entry-table conformance (AES606) are stamped with RULE_CODE_README_DOC
// ("AES602"). The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IReadmeDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::utility_capabilities_doc_h2_checks::check_h2_table;
use shared_doc_rules::utility_doc_common_checks::check_doc_heading;
use shared_doc_rules::utility_markdown_scanner::{
    doc_h2_contract, sections,
};

pub struct ReadmeCapability;

impl ReadmeCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IReadmeDocProtocol for ReadmeCapability {
    fn audit_readme(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::README_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
        }
        findings
    }
}

impl ReadmeCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES602 — heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::README_DOC) {
            check_doc_heading(consts::RULE_CODE_README_DOC, text, consts::README_DOC, required, allowed, findings);
        }

        // AES606 — Quick Start table carries the fixed columns.
        findings.extend(check_h2_table(
            &parsed,
            "Quick Start",
            consts::README_QUICK_START_COLUMNS,
        ));
    }
}
