// PURPOSE: Per-doc capability for CONTRIBUTING.md — AES604
//
// Answers ALL invariants scoped to CONTRIBUTING.md behind
// IContributingDocProtocol. Crosslinks (PRD, ARCHITECTURE), heading
// structure (AES605), and entry-table conformance (AES606) are stamped with
// RULE_CODE_CONTRIBUTING_DOC ("AES604"). The capability stamps the document
// path on every finding.

use shared_doc_rules::contract_doc_protocol::IContributingDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_names::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_capabilities_doc_h2_checks::{
    detect_h2_table, SectionShapeDefect, SectionShapeDefectKind,
};
use shared_doc_rules::utility_doc_common_checks::{
    detect_doc_heading, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

#[derive(Default)]
pub struct ContributingCapability;

impl ContributingCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IContributingDocProtocol for ContributingCapability {
    fn audit_contributing(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::CONTRIBUTING_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_CONTRIBUTING_DOC;
            }
        }
        findings
    }
}

impl ContributingCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AES604 — crosslinks (PRD, ARCHITECTURE).
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::CONTRIBUTING_DOC) {
            for defect in detect_doc_heading(text, consts::CONTRIBUTING_DOC, required, allowed) {
                let (violation, message, reason, fix) = match &defect {
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H1Count { line, count } => (
                        DOC_VIOLATION_NAMES.h1_count_violation(),
                        format!("{} opens with {} level-1 headings", consts::CONTRIBUTING_DOC, count),
                        format!("{} must open with exactly one H1 naming the project", consts::CONTRIBUTING_DOC),
                        format!("Keep exactly one `# {}` line at the top of {}", consts::CONTRIBUTING_DOC, consts::CONTRIBUTING_DOC),
                    ),
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H2Missing { line, missing } => (
                        DOC_VIOLATION_NAMES.h2_missing(),
                        format!("{} has no H2 heading for {}", consts::CONTRIBUTING_DOC, missing.join(", ")),
                        format!("{} needs a level-2 heading for every template-required section", consts::CONTRIBUTING_DOC),
                        format!("Add the missing `## {}` section(s) to {}", missing.join(" `## `"), consts::CONTRIBUTING_DOC),
                    ),
                    shared_doc_rules::utility_doc_common_checks::HeadingDefect::H2Unexpected { line, unexpected } => (
                        DOC_VIOLATION_NAMES.h2_unexpected(),
                        format!("{} carries H2 heading(s) outside the template: {}", consts::CONTRIBUTING_DOC, unexpected.join(", ")),
                        format!("{} must not carry H2 headings the template does not declare", consts::CONTRIBUTING_DOC),
                        format!("Remove or rename the unexpected heading(s): {}", unexpected.join(", ")),
                    ),
                };
                findings.push(
                    DocFinding::new_with_line("", defect.line(), consts::RULE_CODE_CONTRIBUTING_DOC, violation, message)
                        .with_reason(reason, fix),
                );
            }
        }

        // AES604 — Workflow table carries fixed columns.
        if let Some(defect) = detect_h2_table(&parsed, "Workflow", consts::CONTRIBUTING_WORKFLOW_COLUMNS) {
            findings.push(Self::convert_shape_defect(defect));
        }
        // AES604 — Commit Convention table carries fixed columns.
        if let Some(defect) = detect_h2_table(&parsed, "Commit Convention", consts::CONTRIBUTING_COMMIT_COLUMNS) {
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
        DocFinding::new_with_line("", defect.line, consts::RULE_CODE_CONTRIBUTING_DOC, violation, message)
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
