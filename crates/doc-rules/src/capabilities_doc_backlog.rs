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
use shared_doc_rules::utility_doc_aspect_entry_checks::{
    detect_aspect_entry_tables, AspectEntryDefect,
};
use shared_doc_rules::utility_doc_common_checks::{
    backlog_required_links, check_doc_heading, collect_union_id_set, missing_link_violation_type,
};
use shared_doc_rules::utility_doc_entry_regexes::backlog_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::collect_any_id_set;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

#[derive(Default)]
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
            for f in &mut findings {
                f.code = consts::RULE_CODE_BACKLOG_DOC;
            }
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
            check_doc_heading(
                consts::RULE_CODE_BACKLOG_DOC,
                text,
                consts::BACKLOG_DOC,
                required,
                allowed,
                findings,
            );
        }

        // AES606 — entry tables under Backlog Status.
        if let Some(re) = backlog_entry_re() {
            for defect in detect_aspect_entry_tables(
                &parsed,
                "Backlog Status",
                re,
                consts::BACKLOG_ROW_LABELS,
                consts::BACKLOG_ROW_LABELS,
                &[
                    ("State", 1),
                    ("Done", 1),
                    ("In Progress", 1),
                    ("Risk", 1),
                    ("Updated", 1),
                ],
                false,
            ) {
                findings.push(Self::convert_aspect_entry_defect(defect));
            }
        }
    }

    fn convert_aspect_entry_defect(defect: AspectEntryDefect) -> DocFinding {
        Self::convert_aspect_entry_defect_common(defect, consts::RULE_CODE_BACKLOG_DOC)
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
}
