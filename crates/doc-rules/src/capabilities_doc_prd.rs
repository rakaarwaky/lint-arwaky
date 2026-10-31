// PURPOSE: Per-doc capability for PRD.md — AES605
//
// Answers ALL invariants scoped to PRD.md behind IPrdDocProtocol.
// Crosslinks (AES604), heading structure (AES605), entry-table conformance
// (AES606) are stamped with RULE_CODE_PRD_DOC ("AES605"); FR-format family
// violations (PR-ID parity, API method existence) carry their own AES601
// code. The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IPrdDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::utility_doc_aspect_entry_checks::check_aspect_entry_tables;
use shared_doc_rules::utility_doc_common_checks::{
    check_doc_heading, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_doc_entry_regexes::pr_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::{
    check_api_method_existence, check_pr_id_parity,
};
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

#[derive(Default)]
pub struct PrdCapability;

impl PrdCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IPrdDocProtocol for PrdCapability {
    fn audit_prd(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::PRD_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_PRD_DOC;
            }
        }
        findings
    }
}

impl PrdCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES605 — spec purity: PRD names no source file and carries no status.
        // (Implemented via shared checks below where a root master exists.)
        // PRD carries no bare-FR-ID shape; FR-ID format applies to FRD only.

        // AES601 — PR-ID parity (PRD ↔ ROADMAP).
        if let Some(roadmap) = context
            .documents()
            .iter()
            .find(|d| context.document_name(d) == consts::ROADMAP_DOC)
        {
            check_pr_id_parity(consts::RULE_CODE_FR_FORMAT, consts::PR_ROADMAP_ID_MISSING, consts::ROADMAP_PR_ID_NOT_PR, document, roadmap, findings);
        }

        // AES601 — promised API method existence.
        check_api_method_existence(consts::RULE_CODE_FR_FORMAT, consts::FR_API_METHOD_VIOLATION_NOT_FOUND, document, context, findings);

        // AES604 — crosslinks (ROADMAP, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::PRD_DOC) {
            check_doc_heading(
                consts::RULE_CODE_PRD_DOC,
                text,
                consts::PRD_DOC,
                required,
                allowed,
                findings,
            );
        }

        // AES606 — entry tables under Project Requirements.
        if let Some(re) = pr_entry_re() {
            findings.extend(check_aspect_entry_tables(
                consts::RULE_CODE_PRD_DOC,
                &parsed,
                "Project Requirements",
                re,
                consts::PRD_ASPECTS,
                consts::PRD_ASPECTS_OPEN,
                consts::PRD_ASPECT_SEED_MIN,
                true,
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
