// PURPOSE: Per-doc capability for DATA.md — AES608
//
// Answers ALL invariants scoped to DATA.md behind IDataDocProtocol.
// Bare-ID format, crosslinks, heading structure, and entry-table
// conformance are stamped with RULE_CODE_DATA_DOC ("AES608").
// The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IDataDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_names::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_doc_aspect_entry_checks::{
    AspectEntryDefect, detect_aspect_entry_tables,
};
use shared_doc_rules::utility_doc_common_checks::{
    HeadingDefect, detect_doc_heading, detect_doc_line_drift, missing_link_violation_type,
    required_links,
};
use shared_doc_rules::utility_doc_entry_regexes::dt_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::detect_fr_id_format;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DataCapability;

// ─── Block 2: Protocol Implementation ─────────────────────

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

// ─── Block 3: Inherent Implementation and Helpers ─────────

impl Default for DataCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl DataCapability {
    pub fn new() -> Self {
        Self
    }
}

impl DataCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // Template body conformance.
        let drift = detect_doc_line_drift(text, consts::DATA_TEMPLATE_LINES);
        findings.extend(shared_doc_rules::utility_doc_line_drift::line_drift_findings(
            drift,
            consts::DATA_DOC,
            consts::RULE_CODE_DATA_DOC,
        ));

        // Bare-ID format (DT-<NAME>-NNN without qualified form).
        for defect in detect_fr_id_format(document) {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    defect.line,
                    consts::RULE_CODE_DATA_DOC,
                    DOC_VIOLATION_NAMES.fr_id_missing_feature_prefix(),
                    format!(
                        "line {} bare ID `{}: {}` without its `<FEATURE>` prefix",
                        defect.line, defect.num, defect.title
                    ),
                )
                .with_reason(
                    "A DATA ID must carry its feature prefix so it stays unique across the doc chain.",
                    format!(
                        "Rename to `<FEATURENAME>-{}: {}` in `## Data Requirements`.",
                        defect.num, defect.title
                    ),
                ),
            );
        }

        // Crosslinks (PRD, BACKLOG, ROADMAP, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_DATA_DOC, findings);

        // Heading structure (DT-prefixed H2s allowed).
        if let Some((required, allowed)) = doc_h2_contract(consts::DATA_DOC) {
            for defect in detect_doc_heading(text, consts::DATA_DOC, required, allowed) {
                let (violation, message, reason, fix) = match &defect {
                    HeadingDefect::H1Count { line: _, count } => (
                        consts::DOC_STRUCTURE_VIOLATION_H1_COUNT,
                        format!("{} H1 headings; template requires exactly one", count),
                        format!(
                            "{} has {} H1 headings; the template fixes exactly one H1 at the top.",
                            consts::DATA_DOC,
                            count
                        ),
                        format!(
                            "Remove the extra H1 heading(s) so {} keeps exactly one H1.",
                            consts::DATA_DOC
                        ),
                    ),
                    HeadingDefect::H2Missing { line: _, missing } => (
                        consts::DOC_STRUCTURE_VIOLATION_H2_MISSING,
                        format!("missing required H2 section(s): {}", missing.join(", ")),
                        format!(
                            "{} is missing required H2 section(s): {}",
                            consts::DATA_DOC,
                            missing.join(", ")
                        ),
                        format!("Add the missing H2 section(s): {}", missing.join(", ")),
                    ),
                    HeadingDefect::H2Unexpected {
                        line: _,
                        unexpected,
                    } => (
                        consts::DOC_STRUCTURE_VIOLATION_H2_UNEXPECTED,
                        format!("unexpected H2 section(s): {}", unexpected.join(", ")),
                        format!(
                            "{} carries H2 section(s) outside the template: {}",
                            consts::DATA_DOC,
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
                        consts::RULE_CODE_DATA_DOC,
                        violation,
                        message,
                    )
                    .with_reason(reason, fix),
                );
            }
        }

        // Entry tables under Data Requirements.
        if let Some(re) = dt_entry_re() {
            for defect in
                detect_aspect_entry_tables(&parsed, "Data Requirements", re, &[], &[], &[], false)
            {
                findings.push(Self::convert_aspect_entry_defect(defect));
            }
        }
    }

    fn convert_aspect_entry_defect(defect: AspectEntryDefect) -> DocFinding {
        Self::convert_aspect_entry_defect_common(defect, consts::RULE_CODE_DATA_DOC)
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
                consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT,
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
                consts::TEMPLATE_CONFORMANCE_VIOLATION_H4_ORDER,
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
                consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT,
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
            AspectEntryDefect::HeaderExtra {
                line,
                aspect,
                extras,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_EXTRA_COLUMNS,
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
            AspectEntryDefect::RowCount {
                line,
                aspect,
                h3_title,
                total,
                seed_min,
            } => (
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
            AspectEntryDefect::RowMinimum {
                line,
                aspect,
                h3_title,
                total,
                min,
            } => (
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
            AspectEntryDefect::EmptyDetailCell {
                line,
                aspect,
                row_no,
            } => (
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
