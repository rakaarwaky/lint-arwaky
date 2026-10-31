// PURPOSE: Per-doc capability for README.md — AES602
//
// Answers ALL invariants scoped to README.md behind IReadmeDocProtocol.
// Heading structure and entry-table conformance are stamped with
// RULE_CODE_README_DOC ("AES602"). The capability stamps the document
// path on every finding.

use shared_doc_rules::contract_doc_protocol::IReadmeDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_vo::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_capabilities_doc_h2_checks::{
    SectionShapeDefect, SectionShapeDefectKind, detect_h2_table,
};
use shared_doc_rules::utility_doc_common_checks::{detect_doc_heading, detect_doc_line_drift};
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ReadmeCapability;

// ─── Block 2: Protocol Implementation ─────────────────────

impl IReadmeDocProtocol for ReadmeCapability {
    fn audit_readme(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::README_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_README_DOC;
            }
        }
        findings
    }
}

// ─── Block 3: Inherent Implementation and Helpers ─────────

impl Default for ReadmeCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadmeCapability {
    pub fn new() -> Self {
        Self
    }
}

impl ReadmeCapability {
    fn audit_one(document: &DocSource, _context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // Template body conformance.
        let drift = detect_doc_line_drift(text, consts::README_TEMPLATE_LINES);
        findings.extend(shared_doc_rules::utility_doc_line_drift::line_drift_findings(
            drift,
            consts::README_DOC,
            consts::RULE_CODE_README_DOC,
        ));

        // Heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::README_DOC) {
            for defect in detect_doc_heading(text, consts::README_DOC, required, allowed) {
                let (violation, message, reason, fix) = match &defect {
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H1Count {
                        line: _,
                        count,
                    } => (
                        DOC_VIOLATION_NAMES.h1_count_violation(),
                        format!(
                            "{} opens with {} level-1 headings",
                            consts::README_DOC,
                            count
                        ),
                        format!(
                            "{} must open with exactly one H1 naming the project",
                            consts::README_DOC
                        ),
                        format!(
                            "Keep exactly one `# {}` line at the top of {}",
                            consts::README_DOC,
                            consts::README_DOC
                        ),
                    ),
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H2Missing {
                        line: _,
                        missing,
                    } => (
                        DOC_VIOLATION_NAMES.h2_missing(),
                        format!(
                            "{} has no H2 heading for {}",
                            consts::README_DOC,
                            missing.join(", ")
                        ),
                        format!(
                            "{} needs a level-2 heading for every template-required section",
                            consts::README_DOC
                        ),
                        format!(
                            "Add the missing `## {}` section(s) to {}",
                            missing.join(" `## `"),
                            consts::README_DOC
                        ),
                    ),
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H2Unexpected {
                        line: _,
                        unexpected,
                    } => (
                        DOC_VIOLATION_NAMES.h2_unexpected(),
                        format!(
                            "{} carries H2 heading(s) outside the template: {}",
                            consts::README_DOC,
                            unexpected.join(", ")
                        ),
                        format!(
                            "{} must not carry H2 headings the template does not declare",
                            consts::README_DOC
                        ),
                        format!(
                            "Remove or rename the unexpected heading(s): {}",
                            unexpected.join(", ")
                        ),
                    ),
                };
                findings.push(
                    DocFinding::new_with_line(
                        "",
                        defect.line(),
                        consts::RULE_CODE_README_DOC,
                        violation,
                        message,
                    )
                    .with_reason(reason, fix),
                );
            }
        }

        // Quick Start table carries the fixed columns.
        if let Some(defect) =
            detect_h2_table(&parsed, "Quick Start", consts::README_QUICK_START_COLUMNS)
        {
            findings.push(Self::convert_shape_defect(defect));
        }
    }

    fn convert_shape_defect(defect: SectionShapeDefect) -> DocFinding {
        let (violation, message, reason, fix) = match defect.kind {
            SectionShapeDefectKind::ExtraColumns => (
                DOC_VIOLATION_NAMES.table_extra_columns(),
                format!(
                    "line {} `{} {}` table carries column(s) outside the template: {}",
                    defect.line,
                    defect.marker,
                    defect.title,
                    defect.extra_columns.join(", ")
                ),
                format!(
                    "The HOW-TO template fixes the header of `{}` to exactly {}; an undeclared column breaks the fixed shape.",
                    defect.title,
                    defect.columns.join(" | ")
                ),
                format!(
                    "Remove the extra column(s) ({}) from `{} {}` or fold the data into an existing column.",
                    defect.extra_columns.join(", "),
                    defect.marker,
                    defect.title,
                ),
            ),
            SectionShapeDefectKind::HeaderRepeated => (
                DOC_VIOLATION_NAMES.table_header_repeated(),
                format!(
                    "line {} `{} {}` repeats its header row as a data row",
                    defect.line, defect.marker, defect.title
                ),
                format!(
                    "The HOW-TO template mandates one header row under `{}`; repeating it in a data-row position ships the column names where filled values belong.",
                    defect.title
                ),
                format!(
                    "Replace the duplicated `| {} |` row under `{} {}` with real values, or delete it if no values exist yet.",
                    defect.columns.join(" | "),
                    defect.marker,
                    defect.title
                ),
            ),
            SectionShapeDefectKind::MissingColumns => (
                DOC_VIOLATION_NAMES.table_missing_columns(),
                format!(
                    "line {} `{} {}` must be a markdown table with columns {}",
                    defect.line,
                    defect.marker,
                    defect.title,
                    defect.columns.join(" | ")
                ),
                format!(
                    "The HOW-TO template mandates `{}` under this section; a body carrying bullets, prose, or a table missing required columns is not a report a reader can verify.",
                    defect.title
                ),
                format!(
                    "Replace the body of `{} {}` with a table carrying columns {}.",
                    defect.marker,
                    defect.title,
                    defect.columns.join(" | ")
                ),
            ),
            SectionShapeDefectKind::NotTable => (
                DOC_VIOLATION_NAMES.section_not_table(),
                format!(
                    "line {} `{} {}` must be a markdown table with columns {}",
                    defect.line,
                    defect.marker,
                    defect.title,
                    defect.columns.join(" | ")
                ),
                format!(
                    "The HOW-TO template mandates `{}` under this section; a body carrying bullets, prose, or a table missing required columns is not a report a reader can verify.",
                    defect.title
                ),
                format!(
                    "Replace the body of `{} {}` with a table carrying columns {}.",
                    defect.marker,
                    defect.title,
                    defect.columns.join(" | ")
                ),
            ),
        };
        DocFinding::new_with_line(
            "",
            defect.line,
            consts::RULE_CODE_README_DOC,
            violation,
            message,
        )
        .with_reason(reason, fix)
    }
}
