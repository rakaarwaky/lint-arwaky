// PURPOSE: Per-doc capability for CONTRIBUTING.md — AES604
//
// Answers ALL invariants scoped to CONTRIBUTING.md behind
// IContributingDocProtocol. Crosslinks (PRD, ARCHITECTURE), heading
// structure (AES605), and entry-table conformance (AES606) are stamped with
// RULE_CODE_CONTRIBUTING_DOC ("AES604"). The capability stamps the document
// path on every finding.

use shared_doc_rules::contract_doc_protocol::IContributingDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::utility_capabilities_doc_h2_checks::check_h2_table;
use shared_doc_rules::utility_doc_common_checks::{
    check_doc_heading, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

#[derive(Default)]
pub struct ContributingCapability;

impl ContributingCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IContributingDocProtocol for ContributingCapability {
    fn audit_contributing(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::CONTRIBUTING_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_CONTRIBUTING_DOC;
            }
        }
        findings
    }
}

impl ContributingCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES604 — crosslinks (PRD, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::CONTRIBUTING_DOC) {
            check_doc_heading(
                consts::RULE_CODE_CONTRIBUTING_DOC,
                text,
                consts::CONTRIBUTING_DOC,
                required,
                allowed,
                findings,
            );
        }

        // AES604 — Workflow table carries fixed columns.
        findings.extend(check_h2_table(
            consts::RULE_CODE_CONTRIBUTING_DOC,
            &parsed,
            "Workflow",
            consts::CONTRIBUTING_WORKFLOW_COLUMNS,
            &shared_doc_rules::taxonomy_doc_rules_violation_names::DocViolationNames,
        ));
        // AES604 — Commit Convention table carries fixed columns.
        findings.extend(check_h2_table(
            consts::RULE_CODE_CONTRIBUTING_DOC,
            &parsed,
            "Commit Convention",
            consts::CONTRIBUTING_COMMIT_COLUMNS,
            &shared_doc_rules::taxonomy_doc_rules_violation_names::DocViolationNames,
        ));
    }

    /// Crosslink check: every required link target must appear as a link in
    /// the document.
    fn check_crosslinks(
        document: &DocSource,
        context: &DocAuditContext,
        code: &'static str,
        findings: &mut Vec<DocFinding>,
    ) {
        let name = context.document_name(document);
        for &link in required_links(name) {
            if !document.text.contains(&format!("[{link}")) {
                let violation_type = missing_link_violation_type(link);
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        0,
                        code,
                        violation_type,
                        format!("{name} does not link its required {link}"),
                    )
                    .with_reason(
                        "Every document must cross-link its required artifacts so the chain stays navigable.",
                        format!("Add a link to {link} in the Reference section."),
                    ),
                );
            }
        }
    }
}
