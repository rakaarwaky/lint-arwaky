// PURPOSE: Per-doc capability for AGENTS.md — AES601
//
// Answers ALL invariants scoped to AGENTS.md behind IAgentsDocProtocol.
// Crosslinks, heading structure, and Runtime entry-table fixed-row
// conformance are stamped with RULE_CODE_AGENTS_DOC ("AES601").
// The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IAgentsDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_names::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_capabilities_doc_h2_checks::{
    H2FixedRowsDefect, SectionShapeDefect, SectionShapeDefectKind, detect_h2_fixed_rows,
    detect_h2_table,
};
use shared_doc_rules::utility_doc_common_checks::{
    HeadingDefect, LineDriftDefect, detect_agents_line_drift, detect_doc_heading,
    missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

#[derive(Default)]
pub struct AgentsCapability;

impl AgentsCapability {
    pub fn new() -> Self {
        Self
    }
}

impl IAgentsDocProtocol for AgentsCapability {
    fn audit_agents(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::AGENTS_DOC {
                continue;
            }
            Self::audit_one(document, context, &mut findings);
            context.stamp(document, &mut findings);
            for f in &mut findings {
                f.code = consts::RULE_CODE_AGENTS_DOC;
            }
        }
        findings
    }
}

impl AgentsCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // AGENTS line drift against the template.
        for defect in
            detect_agents_line_drift(text, consts::AGENTS_TEMPLATE_LINES, consts::AGENTS_DOC)
        {
            let (violation, message, reason, fix) = match &defect {
                LineDriftDefect::FrontmatterMissing { line: _, doc_name } => (
                    DOC_VIOLATION_NAMES.frontmatter_missing(),
                    format!("{} has no YAML frontmatter block", doc_name),
                    format!(
                        "{} must open with a `---` YAML frontmatter block.",
                        doc_name
                    ),
                    format!(
                        "Add a `---` YAML frontmatter block at the top of {} .",
                        doc_name
                    ),
                ),
                LineDriftDefect::SectionNotInTemplate { line: _, section } => (
                    DOC_VIOLATION_NAMES.line_drift(),
                    format!("{} section is not in the AGENTS template", section),
                    format!(
                        "{} carries a section the template does not declare.",
                        section
                    ),
                    format!("Remove the off-template section `{}`.", section),
                ),
                LineDriftDefect::MissingTemplateSection { line: _, section } => (
                    DOC_VIOLATION_NAMES.line_drift(),
                    format!("missing template section: {}", section),
                    format!(
                        "{} is missing the template section `{}`.",
                        consts::AGENTS_DOC,
                        section
                    ),
                    format!("Add `{}`.", section),
                ),
                LineDriftDefect::LineMismatch {
                    line: _,
                    section,
                    expected,
                    found,
                } => (
                    DOC_VIOLATION_NAMES.line_drift(),
                    format!(
                        "{} section line drift: expected `{}` found `{}`",
                        section, expected, found
                    ),
                    format!(
                        "{} section of `{}` must match the template line-by-line.",
                        section,
                        consts::AGENTS_DOC
                    ),
                    format!("Align `{}` section with the template.", section),
                ),
                LineDriftDefect::H1NoTitle { line: _ } => (
                    DOC_VIOLATION_NAMES.line_drift(),
                    "AGENTS.md H1 has no title".to_string(),
                    format!("{} H1 heading carries no title.", consts::AGENTS_DOC),
                    format!("Give the {} H1 a title.", consts::AGENTS_DOC),
                ),
            };
            findings.push(
                DocFinding::new_with_line(
                    "",
                    defect.line(),
                    consts::RULE_CODE_AGENTS_DOC,
                    violation,
                    message,
                )
                .with_reason(reason, fix),
            );
        }

        // Crosslinks (README, CONTRIBUTING, ARCHITECTURE, ROADMAP, PRD).
        Self::check_crosslinks(document, context, consts::RULE_CODE_AGENTS_DOC, findings);

        // Heading structure (exactly one H1, required H2s, no off-template H2s).
        if let Some((required, allowed)) = doc_h2_contract(consts::AGENTS_DOC) {
            for defect in detect_doc_heading(text, consts::AGENTS_DOC, required, allowed) {
                let (violation, message, reason, fix) = match &defect {
                    HeadingDefect::H1Count { line: _, count } => (
                        consts::DOC_STRUCTURE_VIOLATION_H1_COUNT,
                        format!("{} H1 headings; template requires exactly one", count),
                        format!(
                            "{} has {} H1 headings; the template fixes exactly one H1 at the top.",
                            consts::AGENTS_DOC,
                            count
                        ),
                        format!(
                            "Remove the extra H1 heading(s) so {} keeps exactly one H1.",
                            consts::AGENTS_DOC
                        ),
                    ),
                    HeadingDefect::H2Missing { line: _, missing } => (
                        consts::DOC_STRUCTURE_VIOLATION_H2_MISSING,
                        format!("missing required H2 section(s): {}", missing.join(", ")),
                        format!(
                            "{} is missing required H2 section(s): {}",
                            consts::AGENTS_DOC,
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
                            consts::AGENTS_DOC,
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
                        consts::RULE_CODE_AGENTS_DOC,
                        violation,
                        message,
                    )
                    .with_reason(reason, fix),
                );
            }
        }

        // Runtime table carries the fixed row labels.
        if let Some(defect) =
            detect_h2_fixed_rows(&parsed, "Runtime", consts::AGENTS_RUNTIME_ROW_LABELS)
        {
            findings.push(Self::convert_fixed_rows_defect(
                defect,
                consts::AGENTS_RUNTIME_ROW_LABELS,
            ));
        }

        // Commands table carries the fixed columns.
        if let Some(defect) = detect_h2_table(&parsed, "Commands", consts::AGENTS_COMMAND_COLUMNS) {
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
            consts::RULE_CODE_AGENTS_DOC,
            violation,
            message,
        )
        .with_reason(reason, fix)
    }

    fn convert_fixed_rows_defect(defect: H2FixedRowsDefect, row_labels: &[&str]) -> DocFinding {
        DocFinding::new_with_line(
            "",
            defect.line,
            consts::RULE_CODE_AGENTS_DOC,
            DOC_VIOLATION_NAMES.entry_missing_row(),
            format!(
                "line {} `## {}` table row labels do not match the template; missing {}, extra {}",
                defect.line,
                defect.title,
                defect.missing.join(", "),
                defect.extra.join(", ")
            ),
        )
        .with_reason(
            format!(
                "The `{}` section must carry the fixed row labels {} so the table is complete.",
                defect.title,
                row_labels.join(", ")
            ),
            format!(
                "Add the missing row(s) {} and remove the extra row(s) {} from the `## {}` table.",
                defect.missing.join(", "),
                defect.extra.join(", "),
                defect.title
            ),
        )
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
