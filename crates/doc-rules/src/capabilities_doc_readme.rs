// PURPOSE: Per-doc capability for README.md — AES602
//
// Answers ALL invariants scoped to README.md behind IReadmeDocProtocol.
// Crosslinks (none; placeholder table), heading structure (AES605),
// entry-table conformance (AES606) are stamped with RULE_CODE_README_DOC
// ("AES602"). The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IReadmeDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_names::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_capabilities_doc_h2_checks::{
    detect_h2_table, SectionShapeDefect, SectionShapeDefectKind,
};
use shared_doc_rules::utility_doc_common_checks::check_doc_heading;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

#[derive(Default)]
pub struct ReadmeCapability;

impl ReadmeCapability {
    pub fn new() -> Self {
        Self
    }
}

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

impl ReadmeCapability {
    fn audit_one(document: &DocSource, _context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES602 — heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::README_DOC) {
            check_doc_heading(
                consts::RULE_CODE_README_DOC,
                text,
                consts::README_DOC,
                required,
                allowed,
                &DOC_VIOLATION_NAMES,
                findings,
            );
        }

        // AES602 — Quick Start table carries the fixed columns.
        if let Some(defect) = detect_h2_table(&parsed, "Quick Start", consts::README_QUICK_START_COLUMNS) {
            findings.push(Self::convert_shape_defect(defect));
        }
    }

    fn convert_shape_defect(defect: SectionShapeDefect) -> DocFinding {
        let (violation, message, reason, fix) = match defect.kind {
            SectionShapeDefectKind::ExtraColumns => (
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_EXTRA_COLUMNS,
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
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_HEADER_REPEATED,
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
                consts::TEMPLATE_CONFORMANCE_VIOLATION_TABLE_MISSING_COLUMNS,
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
                consts::TEMPLATE_CONFORMANCE_VIOLATION_SECTION_NOT_TABLE,
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
        DocFinding::new_with_line("", defect.line, consts::RULE_CODE_README_DOC, violation, message)
            .with_reason(reason, fix)
    }
}
