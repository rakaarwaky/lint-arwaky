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
use shared_doc_rules::taxonomy_doc_rules_violation_vo::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_doc_aspect_entry_checks::{
    AspectEntryDefect, detect_aspect_entry_tables,
};
use shared_doc_rules::utility_doc_common_checks::{
    detect_doc_heading, heading_defect_messages, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_doc_entry_regexes::dt_entry_re;
use shared_doc_rules::utility_doc_fr_format_checks::detect_fr_id_format;
use shared_doc_rules::utility_doc_line_drift::detect_doc_line_drift;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

// ─── Block 1: Module Declaration ───────────────────────────

pub struct DataCapability;

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

// ─── Block 2: Module Contract ─

impl IDataDocProtocol for DataCapability {
    fn audit_data(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::DATA_DOC {
                continue;
            }
            let mut document_findings = Vec::new();
            Self::audit_one(document, context, &mut document_findings);
            context.stamp(document, &mut document_findings);
            for f in &mut document_findings {
                f.code = consts::RULE_CODE_DATA_DOC;
            }
            findings.extend(document_findings);
        }
        findings
    }
}

// ─── Block 3: Module Helpers ───

impl DataCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // Template body conformance.
        let drift = detect_doc_line_drift(text, consts::DATA_TEMPLATE_LINES);
        findings.extend(
            shared_doc_rules::utility_doc_line_drift::line_drift_findings(
                drift,
                consts::DATA_DOC,
                consts::RULE_CODE_DATA_DOC,
            ),
        );

        // Bare-ID format (DT-<NAME>-NNN without qualified form).
        for defect in detect_fr_id_format(document) {
            findings.push(
                DocFinding::new_with_line(
                    "",
                    defect.line,
                    consts::RULE_CODE_DATA_DOC,
                    DOC_VIOLATION_NAMES.fr_id_missing_feature_prefix(),
                    format!(
                        "bare ID `{}: {}` without its `<FEATURE>` prefix",
                        defect.num, defect.title
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
                let (violation, message, reason, fix) =
                    heading_defect_messages(consts::DATA_DOC, &defect);
                let mut f = DocFinding::new_with_line(
                    "",
                    defect.line(),
                    consts::RULE_CODE_DATA_DOC,
                    violation,
                    message,
                )
                .with_reason(reason, fix);
                f.severity =
                    shared_doc_rules::taxonomy_doc_rules_violation_vo::severity_for_violation(
                        violation,
                    );
                findings.push(f);
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
                line: _,
                h3_title,
                aspect,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT,
                format!("`### {}` missing aspect H4 `#### {}`", h3_title, aspect),
                format!(
                    "The HOW-TO template requires every entry H3 under `### {}` to carry a `#### {}` subsection.",
                    h3_title, aspect
                ),
                format!("Add `#### {}` under `### {}`.", aspect, h3_title),
            ),
            AspectEntryDefect::OrderMismatch {
                line: _,
                h3_title,
                aspects,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_H4_ORDER,
                format!(
                    "`### {}` aspect H4s out of canonical order: expected {}",
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
                line: _,
                aspect,
                h3_title,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_MISSING_H4_ASPECT,
                format!(
                    "`#### {}` under `### {}` carries an empty body",
                    aspect, h3_title
                ),
                format!(
                    "Every PRD aspect `#### {}` under `### {}` must carry non-empty content.",
                    aspect, h3_title
                ),
                format!("Add content to `#### {}` under `### {}`.", aspect, h3_title),
            ),
            AspectEntryDefect::NotATable {
                line: _,
                aspect,
                h3_title,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE,
                format!(
                    "`#### {}` under `### {}` must be a markdown table",
                    aspect, h3_title
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
            AspectEntryDefect::HeaderMissing { line: _, aspect } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS,
                format!(
                    "`#### {}` aspect table header missing required columns",
                    aspect
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
                line: _,
                aspect,
                extras,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_EXTRA_COLUMNS,
                format!(
                    "`#### {}` aspect table carries extra column(s): {}",
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
                line: _,
                aspect,
                h3_title,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_NUMBERING,
                format!(
                    "`#### {}` under `### {}` row numbers are not contiguous 1..N",
                    aspect, h3_title
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
                line: _,
                aspect,
                h3_title,
                total,
                seed_min,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_COUNT,
                format!(
                    "`#### {}` under `### {}` bounded aspect carries {} rows, expected exactly {}",
                    aspect, h3_title, total, seed_min
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
                line: _,
                aspect,
                h3_title,
                total,
                min,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_ROW_MINIMUM,
                format!(
                    "`#### {}` under `### {}` open-ended aspect carries {} rows, need at least {}",
                    aspect, h3_title, total, min
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
                line: _,
                aspect,
                row_no,
            } => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_EMPTY_DETAIL_CELL,
                format!(
                    "`#### {}` row `{}` Detail cell is empty or a placeholder dash",
                    aspect, row_no
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
