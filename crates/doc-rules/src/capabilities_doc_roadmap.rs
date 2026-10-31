// PURPOSE: Per-doc capability for ROADMAP.md — AES606
//
// Answers ALL invariants scoped to ROADMAP.md behind IRoadmapDocProtocol.
// PR-ID parity, crosslinks, heading structure, and entry-table
// conformance are stamped with RULE_CODE_ROADMAP_DOC ("AES606").
// The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IRoadmapDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_vo::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_doc_aspect_entry_checks::{
    AspectEntryDefect, detect_aspect_entry_tables,
};
use shared_doc_rules::utility_doc_common_checks::{
    HeadingDefect, detect_doc_heading, detect_doc_line_drift, missing_link_violation_type,
    required_links,
};
use shared_doc_rules::utility_doc_entry_regexes::pr_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::{PrIdParityDefect, detect_pr_id_parity};
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct RoadmapCapability;

// ─── Block 2: Protocol Implementation ─────────────────────

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

// ─── Block 3: Inherent Implementation and Helpers ─────────

impl Default for RoadmapCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl RoadmapCapability {
    pub fn new() -> Self {
        Self
    }
}

impl RoadmapCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // Template body conformance.
        let drift = detect_doc_line_drift(text, consts::ROADMAP_TEMPLATE_LINES);
        findings.extend(shared_doc_rules::utility_doc_line_drift::line_drift_findings(
            drift,
            consts::ROADMAP_DOC,
            consts::RULE_CODE_ROADMAP_DOC,
        ));

        // PR-ID parity (PRD ↔ ROADMAP).
        if let Some(prd) = context
            .documents()
            .iter()
            .find(|d| context.document_name(d) == consts::PRD_DOC)
        {
            for defect in detect_pr_id_parity(prd, document) {
                let (violation, message, reason, fix) = match &defect {
                    PrIdParityDefect::PrMissingFromRoadmap { line, id } => (
                        DOC_VIOLATION_NAMES.pr_roadmap_id_missing(),
                        format!(
                            "line {} PR `{}` declared in PRD has no ROADMAP `Roadmap Status` entry",
                            line, id
                        ),
                        format!("Add `{}` to ROADMAP's `Roadmap Status` table.", id),
                        format!("Add `{}` under `Roadmap Status` in ROADMAP.md.", id),
                    ),
                    PrIdParityDefect::RoadmapNotInPrd { line, id, valid } => (
                        DOC_VIOLATION_NAMES.roadmap_pr_id_not_pr(),
                        format!(
                            "line {} ROADMAP `PR-` item `{}` is not declared in the PRD",
                            line, id
                        ),
                        format!(
                            "The PRD declares `Project Requirements`; `{}` must appear there first.",
                            id
                        ),
                        format!(
                            "Declare `{}` in PRD's `Project Requirements`. Valid ids: {}",
                            id,
                            valid.join(", ")
                        ),
                    ),
                };
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        defect.line(),
                        consts::RULE_CODE_ROADMAP_DOC,
                        violation,
                        message,
                    )
                    .with_reason(reason, fix),
                );
            }
        }

        // Crosslinks (PRD, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_ROADMAP_DOC, findings);

        // Heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::ROADMAP_DOC) {
            for defect in detect_doc_heading(text, consts::ROADMAP_DOC, required, allowed) {
                let (violation, message, reason, fix) = match &defect {
                    HeadingDefect::H1Count { line: _, count } => (
                        DOC_VIOLATION_NAMES.h1_count_violation(),
                        format!("{} H1 headings; template requires exactly one", count),
                        format!(
                            "{} has {} H1 headings; the template fixes exactly one H1 at the top.",
                            consts::ROADMAP_DOC,
                            count
                        ),
                        format!(
                            "Remove the extra H1 heading(s) so {} keeps exactly one H1.",
                            consts::ROADMAP_DOC
                        ),
                    ),
                    HeadingDefect::H2Missing { line: _, missing } => (
                        DOC_VIOLATION_NAMES.h2_missing(),
                        format!("missing required H2 section(s): {}", missing.join(", ")),
                        format!(
                            "{} is missing required H2 section(s): {}",
                            consts::ROADMAP_DOC,
                            missing.join(", ")
                        ),
                        format!("Add the missing H2 section(s): {}", missing.join(", ")),
                    ),
                    HeadingDefect::H2Unexpected {
                        line: _,
                        unexpected,
                    } => (
                        DOC_VIOLATION_NAMES.h2_unexpected(),
                        format!("unexpected H2 section(s): {}", unexpected.join(", ")),
                        format!(
                            "{} carries H2 section(s) outside the template: {}",
                            consts::ROADMAP_DOC,
                            unexpected.join(", ")
                        ),
                        format!(
                            "Remove the off-template H2 section(s): {}",
                            unexpected.join(", ")
                        ),
                    ),
                };
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        defect.line(),
                        consts::RULE_CODE_ROADMAP_DOC,
                        violation,
                        message,
                    )
                    .with_reason(reason, fix),
                );
            }
        }

        // Entry tables under Roadmap Status.
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

    fn convert_aspect_entry_defect_common(
        defect: AspectEntryDefect,
        rule_code: &'static str,
    ) -> DocFinding {
        let (violation, message, reason, fix) = match &defect {
            AspectEntryDefect::MissingAspect {
                line,
                h3_title,
                aspect,
            } => (
                DOC_VIOLATION_NAMES.missing_h4_aspect(),
                format!(
                    "line {} `### {}` missing aspect H4 `#### {}`",
                    line, h3_title, aspect
                ),
                format!(
                    "The HOW-TO template requires every entry H3 under `### {}` to carry a `#### {}` subsection.",
                    h3_title, aspect
                ),
                format!("Add `#### {}` under `### {}`.", aspect, h3_title),
            ),
            AspectEntryDefect::OrderMismatch {
                line,
                h3_title,
                aspects,
            } => (
                DOC_VIOLATION_NAMES.h4_order_violation(),
                format!(
                    "line {} `### {}` aspect H4s out of canonical order: expected {}",
                    line,
                    h3_title,
                    aspects.join(", ")
                ),
                format!(
                    "Reorder the `####` aspect subsections under `### {}` to match the template order: {}.",
                    h3_title,
                    aspects.join(", ")
                ),
                format!(
                    "Move the aspect H4s under `### {}` into canonical order: {}.",
                    h3_title,
                    aspects.join(", ")
                ),
            ),
            AspectEntryDefect::EmptyBody {
                line,
                aspect,
                h3_title,
            } => (
                DOC_VIOLATION_NAMES.missing_h4_aspect(),
                format!(
                    "line {} `#### {}` under `### {}` carries an empty body",
                    line, aspect, h3_title
                ),
                format!(
                    "Every PRD aspect `#### {}` under `### {}` must carry non-empty content.",
                    aspect, h3_title
                ),
                format!("Add content to `#### {}` under `### {}`.", aspect, h3_title),
            ),
            AspectEntryDefect::NotATable {
                line,
                aspect,
                h3_title,
            } => (
                DOC_VIOLATION_NAMES.section_not_table(),
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
                DOC_VIOLATION_NAMES.table_missing_columns(),
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
            AspectEntryDefect::HeaderExtra {
                line,
                aspect,
                extras,
            } => (
                DOC_VIOLATION_NAMES.table_extra_columns(),
                format!(
                    "line {} `#### {}` aspect table carries extra column(s): {}",
                    line,
                    aspect,
                    extras.join(", ")
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
            AspectEntryDefect::RowNumbering {
                line,
                aspect,
                h3_title,
            } => (
                DOC_VIOLATION_NAMES.row_numbering_violation(),
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
            AspectEntryDefect::RowCount {
                line,
                aspect,
                h3_title,
                total,
                seed_min,
            } => (
                DOC_VIOLATION_NAMES.row_count_violation(),
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
            AspectEntryDefect::RowMinimum {
                line,
                aspect,
                h3_title,
                total,
                min,
            } => (
                DOC_VIOLATION_NAMES.row_minimum_violation(),
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
            AspectEntryDefect::EmptyDetailCell {
                line,
                aspect,
                row_no,
            } => (
                DOC_VIOLATION_NAMES.empty_detail_cell(),
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
