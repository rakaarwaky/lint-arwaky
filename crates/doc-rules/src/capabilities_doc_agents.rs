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
use shared_doc_rules::utility_capabilities_doc_h2_checks::{
    check_h2_fixed_rows, check_h2_table,
};
use shared_doc_rules::utility_doc_common_checks::{
    check_doc_heading, check_agents_line_drift, missing_link_violation_type, required_links,
};
use shared_doc_rules::utility_markdown_scanner::{
    doc_h2_contract, sections,
};

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
            text,
            &AGENTS_TEMPLATE.lines().collect::<Vec<&str>>(),
            findings,
            consts::AGENTS_DOC,
        );

        // AES604 — crosslinks (README, CONTRIBUTING, ARCHITECTURE, ROADMAP, PRD).
        Self::check_crosslinks(document, context, consts::RULE_CODE_CROSSLINKS, findings);

        // AES605 — heading structure (exactly one H1, required H2s, no off-template H2s).
        if let Some((required, allowed)) = doc_h2_contract(consts::AGENTS_DOC) {
            check_doc_heading(text, consts::AGENTS_DOC, required, allowed, findings);
        }

        // AES606 — Runtime table carries the fixed row labels.
        findings.extend(check_h2_fixed_rows(
            &parsed,
            "Runtime",
            consts::AGENTS_RUNTIME_ROW_LABELS,
        ));

        // AES606 — Commands table carries the fixed columns.
        findings.extend(check_h2_table(
            &parsed,
            "Commands",
            consts::AGENTS_COMMAND_COLUMNS,
        ));
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
