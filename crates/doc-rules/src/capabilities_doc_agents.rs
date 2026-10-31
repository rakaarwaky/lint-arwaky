// PURPOSE: Per-doc capability for AGENTS.md — AES601
//
// Answers ALL invariants scoped to AGENTS.md behind IAgentsDocProtocol.
// Crosslinks (AES604), heading structure (AES605), and Runtime entry-table
// fixed-row conformance (AES606) are stamped with RULE_CODE_AGENTS_DOC
// ("AES601"). The capability stamps the document path on every finding.

use shared_doc_rules::contract_doc_protocol::IAgentsDocProtocol;
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_constant as consts;
use shared_doc_rules::taxonomy_doc_rules_request::{DocFinding, DocSource};
use shared_doc_rules::taxonomy_doc_rules_violation_names::DOC_VIOLATION_NAMES;
use shared_doc_rules::utility_capabilities_doc_h2_checks::{
    detect_h2_fixed_rows, detect_h2_table, H2FixedRowsDefect, SectionShapeDefect,
    SectionShapeDefectKind,
};
use shared_doc_rules::utility_doc_common_checks::{
    check_agents_line_drift, check_doc_heading, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_markdown_scanner::{doc_h2_contract, sections};

/// Canonical AGENTS.md template — the reference shape AGENTS.md must match
/// verbatim (modulo `{...}` placeholder slots). Transcribed from
/// HOW-TO-MAKE-AGENTS.md and the workspace AGENTS.md.
pub const AGENTS_TEMPLATE: &str = "---
trigger: always
description: \"Sample.\"
---

# Sample AGENTS.md

## Agent Overview

{One paragraph: what AI agents must know before generating code, docs, or suggestions in this workspace.}

## Runtime

| Aspect          | Detail                                          |
|-----------------|-------------------------------------------------|
| Language        | {language and pinned version}                   |
| Environment     | {environment manager and creation method}       |
| Artifacts       | {reviewable artifact location}                  |
| Package Manager | {package manager and lockfile}                  |
| Version Probe   | `{version probe command}`                       |
| Setup           | `{environment setup command}`                    |
| Activation      | `{optional environment activation command}`      |

## Project Structure

| Path       | Purpose              |
|------------|----------------------|
| {path}     | {what lives here}    |

## Commands

| Category    | Command                              | Scope      | CI Gate              |
|-------------|--------------------------------------|------------|----------------------|
| Test (all)  | `{whole-workspace test command}`     | All        | ci.yml: {job name}   |
| Test (unit) | `{single-package test command}`      | One unit   | ci.yml: {job name}   |
| Test (file) | `{single-file test command}`         | One file   | ci.yml: {job name}   |
| Lint        | `{formatter or linter command}`      | All        | ci.yml: {job name}   |
| Type Check  | `{type checker command}`             | All        | ci.yml: {job name}   |
| Arch Scan   | `{architecture scanner command}`     | All        | ci.yml: {job name}   |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| README       | What the tool does, every command, install | [README.md](README.md) |
| CONTRIBUTING | Branch, commit, and PR conventions         | [CONTRIBUTING.md](CONTRIBUTING.md) |
| ARCHITECTURE | Layer contract for every crate             | [ARCHITECTURE.md](ARCHITECTURE.md) |
| ROADMAP      | What ships next, in order                  | [ROADMAP.md](ROADMAP.md) |
| PRD          | What the product does and why              | [PRD.md](PRD.md) |
";

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

        // AES601 — AGENTS line drift against the template.
        check_agents_line_drift(
            consts::RULE_CODE_AGENTS_DOC,
            text,
            &AGENTS_TEMPLATE.lines().collect::<Vec<&str>>(),
            &DOC_VIOLATION_NAMES,
            findings,
            consts::AGENTS_DOC,
        );

        // AES604 — crosslinks (README, CONTRIBUTING, ARCHITECTURE, ROADMAP, PRD).
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure (exactly one H1, required H2s, no off-template H2s).
        if let Some((required, allowed)) = doc_h2_contract(consts::AGENTS_DOC) {
            check_doc_heading(
                consts::RULE_CODE_AGENTS_DOC,
                text,
                consts::AGENTS_DOC,
                required,
                allowed,
                &DOC_VIOLATION_NAMES,
                findings,
            );
        }

        // AES601 — Runtime table carries the fixed row labels.
        if let Some(defect) = detect_h2_fixed_rows(&parsed, "Runtime", consts::AGENTS_RUNTIME_ROW_LABELS) {
            findings.push(Self::convert_fixed_rows_defect(defect, consts::AGENTS_RUNTIME_ROW_LABELS));
        }

        // AES601 — Commands table carries the fixed columns.
        if let Some(defect) = detect_h2_table(&parsed, "Commands", consts::AGENTS_COMMAND_COLUMNS) {
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
        DocFinding::new_with_line("", defect.line, consts::RULE_CODE_AGENTS_DOC, violation, message)
            .with_reason(reason, fix)
    }

    fn convert_fixed_rows_defect(defect: H2FixedRowsDefect, row_labels: &[&str]) -> DocFinding {
        DocFinding::new_with_line(
            "",
            defect.line,
            consts::RULE_CODE_AGENTS_DOC,
            consts::TEMPLATE_CONFORMANCE_VIOLATION_ENTRY_MISSING_ROW,
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
