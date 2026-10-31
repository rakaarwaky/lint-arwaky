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
use shared_doc_rules::utility_doc_aspect_entry_checks::{
    detect_aspect_entry_tables, AspectEntryDefect,
};
use shared_doc_rules::utility_doc_common_checks::{
    check_doc_heading, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_doc_entry_regexes::pr_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::check_pr_id_parity;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

#[derive(Default)]
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
            for f in &mut findings {
                f.code = consts::RULE_CODE_ROADMAP_DOC;
            }
        }
        findings
    }
}

impl RoadmapCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES601 — PR-ID parity (PRD ↔ ROADMAP).
        if let Some(prd) = context
            .documents()
            .iter()
            .find(|d| context.document_name(d) == consts::PRD_DOC)
        {
            check_pr_id_parity(consts::RULE_CODE_FR_FORMAT, consts::PR_ROADMAP_ID_MISSING, consts::ROADMAP_PR_ID_NOT_PR, prd, document, findings);
        }

        // AES604 — crosslinks (PRD, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::ROADMAP_DOC) {
            check_doc_heading(
                consts::RULE_CODE_ROADMAP_DOC,
                text,
                consts::ROADMAP_DOC,
                required,
                allowed,
                findings,
            );
        }

        // AES606 — entry tables under Roadmap Status.
        if let Some(re) = pr_entry_re() {
            for defect in detect_aspect_entry_tables(
                &parsed,
                "Roadmap Status",
                re,
                consts::ROADMAP_ROW_LABELS,
                consts::ROADMAP_ROW_LABELS,
                &[("Priority", 1), ("State", 1), ("Backlog", 1)],
                false,
            ) {
                findings.push(Self::convert_aspect_entry_defect(defect));
            }
        }
    }

    fn convert_aspect_entry_defect(defect: AspectEntryDefect) -> DocFinding {
        Self::convert_aspect_entry_defect_common(defect, consts::RULE_CODE_ROADMAP_DOC)
    }

    fn convert_aspect_entry_defect_common(defect: AspectEntryDefect, rule_code: &'static str) -> DocFinding {
        let (violation, message, reason, fix) = match &defect {
            AspectEntryDefect::MissingAspect { line, h3_title, aspect } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT,
                format!(
                    "line {} `### {}` missing aspect H4 `#### {}`",
                    line, h3_title, aspect
                ),
                format!(
                    "The HOW-TO template requires every entry H3 under `### {}` to carry a `#### {}` subsection.",
                    h3_title, aspect
                ),
                format!(
                    "Add `#### {}` under `### {}`.",
                    aspect, h3_title
                ),
            ),
            AspectEntryDefect::OrderMismatch { line, h3_title, aspects } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_H4_ORDER,
                format!(
                    "line {} `### {}` aspect H4s out of canonical order: expected {}",
                    line, h3_title, aspects.join(", ")
                ),
                format!(
                    "Reorder the `####` aspect subsections under `### {}` to match the template order: {}.",
                    h3_title, aspects.join(", ")
                ),
                format!(
                    "Move the aspect H4s under `### {}` into canonical order: {}.",
                    h3_title, aspects.join(", ")
                ),
            ),
            AspectEntryDefect::EmptyBody { line, aspect, h3_title } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT,
                format!(
                    "line {} `#### {}` under `### {}` carries an empty body",
                    line, aspect, h3_title
                ),
                format!(
                    "Every PRD aspect `#### {}` under `### {}` must carry non-empty content.",
                    aspect, h3_title
                ),
                format!(
                    "Add content to `#### {}` under `### {}`.",
                    aspect, h3_title
                ),
            ),
            AspectEntryDefect::NotATable { line, aspect, h3_title } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE,
                format!(
                    "line {} `#### {}` under `### {}` must be a markdown table",
                    line, aspect, h3_title
                ),
                format!(
                    "The aspect `#### {}` under `### {}` must carry a `| No | Detail |` table.",
                    aspect, h3_title
                ),
                format!(
                    "Replace the body of `#### {}` under `### {}` with a `| No | Detail |` table.",
                    aspect, h3_title
                ),
            ),
            AspectEntryDefect::HeaderMissing { line, aspect } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS,
                format!(
                    "line {} `#### {}` aspect table header missing required columns",
                    line, aspect
                ),
                format!(
                    "The aspect table must carry columns `| No | Detail |` under `#### {}`; a header missing a required column cannot be parsed.",
                    aspect
                ),
                format!(
                    "Set the aspect table header to `| No | Detail |` under `#### {}`.",
                    aspect
                ),
            ),
            AspectEntryDefect::HeaderExtra { line, aspect, extras } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_EXTRA_COLUMNS,
                format!(
                    "line {} `#### {}` aspect table carries extra column(s): {}",
                    line, aspect, extras.join(", ")
                ),
                format!(
                    "The aspect table header under `#### {}` must be exactly `| No | Detail |`; extra columns break the fixed shape.",
                    aspect
                ),
                format!(
                    "Remove the extra column(s) ({}) from the aspect table header under `#### {}`.",
                    extras.join(", "),
                    aspect
                ),
            ),
            AspectEntryDefect::RowNumbering { line, aspect, h3_title } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_NUMBERING,
                format!(
                    "line {} `#### {}` under `### {}` row numbers are not contiguous 1..N",
                    line, aspect, h3_title
                ),
                format!(
                    "The aspect table rows under `#### {}` must be numbered contiguously starting at 1.",
                    aspect
                ),
                format!(
                    "Renumber the rows of `#### {}` under `### {}` as 1, 2, ... N with no gaps.",
                    aspect, h3_title
                ),
            ),
            AspectEntryDefect::RowCount { line, aspect, h3_title, total, seed_min } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_COUNT,
                format!(
                    "line {} `#### {}` under `### {}` bounded aspect carries {} rows, expected exactly {}",
                    line, aspect, h3_title, total, seed_min
                ),
                format!(
                    "The bounded aspect `#### {}` under `### {}` must carry exactly {} numbered rows.",
                    aspect, h3_title, seed_min
                ),
                format!(
                    "Adjust `#### {}` under `### {}` to carry exactly {} numbered rows.",
                    aspect, h3_title, seed_min
                ),
            ),
            AspectEntryDefect::RowMinimum { line, aspect, h3_title, total, min } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_MINIMUM,
                format!(
                    "line {} `#### {}` under `### {}` open-ended aspect carries {} rows, need at least {}",
                    line, aspect, h3_title, total, min
                ),
                format!(
                    "The open-ended aspect `#### {}` under `### {}` must carry at least {} numbered rows.",
                    aspect, h3_title, min
                ),
                format!(
                    "Add numbered rows to `#### {}` under `### {}` until at least {} are present.",
                    aspect, h3_title, min
                ),
            ),
            AspectEntryDefect::EmptyDetailCell { line, aspect, row_no } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_EMPTY_DETAIL_CELL,
                format!(
                    "line {} `#### {}` row `{}` Detail cell is empty or a placeholder dash",
                    line, aspect, row_no
                ),
                format!(
                    "Every numbered row in `#### {}` must carry real content in its Detail cell; a lone dash cell carries nothing.",
                    aspect
                ),
                format!(
                    "Fill the Detail cell of row `{}` in `#### {}` with real content, or delete the row if no value exists yet.",
                    row_no, aspect
                ),
            ),
        };
        DocFinding::new_with_line("", defect.line(), rule_code, violation, message)
            .with_reason(reason, fix)
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
