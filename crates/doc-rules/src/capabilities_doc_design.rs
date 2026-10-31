// PURPOSE: Per-doc capability for DESIGN.md — AES610
//
// Answers ALL invariants scoped to DESIGN.md behind IDesignDocProtocol.
// DS-ID alignment with BACKLOG (AES601), crosslinks (AES604), heading
// structure (AES605), entry-table conformance + H3 sweep (AES606) are
// stamped with RULE_CODE_DESIGN_DOC ("AES610"). The capability stamps the
// document path on every finding.

use shared_doc_rules::contract_doc_protocol::IDesignDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::utility_doc_aspect_entry_checks::check_aspect_entry_tables;
use shared_doc_rules::utility_doc_common_checks::{
    check_doc_heading, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_doc_entry_regexes::ds_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::collect_any_id_set;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};
use shared_doc_rules::utility_protocol_counter::ds_id_heading_re;

pub struct DesignCapability;

impl DesignCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IDesignDocProtocol for DesignCapability {
    fn audit_design(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::DESIGN_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_DESIGN_DOC;
            }
        }
        findings
    }
}

impl DesignCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES601 — DS-ID alignment with BACKLOG.
        if let Some(backlog) = context
            .documents()
            .iter()
            .find(|d| context.document_name(d) == consts::BACKLOG_DOC)
        {
            let union = shared_doc_rules::utility_doc_common_checks::collect_union_id_set(
                collect_any_id_set,
                Some(document),
                Some(backlog),
            );
            shared_doc_rules::utility_doc_common_checks::check_backlog_qualifier_alignment(
                consts::RULE_CODE_DESIGN_DOC,
                backlog,
                union.as_ref(),
                consts::DS_BACKLOG_ID_MISSING,
                consts::DS_BACKLOG_ID_NOT_DS,
                findings,
            );
        }

        // AES604 — crosslinks (PRD, BACKLOG, ROADMAP, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::DESIGN_DOC) {
            check_doc_heading(
                consts::RULE_CODE_DESIGN_DOC,
                text,
                consts::DESIGN_DOC,
                required,
                allowed,
                findings,
            );
        }

        // AES606 — entry tables under Design Requirements + H3 sweep.
        if let Some(re) = ds_entry_re() {
            findings.extend(check_aspect_entry_tables(
                consts::RULE_CODE_DESIGN_DOC,
                &parsed,
                "Design Requirements",
                re,
                &[],
                &[],
                &[],
                false,
            ));
        }
        Self::check_design_h3s(text, findings);
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

    /// H3 sweep: every H3 under Design Requirements must match the DS-ID
    /// pattern; Design Overview and Reference sanction no H3.
    fn check_design_h3s(text: &str, findings: &mut Vec<DocFinding>) {
        let Some(re) = ds_id_heading_re() else {
            return;
        };
        let mut in_design_reqs = false;
        let mut in_design_overview = false;
        let mut in_reference = false;
        for line in text.lines() {
            let trimmed = line.trim();
            if let Some(title) = trimmed.strip_prefix("## ") {
                let norm = title.to_lowercase();
                in_design_reqs = norm == "design requirements";
                in_design_overview = norm == "design overview";
                in_reference = norm == "reference";
                continue;
            }
            if let Some(title) = trimmed.strip_prefix("### ") {
                if in_design_overview || in_reference {
                    findings.push(DocFinding::new_with_line(
                        "",
                        0,
                        consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                        consts::TEMPLATE_CONFORMANCE_VIOLATION_H3_UNEXPECTED,
                        format!("Design has an H3 `{title}` where the template sanctions none"),
                    ));
                    continue;
                }
                if in_design_reqs && !re.is_match(trimmed) {
                    findings.push(DocFinding::new_with_line(
                        "",
                        0,
                        consts::RULE_CODE_TEMPLATE_CONFORMANCE,
                        consts::TEMPLATE_CONFORMANCE_VIOLATION_H3_UNEXPECTED,
                        format!(
                            "Design Requirements H3 `{title}` does not match the DS-ID pattern"
                        ),
                    ));
                }
            }
        }
    }
}
