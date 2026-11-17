// PURPOSE: Per-doc capability for ARCHITECTURE.md — AES603
//
// Answers ALL invariants scoped to ARCHITECTURE.md behind
// IArchitectureDocProtocol. Heading structure and template body
// conformance are stamped with RULE_CODE_ARCHITECTURE_DOC
// ("AES603"). The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IArchitectureDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::utility_doc_common_checks::{detect_doc_heading, heading_defect_messages};
use shared_doc_rules::utility_doc_line_drift::detect_doc_line_drift;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

// ─── Block 1: Module Declaration ───────────────────────────

pub struct ArchitectureCapability;

impl Default for ArchitectureCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchitectureCapability {
    pub fn new() -> Self {
        Self
    }
}

// ─── Block 2: Module Contract ─

impl IArchitectureDocProtocol for ArchitectureCapability {
    fn audit_architecture(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::ARCHITECTURE_DOC {
                continue;
            }
            let mut document_findings = Vec::new();
            Self::audit_one(document, context, &mut document_findings);
            context.stamp(document, &mut document_findings);
            for f in &mut document_findings {
                f.code = consts::RULE_CODE_ARCHITECTURE_DOC;
            }
            findings.extend(document_findings);
        }
        findings
    }
}

// ─── Block 3: Module Helpers ───

impl ArchitectureCapability {
    fn audit_one(document: &DocSource, _context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // Template body conformance.
        let drift = detect_doc_line_drift(text, consts::ARCHITECTURE_TEMPLATE_LINES);
        findings.extend(
            shared_doc_rules::utility_doc_line_drift::line_drift_findings(
                drift,
                consts::ARCHITECTURE_DOC,
                consts::RULE_CODE_ARCHITECTURE_DOC,
            ),
        );

        // Heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::ARCHITECTURE_DOC) {
            for defect in detect_doc_heading(text, consts::ARCHITECTURE_DOC, required, allowed) {
                let (violation, message, reason, fix) =
                    heading_defect_messages(consts::ARCHITECTURE_DOC, &defect);
                let mut f = DocFinding::new_with_line(
                    "",
                    defect.line(),
                    consts::RULE_CODE_ARCHITECTURE_DOC,
                    violation,
                    message,
                )
                .with_reason(reason, fix);
                f.severity =
                    shared_doc_rules::taxonomy_doc_rules_violation_vo::severity_for_violation(
                        violation,
                    );
                findings.push(f);
            }
        }

        // Template body conformance is exercised through the
        // required H2 set above; ARCHITECTURE.md carries no entry tables.
        let _ = parsed;
    }
}
