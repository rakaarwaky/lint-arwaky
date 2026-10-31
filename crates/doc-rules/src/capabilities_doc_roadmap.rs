// PURPOSE: Per-doc capability for ROADMAP.md — AES606
//
// Answers ALL invariants scoped to ROADMAP.md behind IRoadmapDocProtocol.
// Crosslinks (AES604), heading structure (AES605), entry-table conformance
// (AES606) are stamped with RULE_CODE_ROADMAP_DOC ("AES606"); PR-ID parity
// carries RULE_CODE_FR_FORMAT ("AES601"). The capability stamps the document
// path on every finding.

use shared_doc_rules::contract_doc_protocol::IRoadmapDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::utility_doc_aspect_entry_checks::check_aspect_entry_tables;
use shared_doc_rules::utility_doc_common_checks::{
    check_doc_heading, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_doc_entry_regexes::pr_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::check_pr_id_parity;
use shared_doc_rules::utility_markdown_scanner::{
    doc_h2_contract, sections,
};

pub struct RoadmapCapability;

impl RoadmapCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IRoadmapDocProtocol for RoadmapCapability {
    fn audit_roadmap(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::ROADMAP_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
        }
        findings
    }
}

impl RoadmapCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES601 — PR-ID parity (PRD ↔ ROADMAP).
        if let Some(prd) = context.documents().iter().find(|d| context.document_name(d) == consts::PRD_DOC) {
            check_pr_id_parity(prd, document, findings);
        }

        // AES604 — crosslinks (PRD, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::ROADMAP_DOC) {
            check_doc_heading(text, consts::ROADMAP_DOC, required, allowed, findings);
        }

        // AES606 — entry tables under Roadmap Status.
        if let Some(re) = pr_entry_re() {
            findings.extend(check_aspect_entry_tables(
                &parsed,
                "Roadmap Status",
                re,
                consts::ROADMAP_ROW_LABELS,
                consts::ROADMAP_ROW_LABELS,
                &[("Priority", 1), ("State", 1), ("Backlog", 1)],
                false,
            ));
        }
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
