// PURPOSE: Per-doc capability for BACKLOG.md — AES607
//
// Answers ALL invariants scoped to BACKLOG.md behind IBacklogDocProtocol.
// BACKLOG ID-level alignment against its paired spec union (AES601),
// role-aware crosslinks (AES604), heading structure (AES605), and
// entry-table conformance (AES606) are stamped with RULE_CODE_BACKLOG_DOC
// ("AES607"). The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IBacklogDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::utility_doc_aspect_entry_checks::check_aspect_entry_tables;
use shared_doc_rules::utility_doc_common_checks::{
    backlog_required_links, check_doc_heading, collect_union_id_set, missing_link_violation_type,
};
use shared_doc_rules::utility_doc_entry_regexes::{backlog_entry_re, fr_entry_re};
use shared_doc_rules::utility_doc_fr_format_checks::collect_any_id_set;
use shared_doc_rules::utility_markdown_scanner::{
    doc_h2_contract, sections,
};

pub struct BacklogCapability;

impl BacklogCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IBacklogDocProtocol for BacklogCapability {
    fn audit_backlog(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::BACKLOG_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
        }
        findings
    }
}

impl BacklogCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES601 — BACKLOG ID-level alignment against its paired spec union.
        Self::check_backlog_alignment(document, context, findings);

        // AES604 — role-aware crosslinks.
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::BACKLOG_DOC) {
            check_doc_heading(consts::RULE_CODE_BACKLOG_DOC, text, consts::BACKLOG_DOC, required, allowed, findings);
        }

        // AES606 — entry tables under Backlog Status.
        if let Some(re) = backlog_entry_re() {
            findings.extend(check_aspect_entry_tables(
                &parsed,
                "Backlog Status",
                re,
                consts::BACKLOG_ROW_LABELS,
                consts::BACKLOG_ROW_LABELS,
                &[("State", 1), ("Done", 1), ("In Progress", 1), ("Risk", 1), ("Updated", 1)],
                false,
            ));
        }
    }

    /// BACKLOG ID-level alignment: entry H3 tokens must match declared IDs of
    /// the paired spec union, and vice versa.
    fn check_backlog_alignment(
        document: &DocSource,
        context: &DocAuditContext,
        findings: &mut Vec<DocFinding>,
    ) {
        // Build the union from any FRD/DATA/DESIGN present in the same
        // folder; fall back to root PRD/ROADMAP.
        let union = collect_union_id_set(
            collect_any_id_set,
            context.documents().iter().find(|d| {
                context.document_name(d) == consts::FRD_DOC
                    || context.document_name(d) == consts::DATA_DOC
                    || context.document_name(d) == consts::DESIGN_DOC
            }),
            context.documents().iter().find(|d| {
                context.document_name(d) == consts::PRD_DOC
                    || context.document_name(d) == consts::ROADMAP_DOC
            }),
        );

        let missing_code = if context.is_root_document(document) {
            consts::PR_BACKLOG_ID_MISSING
        } else {
            consts::BACKLOG_QUALIFIER_MISMATCH
        };
        let not_code = if context.is_root_document(document) {
            consts::BACKLOG_ID_NOT_PR
        } else {
            consts::DT_BACKLOG_ID_NOT_DT
        };

        shared_doc_rules::utility_doc_common_checks::check_backlog_qualifier_alignment(
            consts::RULE_CODE_BACKLOG_DOC,
            document,
            union.as_ref(),
            missing_code,
            not_code,
            findings,
        );
    }

    /// Crosslink check: every required link target must appear as a link in
    /// the document, using role-aware link sets.
    fn check_crosslinks(
        document: &DocSource,
        context: &DocAuditContext,
        code: &'static str,
        findings: &mut Vec<DocFinding>,
    ) {
        let name = context.document_name(document);
        let links = backlog_required_links(name, document, context);
        for &link in &links {
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

    /// FR entry tables under Backlog Status (legacy format support).
    fn check_legacy_fr_entry_tables(
        document: &DocSource,
        findings: &mut Vec<DocFinding>,
    ) {
        if let Some(re) = fr_entry_re() {
            let parsed = sections(&document.text);
            findings.extend(
                shared_doc_rules::utility_doc_common_checks::check_entry_tables(
                    &parsed,
                    "Backlog Status",
                    re,
                    consts::ASPECT_DETAIL_COLUMNS,
                    consts::FRD_ROW_LABELS,
                ),
            );
        }
    }
}
