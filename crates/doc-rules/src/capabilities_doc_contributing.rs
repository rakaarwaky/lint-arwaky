// PURPOSE: Per-doc capability for CONTRIBUTING.md — AES604
//
// Answers ALL invariants scoped to CONTRIBUTING.md behind
// IContributingDocProtocol. Crosslinks (PRD, ARCHITECTURE), heading
// structure, and entry-table conformance are stamped with
// RULE_CODE_CONTRIBUTING_DOC ("AES604"). The capability stamps the document
// path on every finding.

use shared_doc_rules::contract_doc_protocol::IContributingDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_vo::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_capabilities_doc_h2_checks::{
    SectionShapeDefect, SectionShapeDefectKind, detect_h2_table,
};
use shared_doc_rules::utility_doc_common_checks::{
    detect_doc_heading, heading_defect_messages, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_doc_line_drift::detect_doc_line_drift;
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

// ─── Block 1: Module Declaration ───────────────────────────

pub struct ContributingCapability;

impl Default for ContributingCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl ContributingCapability {
    pub fn new() -> Self {
        Self
    }
}

// ─── Block 2: Module Contract ─

impl IContributingDocProtocol for ContributingCapability {
    fn audit_contributing(&self, context: &DocAuditContext) -> Vec<DocFinding> {
        let mut findings = Vec::new();
        for document in context.documents() {
            if context.document_name(document) != consts::CONTRIBUTING_DOC {
                continue;
            }
            let mut document_findings = Vec::new();
            Self::audit_one(document, context, &mut document_findings);
            context.stamp(document, &mut document_findings);
            for f in &mut document_findings {
                f.code = consts::RULE_CODE_CONTRIBUTING_DOC;
            }
            findings.extend(document_findings);
        }
        findings
    }
}

// ─── Block 3: Module Helpers ───

impl ContributingCapability {
    fn audit_one(document: &DocSource, context: &DocAuditContext, findings: &mut Vec<DocFinding>) {
        let text = &document.text;
        let parsed = sections(text);

        // Template body conformance.
        let drift = detect_doc_line_drift(text, consts::CONTRIBUTING_TEMPLATE_LINES);
        findings.extend(
            shared_doc_rules::utility_doc_line_drift::line_drift_findings(
                drift,
                consts::CONTRIBUTING_DOC,
                consts::RULE_CODE_CONTRIBUTING_DOC,
            ),
        );

        // Crosslinks (PRD, ARCHITECTURE).
        Self::check_crosslinks(
            document,
            context,
            consts::RULE_CODE_CONTRIBUTING_DOC,
            findings,
        );

        // Heading structure.
        if let Some((required, allowed)) = doc_h2_contract(consts::CONTRIBUTING_DOC) {
            for defect in detect_doc_heading(text, consts::CONTRIBUTING_DOC, required, allowed) {
                let (violation, message, reason, fix) =
                    heading_defect_messages(consts::CONTRIBUTING_DOC, &defect);
                let mut f = DocFinding::new_with_line(
                    "",
                    defect.line(),
                    consts::RULE_CODE_CONTRIBUTING_DOC,
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

        // Workflow table carries fixed columns.
        if let Some(defect) =
            detect_h2_table(&parsed, "Workflow", consts::CONTRIBUTING_WORKFLOW_COLUMNS)
        {
            findings.push(Self::convert_shape_defect(defect));
        }
        // Commit Convention table carries fixed columns.
        if let Some(defect) = detect_h2_table(
            &parsed,
            "Commit Convention",
            consts::CONTRIBUTING_COMMIT_COLUMNS,
        ) {
            findings.push(Self::convert_shape_defect(defect));
        }
    }

    fn convert_shape_defect(defect: SectionShapeDefect) -> DocFinding {
        let (violation, message, reason, fix) = match defect.kind {
            SectionShapeDefectKind::ExtraColumns => (
                DOC_VIOLATION_NAMES.table_extra_columns(),
                format!(
                    "`{} {}` table carries column(s) outside the template: {}",
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
                    "`{} {}` repeats its header row as a data row",
                    defect.marker, defect.title
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
                    "`{} {}` must be a markdown table with columns {}",
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
                    "`{} {}` must be a markdown table with columns {}",
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
            consts::RULE_CODE_CONTRIBUTING_DOC,
            violation,
            message,
        )
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
