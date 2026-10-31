// PURPOSE: Per-doc capability for DATA.md — AES608
//
// Answers ALL invariants scoped to DATA.md behind IDataDocProtocol.
// Crosslinks (AES604), heading structure (AES605), entry-table conformance
// (AES606) are stamped with RULE_CODE_DATA_DOC ("AES608"). FR-ID bare-ID
// format carries RULE_CODE_FR_FORMAT ("AES601"). The capability stamps the
// document path on every finding.

use shared_doc_rules::contract_doc_protocol::IDataDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::utility_doc_aspect_entry_checks::check_aspect_entry_tables;
use shared_doc_rules::utility_doc_common_checks::{
    check_doc_heading, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_doc_entry_regexes::dt_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::check_fr_id_format;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

#[derive(Default)]
pub struct DataCapability;

impl DataCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IDataDocProtocol for DataCapability {
    fn audit_data(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::DATA_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_DATA_DOC;
            }
        }
        findings
    }
}

impl DataCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES608 — bare-ID format (DT-<NAME>-NNN without qualified form).
        check_fr_id_format(consts::RULE_CODE_DATA_DOC, consts::FR_ID_VIOLATION_MISSING_FEATURE_PREFIX, document, findings);

        // AES604 — crosslinks (PRD, BACKLOG, ROADMAP, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure (DT-prefixed H2s allowed).
        if let Some((required, allowed)) = doc_h2_contract(consts::DATA_DOC) {
            check_doc_heading(
                consts::RULE_CODE_DATA_DOC,
                text,
                consts::DATA_DOC,
                required,
                allowed,
                findings,
            );
        }

        // AES606 — entry tables under Data Requirements.
        if let Some(re) = dt_entry_re() {
            findings.extend(check_aspect_entry_tables(
                consts::RULE_CODE_DATA_DOC,
                &parsed,
                "Data Requirements",
                re,
                &[],
                &[],
                &[],
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
