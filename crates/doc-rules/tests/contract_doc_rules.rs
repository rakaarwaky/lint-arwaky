// PURPOSE: contract tests for doc-rules — each invariant must fire on a bad
// document and stay silent on a conforming one. Findings carry a code
// (AES601–AES605) plus a violation_type, and both are asserted.
use doc_rules_lint_arwaky::root_doc_rules_container::RootDocRulesContainer;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;

use std::fs;
use std::path::Path;

/// A FRD that satisfies every structural invariant.
fn conforming_frd() -> String {
    r#"# FRD — sample

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature.

## System Overview

The feature does the work a surface delegates to.

## Functional Requirements

### FR-SAMPLE-001: Document Invariant Enforcement

- **Description**: Every `.md` document satisfies the AES heading and section contract.
- **Input**: A workspace root and the list of recognized document paths.
- **Output**: `Vec<LintResult>` carrying one finding per invariant violation.
- **Business Rules**: Validates FR-ID format, section structure, spec purity, crosslinks, heading structure, and parity.
- **Edge Cases**: Root-level legacy `BACKLOG.md` is accepted during migration.
- **Error Handling**: Unreadable files produce no findings. Missing documents are skipped silently.

### FR-SAMPLE-002: Audit Orchestration

- **Description**: The doc auditor is reachable through a single aggregate entry point dispatching to the invariant checker.
- **Input**: A `DocRequest` describing the workspace root to audit.
- **Output**: A `DocResponse` carrying a deduplicated, stable-sorted list of findings.
- **Business Rules**: The aggregate orchestrates all invariants under one seam so consumers need only invoke `execute`.
- **Edge Cases**: A request with an empty root produces no findings rather than an error.
- **Error Handling**: Failures inside the checker are propagated as `DocResponse::Findings` with no partial results.

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Single composite entry point. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Routes to the capability. |

## Integration Points

| System | Direction | Purpose | Failure mode |
| --- | --- | --- | --- |
| Shared domain | in | Supplies value objects | None — types only |

## Non-functional Requirements

| Metric | Target | Measurement method |
| --- | --- | --- |
| Dispatch cost | One call per request | Read the dispatch table |

## Test Scenarios

- A conforming request produces a conforming response.

## Assumptions & Constraints

- The capability is stateless.

## Glossary

- **Sample**: A value object used in this example.
"#
    .to_string()
}

/// A PRD.md that satisfies its own AES605 H2 contract.
fn conforming_prd() -> &'static str {
    r"# PRD — sample

## Problem Statement

The problem.

## Goals & Success Metrics

- Fewer defects.

## User Personas

- A maintainer.

## Scope

In scope: the checker.

## Feature Requirements

- P0 — the checker reports every violation.

## Non-functional Requirements

| Metric | Target |
| --- | --- |
| Dispatch | One call |

## Open Questions / Risks

- None.
"
}

/// A ROADMAP.md that satisfies its own AES605 H2 contract.
fn conforming_roadmap() -> &'static str {
    r"# ROADMAP — sample

## Current Condition

- Todo: none
- In Progress: None
- Blocked: None

## Feature Roll-up

| ID | Item |
|---|---|
| WS-1 | Docs. |

## Status Policy

A row is Done with a command.

## State Definitions

| State | Meaning |
|---|---|
| Done | Shipped. |

## Health Definitions

| Health | Meaning |
|---|---|
| On Track | No threat. |

## Risk Register

| Risk | Mitigation |
|---|---|
| None | — |
"
}

/// A conforming BACKLOG.md that satisfies the AES605 H2 contract.
fn conforming_backlog() -> &'static str {
    r"# Feature Backlog: Sample

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State / Health: values from root [ROADMAP.md](../../ROADMAP.md) — do not redefine here.
Last Updated: 2026-09-29

## Current Condition

- Done: nothing yet
- In Progress: None
- Blocked: None
- Next Action: get started

## Backlog

| ID | Priority | State | Health | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|---|
| SAM-01 | P0 | Ready | On Track | — | — | 2026-09-29 |

## Scenario Evidence

| Scenario | Kind | Test file | Test name | Last verified |
|---|---|---|---|---|
| nothing yet | Automated | tests/ | test_nothing | 2026-09-29 |

## Blockers

None

## Dependencies

None

## Release Readiness

| Area | Status | Notes |
|---|---|---|
| Tests | Done | nothing |
| Scenario evidence | Done | nothing |
| Docs | Done | [FRD.md](FRD.md) is specification-only. |

## Deferred

None

## Change Log

| Date | Change | By |
|---|---|---|
| 2026-09-29 | init | @raka |
"
}

/// Build a workspace whose only feature folder is a real feature, so AES702
/// (folder health + docs, moved to structure-rules) does not fire.
fn write_workspace(dir: &Path, frd: &str) {
    write_workspace_in_layout("crates", dir, frd);
}

/// Build a workspace whose feature folder lives under *layout*
/// (`crates` | `modules` | `packages`).
fn write_workspace_in_layout(layout: &str, dir: &Path, frd: &str) {
    let feature = dir.join(layout).join("sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    fs::write(feature.join("FRD.md"), frd).unwrap();
    fs::write(feature.join("BACKLOG.md"), conforming_backlog()).unwrap();
    fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();
    fs::write(dir.join("PRD.md"), conforming_prd()).unwrap();
    fs::write(dir.join("ROADMAP.md"), conforming_roadmap()).unwrap();
}

/// Build a workspace with a conforming AGENTS.md so AES605 stays silent.
fn write_agents_workspace(dir: &Path) {
    let feature = dir.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    fs::write(feature.join("FRD.md"), conforming_frd()).unwrap();
    fs::write(feature.join("BACKLOG.md"), conforming_backlog()).unwrap();
    fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();
    fs::write(dir.join("PRD.md"), conforming_prd()).unwrap();
    fs::write(dir.join("ROADMAP.md"), conforming_roadmap()).unwrap();
    fs::write(
        dir.join("AGENTS.md"),
        "# Sample AGENTS.md\n\n\
## User Context\n\n\
- Preferences: concise.\n\n\
## Precedence\n\n\
1. Explicit user approval.\n\n\
## Security\n\n\
- Explicit approval is required before destructive actions.\n\n\
## Memory\n\n\
- Write important state to the session notes.\n\n\
## Session Start\n\n\
Read the todo list, then check state.\n\n\
## Runtime\n\n\
- Language: Rust.\n\n\
## Quick Facts\n\n\
INPUT  = sample input\n\
OUTPUT = sample output\n\n\
## Pipeline\n\n\
scan → check\n\
lint\n\
orchestrated by CI\n\n\
## Git Workflow\n\n\
Every change must use a worktree or branch.\n\n\
## Commands\n\n\
```bash\ncargo nextest run --workspace\n```\n\n\
## Guided Skills\n\n\
Use `.agents/skills` when a task matches a guided workflow.\n\n\
## Definition of Done\n\n\
A change is done when tests pass.\n\n\
## Writing Style\n\n\
Lead with the point.\n\n\
## Related Documents\n\n\
- [PRD.md](PRD.md): Product requirements.\n",
    )
    .unwrap();
}

fn write_bad_agents(dir: &Path, agent_text: &str) {
    write_agents_workspace(dir);
    fs::write(dir.join("AGENTS.md"), agent_text).unwrap();
}

fn audit(root: &Path) -> Vec<(String, String, String, String)> {
    let orchestrator = RootDocRulesContainer::orchestrator();
    match orchestrator.execute(DocRequest::audit_all(root)) {
        DocResponse::Findings { findings } => findings
            .into_iter()
            .map(|f| {
                (
                    f.code.to_string(),
                    f.violation_type.to_string(),
                    f.message,
                    f.fix,
                )
            })
            .collect(),
    }
}

/// Does any finding carry this (code, violation_type) pair?
fn has(findings: &[(String, String, String, String)], code: &str, violation_type: &str) -> bool {
    findings
        .iter()
        .any(|(c, v, _, _)| c == code && v == violation_type)
}

#[test]
fn conforming_frd_reports_no_findings() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    let findings = audit(tmp.path());
    assert!(findings.is_empty(), "expected clean, got: {findings:#?}");
}

#[test]
fn aes601_fires_on_bare_fr_id() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace("### FR-SAMPLE-001:", "### FR-001:");
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "id_missing_feature_prefix"),
        "expected id_missing_feature_prefix, got: {findings:#?}"
    );
}

#[test]
fn aes601_reports_each_missing_field_with_its_own_type() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd()
        .replace(
            "- **Edge Cases**: Root-level legacy `BACKLOG.md` is accepted during migration.\n",
            "",
        )
        .replace(
            "- **Error Handling**: Unreadable files produce no findings. Missing documents are skipped silently.\n",
            "",
        );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    let field_findings: Vec<&str> = findings
        .iter()
        .filter(|(c, v, _, _)| c == "AES601" && v == "field_missing")
        .map(|(_, _, m, _)| m.as_str())
        .collect();
    // Only FR-SAMPLE-001 loses its fields (FR-SAMPLE-002's text differs); expect 1 finding.
    assert_eq!(
        field_findings.len(),
        1,
        "one finding per FR with missing fields: {findings:#?}"
    );
    let message = &field_findings[0];
    assert!(
        message.contains("Edge Cases") && message.contains("Error Handling"),
        "the message must name each missing field, got: {message}"
    );
}

#[test]
fn aes602_fires_when_aggregate_api_is_absent() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd()
        .replace("### Aggregate API", "### Other API")
        .replace(
            "| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Routes to the capability. |\n",
            "",
        );
    write_workspace(tmp.path(), &frd);
    assert!(
        has(&audit(tmp.path()), "AES602", "api_no_subsection"),
        "expected api_no_subsection, got: {:#?}",
        audit(tmp.path())
    );
}

// ── AES602: API Contract H3 strictness ───────────────────────────────

#[test]
fn aes602_fires_when_api_contract_has_a_per_protocol_h3() {
    let tmp = tempfile::tempdir().unwrap();
    // The original workaround: one Protocol API table holds every method of
    // every protocol, then a level-3 heading per protocol class narrates the
    // split that the single table refused to make.
    let frd = conforming_frd().replace(
        "### Aggregate API",
        "### IParserProtocol\n\n\
             | Method | Input | Output | Error | Event | Description |\n\
             | --- | --- | --- | --- | --- | --- |\n\
             | `parse` | `DocSource` | `Result<Tree>` | — | — | Parse one document. |\n\n\
             ### Aggregate API",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES602", "api_h3_unexpected"),
        "expected api_h3_unexpected, got: {findings:#?}"
    );
}

#[test]
fn aes602_fires_when_api_contract_has_two_extra_h3s() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "### Aggregate API",
        "### IGraphProtocol\n\n\
             | Method | Input | Output | Error | Event | Description |\n\
             | --- | --- | --- | --- | --- | --- |\n\
             | `build` | `Vec<Edge>` | `Graph` | — | — | Build. |\n\n\
             ### IParserProtocol\n\n\
             | Method | Input | Output | Error | Event | Description |\n\
             | --- | --- | --- | --- | --- | --- |\n\
             | `parse` | `DocSource` | `Result<Tree>` | — | — | Parse one document. |\n\n\
             ### Aggregate API",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    // Each unknown H3 is reported.
    let count = findings
        .iter()
        .filter(|(c, v, _, _)| c == "AES602" && v == "api_h3_unexpected")
        .count();
    assert_eq!(count, 2, "expected 2 api_h3_unexpected, got: {findings:#?}");
}

#[test]
fn aes602_fires_when_a_protocol_table_is_parked_under_assumptions() {
    let tmp = tempfile::tempdir().unwrap();
    // The escape `api_h3_unexpected` cannot see: `crates/filesystem/FRD.md`
    // shipped exactly this — the same per-protocol method tables the Protocol
    // API table already carries, moved to a parent section the API Contract
    // check never reads. The document-wide parity check is what catches it.
    let frd = conforming_frd().replace(
        "## Assumptions & Constraints\n\n- The capability is stateless.",
        "## Assumptions & Constraints\n\n\
         - The capability is stateless.\n\n\
         ### IToolResolutionProtocol (12 operations)\n\n\
         | Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | `has_local_bin` | `&Path, &ToolName` | `bool` | — | — | Local bin. |",
    );
    write_workspace(tmp.path(), &frd);
    assert!(
        has(&audit(tmp.path()), "AES602", "h3_off_template"),
        "expected h3_off_template for a protocol table under Assumptions, got: {:#?}",
        audit(tmp.path())
    );
}

#[test]
fn aes602_fires_when_a_scen_heading_is_promoted_below_test_scenarios() {
    let tmp = tempfile::tempdir().unwrap();
    // The template states Test Scenarios as a bullet list, so a scenario
    // promoted to a level-3 heading is an invented section.
    let frd = conforming_frd().replace(
        "## Test Scenarios\n\n- A conforming request produces a conforming response.",
        "## Test Scenarios\n\n\
         - A conforming request produces a conforming response.\n\n\
         ### SCEN-001: Dispatch\n\n\
         | # | Scenario | Expected |\n\
         | --- | --- | --- |\n\
         | 1 | Conforming request | Conforming response |",
    );
    write_workspace(tmp.path(), &frd);
    assert!(
        has(&audit(tmp.path()), "AES602", "h3_off_template"),
        "expected h3_off_template for an invented SCEN heading, got: {:#?}",
        audit(tmp.path())
    );
}

#[test]
fn aes602_accepts_the_three_sanctioned_h3_shapes() {
    let tmp = tempfile::tempdir().unwrap();
    // The conforming FRD already carries all three: two requirement headings
    // plus Protocol API and Aggregate API. Asserting silence proves the check
    // is not simply "any H3 is a violation".
    write_workspace(tmp.path(), &conforming_frd());
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES602", "h3_off_template"),
        "sanctioned H3 shapes must not fire, got: {findings:#?}"
    );
}

#[test]
fn aes602_accepts_a_demoted_h4_heading() {
    let tmp = tempfile::tempdir().unwrap();
    // The remedy the message names: demote to level 4 and keep the content.
    let frd = conforming_frd().replace(
        "## Assumptions & Constraints\n\n- The capability is stateless.",
        "## Assumptions & Constraints\n\n\
         - The capability is stateless.\n\n\
         #### Tool resolution details\n\n\
         - Resolution order is local bin, then system path.",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES602", "h3_off_template"),
        "a level-4 heading is detail, not a section; got: {findings:#?}"
    );
}

#[test]
fn aes602_h3_parity_ignores_headings_inside_a_fenced_block() {
    let tmp = tempfile::tempdir().unwrap();
    // A shell comment in a code fence is not a heading; the template's own
    // Verify block is fenced and must not be read as a section.
    let frd = conforming_frd().replace(
        "## Glossary\n\n- **Sample**: A value object used in this example.",
        "## Glossary\n\n\
         - **Sample**: A value object used in this example.\n\n\
         ```bash\n\
         ### not-a-heading\n\
         # also not a heading\n\
         ```",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES602", "h3_off_template"),
        "fenced headings must be ignored, got: {findings:#?}"
    );
}

#[test]
fn aes602_fires_when_protocol_api_is_prose_only() {
    let tmp = tempfile::tempdir().unwrap();
    // Author dumps every method into prose under Protocol API instead of a table.
    let frd = conforming_frd().replace(
        "| Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Single composite entry point. |\n\n\
         ### Aggregate API",
        "Every method is documented below.\n\n\
         ### Aggregate API",
    );
    write_workspace(tmp.path(), &frd);
    assert!(
        has(&audit(tmp.path()), "AES602", "api_subsection_no_table"),
        "expected api_subsection_no_table, got: {:#?}",
        audit(tmp.path())
    );
}

#[test]
fn aes602_fires_when_aggregate_api_is_prose_only() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "| Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Routes to the capability. |\n\n\
         ## Integration Points",
        "A single entry point routes requests.\n\n\
         ## Integration Points",
    );
    write_workspace(tmp.path(), &frd);
    assert!(
        has(&audit(tmp.path()), "AES602", "api_subsection_no_table"),
        "expected api_subsection_no_table, got: {:#?}",
        audit(tmp.path())
    );
}

#[test]
fn aes602_fires_when_aggregate_api_is_duplicated() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "### Aggregate API",
        "### Aggregate API\n\n\
         | Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Routes to the capability. |\n\n\
         ### Aggregate API",
    );
    write_workspace(tmp.path(), &frd);
    assert!(
        has(&audit(tmp.path()), "AES602", "api_subsection_duplicated"),
        "expected api_subsection_duplicated, got: {:#?}",
        audit(tmp.path())
    );
}

#[test]
fn aes602_fires_when_api_subsections_are_out_of_order() {
    let tmp = tempfile::tempdir().unwrap();
    // Swap the two subsections: Aggregate API before Protocol API.
    let frd = conforming_frd();
    let proto_pos = frd.find("### Protocol API").unwrap();
    let agg_pos = frd.find("### Aggregate API").unwrap();
    let agg_end = frd.find("## Integration Points").unwrap();
    let proto_body = frd[proto_pos..agg_pos].to_string();
    let agg_body = frd[agg_pos..agg_end].to_string();
    let head = &frd[..proto_pos];
    let tail = &frd[agg_end..];
    let frd = format!("{head}{agg_body}{proto_body}{tail}");
    write_workspace(tmp.path(), &frd);
    assert!(
        has(&audit(tmp.path()), "AES602", "order_violation"),
        "expected order_violation, got: {:#?}",
        audit(tmp.path())
    );
}

#[test]
fn aes602_conforming_api_contract_reports_no_findings() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    let findings = audit(tmp.path());
    // No API-Contract-related finding should fire on a conforming FRD.
    let api_findings: Vec<_> = findings
        .iter()
        .filter(|(c, v, _, _)| c == "AES602" && v.starts_with("api_"))
        .collect();
    assert!(
        api_findings.is_empty(),
        "expected no api_* findings on conforming FRD, got: {api_findings:#?}"
    );
}

#[test]
fn aes602_fires_when_integration_points_is_a_bullet_list() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "| System | Direction | Purpose | Failure mode |\n| --- | --- | --- | --- |\n| Shared domain | in | Supplies value objects | None — types only |",
        "- Shared domain supplies value objects",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES602", "integration_not_table"));
}

#[test]
fn aes602_fires_when_nfr_is_prose() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "| Metric | Target | Measurement method |\n| --- | --- | --- |\n| Dispatch cost | One call per request | Read the dispatch table |",
        "- Dispatch cost is one call per request.",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES602", "nfr_not_table"));
}

#[test]
fn aes602_fires_when_sections_are_out_of_order() {
    let tmp = tempfile::tempdir().unwrap();
    let scrambled = conforming_frd()
        .replace("## Test Scenarios", "## @@MARK@@")
        .replace("## Glossary", "## Test Scenarios")
        .replace("## @@MARK@@", "## Glossary");
    write_workspace(tmp.path(), &scrambled);
    assert!(has(&audit(tmp.path()), "AES602", "order_violation"));
}

#[test]
fn aes602_fires_when_scenarios_have_no_bullets() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "- A conforming request produces a conforming response.",
        "1. A conforming request produces a conforming response.",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES602", "scenarios_empty"));
}

#[test]
fn aes602_fires_when_glossary_has_no_bullets() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "- **Sample**: A value object used in this example.",
        "| Sample | A value object |",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES602", "glossary_empty"));
}

#[test]
fn aes603_fires_when_a_spec_names_a_source_file() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "The feature does the work a surface delegates to.",
        "The feature reads settings from main.rs at startup.",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES603", "source_file_named"),
        "expected source_file_named, got: {findings:#?}"
    );
    // The message must name the offending file, not just the rule.
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES603" && v == "source_file_named")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("main.rs"),
        "the message must name the file, got: {message}"
    );
}

#[test]
fn aes603_fires_when_a_spec_carries_status() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "The feature does the work a surface delegates to.",
        "The feature is fully implemented and shipped in v3.7.",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES603", "status_leak"));
}

#[test]
fn aes603_distinguishes_status_leak_from_source_file_naming() {
    // The same document trips both purity types under one code.
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "The feature does the work a surface delegates to.",
        "Implemented: main.rs is wired and shipped in v3.7.",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(has(&findings, "AES603", "status_leak"));
    assert!(has(&findings, "AES603", "source_file_named"));
}

#[test]
fn aes604_fires_when_the_frd_omits_its_backlog_link() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature.\n",
        "",
    );
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES604", "no_backlog_link"));
}

#[test]
fn aes604_fires_when_the_frd_omits_its_prd_link() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace("- PRD: [PRD.md](../../PRD.md)\n", "");
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES604", "no_prd_link"));
}

#[test]
fn aes604_fires_when_a_feature_backlog_restates_master_sections() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    fs::write(
        tmp.path().join("crates/sample/BACKLOG.md"),
        "# BACKLOG — sample\n\n\
## Current Condition\n\n\
- Done: nothing\n\n\
## Backlog\n\n\
| ID | Priority | State | Health | Dependencies | Next Action | Updated |\n\
|---|---:|---|---|---|---|---|\n\
| SAM-01 | P0 | Ready | On Track | — | — | 2026-09-29 |\n\n\
## Scenario Evidence\n\n\
| Scenario | Kind | Test file | Test name | Last verified |\n\
|---|---|---|---|---|\n\
| nothing | Automated | tests/ | test_nothing | 2026-09-29 |\n\n\
## Blockers\n\n\
None\n\n\
## Dependencies\n\n\
None\n\n\
## Release Readiness\n\n\
| Area | Status | Notes |\n\
|---|---|---|\n\
| Tests | Done | nothing |\n\
| Scenario evidence | Done | nothing |\n\
| Docs | Done | [FRD.md](FRD.md) is specification-only. |\n\n\
## Deferred\n\n\
None\n\n\
## Change Log\n\n\
| Date | Change | By |\n\
|---|---|---|\n\
| 2026-09-29 | init | @raka |\n\n\
## Risk Register\n\n\
- A risk.\n",
    )
    .unwrap();
    assert!(has(&audit(tmp.path()), "AES604", "state_vocab_restated"));
}

#[test]
fn prose_only_documents_are_audited_for_purity() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    fs::write(
        tmp.path().join("PRD.md"),
        "# PRD — sample\n\nThe checker reads defaults from main.rs and dispatches in lib.rs.\n",
    )
    .unwrap();
    assert!(
        has(&audit(tmp.path()), "AES603", "source_file_named"),
        "a PRD is a spec and must be audited for purity"
    );
}

#[test]
fn findings_are_deduplicated_and_ordered() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace("### FR-SAMPLE-001:", "### FR-001:");
    write_workspace(tmp.path(), &frd);
    let first = audit(tmp.path());
    let second = audit(tmp.path());
    assert_eq!(first, second, "two runs must agree");
    let mut sorted = first.clone();
    sorted.sort_by(|a, b| {
        (a.0.clone(), a.1.clone(), a.2.clone()).cmp(&(b.0.clone(), b.1.clone(), b.2.clone()))
    });
    assert_eq!(first, sorted, "findings must arrive sorted");
}

// ── AES605: Doc heading structure ────────────────────────────────────

#[test]
fn aes605_conforming_agents_reports_no_findings() {
    let tmp = tempfile::tempdir().unwrap();
    write_agents_workspace(tmp.path());
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES605", "h1_count") && !has(&findings, "AES605", "h2_missing"),
        "expected clean AGENTS.md, got: {findings:#?}"
    );
}

#[test]
fn aes605_fires_when_no_h1() {
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "## Security\n\n- Explicit approval is required.\n",
    );
    let findings = audit(tmp.path());
    assert!(has(&findings, "AES605", "h1_count"));
}

#[test]
fn aes605_fires_when_multiple_h1() {
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "# One\n\n# Two\n\n## Security\n\n- Explicit approval.\n",
    );
    let findings = audit(tmp.path());
    assert!(has(&findings, "AES605", "h1_count"));
}

#[test]
fn aes605_does_not_falsely_fire_on_sharp_commented_commands() {
    // A bash comment such as `# Tests (matches CI "Tests" job)` must NOT
    // count as an extra H1 — fenced code blocks are stripped first.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "\
# Sample AGENTS.md

## Security

- Be safe.

## Commands

```bash
# Tests (matches CI \"Tests\" job)
cargo nextest run
```

## Related Documents\n",
    );
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES605", "h1_count"),
        "fenced code block contents must not be treated as headings; got: {findings:#?}"
    );
}

#[test]
fn aes605_fires_when_required_h2_is_absent() {
    let tmp = tempfile::tempdir().unwrap();
    // Precedence and Definition of Done are missing from the required set.
    write_bad_agents(
        tmp.path(),
        "# Sample AGENTS.md\n\n\
## Security\n\n- Be safe.\n\n\
## Git Workflow\n\nUse a worktree.\n\n\
## Commands\n\n```bash\ntrue\n```\n\n\
",
    );
    let findings = audit(tmp.path());
    assert!(has(&findings, "AES605", "h2_missing"));
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_missing")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap();
    // The message names the absent section(s), not just the rule.
    assert!(
        message.contains("Precedence") || message.contains("Definition of Done"),
        "the message must name the absent section(s); got: {message}"
    );
}

#[test]
fn aes605_allows_extra_and_free_h3_headings() {
    // Extra H3 headings are allowed; only required H2s are enforced.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "# Sample AGENTS.md

## User Context

- Preferences: concise.

## Precedence

1. Safety rules.

## Security

- Be safe.

## Memory

- Write state down.

## Session Start

Read the todo list.

## Runtime

- Language: Rust.

## Quick Facts

INPUT  = sample
OUTPUT = sample

## Pipeline

scan → check
lint
run by CI

## Git Workflow

Use a worktree.

## Commands

```bash
true
```

## Guided Skills

Use `.agents/skills`.

## Definition of Done

Tests pass.

## Writing Style

Lead with the point.

## Related Documents

- [PRD.md](PRD.md).
",
    );
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES605", "h2_missing") && !has(&findings, "AES605", "h2_unexpected"),
        "extra H2/H3 headings should not trigger h2_missing; got: {findings:#?}"
    );
}

#[test]
fn aes605_fires_when_a_required_section_is_absent() {
    // Removing any required section must fire h2_missing.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "\
# Sample AGENTS.md

## Precedence

1. Safety rules.

## Security

- Be safe.

## Git Workflow

Use a worktree.

## Commands

```bash
true
```

## Definition of Done

Tests pass.

## Related Documents

- [PRD.md](PRD.md).
",
    );
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_missing"),
        "missing required sections should fire; got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_missing")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap();
    assert!(
        message.contains("User Context") && message.contains("Memory"),
        "message must name the missing sections; got: {message}"
    );
}

#[test]
fn aes605_fires_on_an_h2_outside_the_template() {
    // "Random Notes" is in neither the required nor the allowed set, so it
    // must be reported for demotion to H3 or removal.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "\
# Sample AGENTS.md

## Precedence

1. Safety rules.

## Security

- Be safe.

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for the layer specification.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT. See [LICENSE](LICENSE).

## Random Notes

Anything the author wanted to jot down.

## Git Workflow

Use a worktree.

## Commands

```bash
true
```

## Definition of Done

Tests pass.

## Related Documents

- [PRD.md](PRD.md).\
",
    );
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "an off-template H2 must fire h2_unexpected; got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap();
    assert!(
        message.contains("random notes"),
        "message must name the off-template heading; got: {message}"
    );
}

#[test]
fn aes605_allows_the_project_specific_h2_set() {
    // Only the 12 required H2 sections are enforced; no optional headings exist.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "# Sample AGENTS.md

## User Context

- Preferences: concise.

## Precedence

1. Safety rules.

## Security

- Be safe.

## Memory

- Write state down.

## Session Start

Read the todo list.

## Runtime

- Language: Rust.

## Quick Facts

INPUT  = sample
OUTPUT = sample

## Pipeline

scan → check
lint
run by CI

## Git Workflow

Use a worktree.

## Commands

```bash
true
```

## Guided Skills

Use `.agents/skills`.

## Definition of Done

Tests pass.

## Writing Style

Lead with the point.

## Related Documents

- [PRD.md](PRD.md).
",
    );
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES605", "h2_unexpected") && !has(&findings, "AES605", "h2_missing"),
        "the project's own H2 set must be accepted; got: {findings:#?}"
    );
}

/// Audit a single root document inside a workspace whose AGENTS.md is
/// already conforming, so only that document contributes findings.
fn audit_root_doc(name: &str, text: &str) -> Vec<(String, String, String, String)> {
    let tmp = tempfile::tempdir().unwrap();
    write_agents_workspace(tmp.path());
    fs::write(tmp.path().join(name), text).unwrap();
    audit(tmp.path())
}

#[test]
fn aes605_reports_a_missing_h2_naming_the_document() {
    // A ROADMAP.md without Risk Register must name the document and section.
    let text = conforming_roadmap().replace("## Risk Register", "## Risks");
    let findings = audit_root_doc("ROADMAP.md", &text);
    assert!(has(&findings, "AES605", "h2_missing"));
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_missing")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("ROADMAP.md") && message.contains("Risk Register"),
        "message must name the document and section; got: {message}"
    );
}

#[test]
fn aes605_reports_an_off_template_h2_with_a_remedy() {
    let text = r#"# Sample README

## Prerequisites

- A toolchain.

## Quick Start

Run it.

## Architecture

Delegate to ARCHITECTURE.md.

## Project Structure

Where things live.

## Available Scripts/Commands

The daily loop.

## Configuration

What to set.

## Testing

Run the tests.

## Contributing

Read CONTRIBUTING.md.

## License

MIT.

## Random Extra

Off-template.
"#;
    let findings = audit_root_doc("README.md", text);
    assert!(has(&findings, "AES605", "h2_unexpected"));
    let (message, fix) = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, fix)| (m.as_str(), fix.as_str()))
        .unwrap_or_default();
    assert!(
        message.contains("random extra") && fix.contains("level-3"),
        "message must name the heading and the fix must name the remedy; got: {message} / {fix}"
    );
}

#[test]
fn aes605_accepts_a_conforming_prd() {
    let findings = audit_root_doc("PRD.md", conforming_prd());
    assert!(
        !has(&findings, "AES605", "h2_missing") && !has(&findings, "AES605", "h2_unexpected"),
        "a conforming PRD must be accepted; got: {findings:#?}"
    );
}

#[test]
fn aes605_fires_h1_count_when_a_document_has_no_h1() {
    let findings = audit_root_doc(
        "CONTRIBUTING.md",
        "## Principles

- No bypasses.
",
    );
    assert!(has(&findings, "AES605", "h1_count"));
}

#[test]
fn aes605_fires_when_an_architecture_layer_section_is_absent() {
    // Strip Root Layer from an otherwise-conforming ARCHITECTURE.md.
    let text = r#"# Architecture

## 1. Purpose

Why.

## 2. Workspace Organization

Terms.

## 3. Naming Convention

Rule.

## 4. Vertical Slicing Layout

Layout.

## 5. Taxonomy Layer

Base.

## 6. Contract Layer

Boundary.

## 7. Utility Layer

Mechanics.

## 8. Capabilities Layer

Behavior.

## 9. Agent Layer

Sequence.

## 10. Surface Layer

Outer.
"#;
    let findings = audit_root_doc("ARCHITECTURE.md", text);
    assert!(has(&findings, "AES605", "h2_missing"), "{findings:#?}");
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_missing")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("Root Layer"),
        "only Root Layer is absent; got: {message}"
    );
}

#[test]
fn aes605_strips_a_leading_list_index_before_matching() {
    // Numbered sections like `## 11. Root Layer` must match the
    // template entry "Root Layer" instead of firing h2_unexpected.
    let text = r#"# Architecture

## 1. Purpose

Why.

## 2. Workspace Organization

Terms.

## 3. Naming Convention

Rule.

## 4. Vertical Slicing Layout

Layout.

## 5. Taxonomy Layer

Base.

## 6. Contract Layer

Boundary.

## 7. Utility Layer

Mechanics.

## 8. Capabilities Layer

Behavior.

## 9. Agent Layer

Sequence.

## 10. Surface Layer

Outer.

## 11. Root Layer

Entry.
"#;
    let findings = audit_root_doc("ARCHITECTURE.md", text);
    assert!(
        !has(&findings, "AES605", "h2_missing") && !has(&findings, "AES605", "h2_unexpected"),
        "numbered sections must match after index stripping; got: {findings:#?}"
    );
}

// ── AES601: FR/protocol class parity ──────────────────────────────────────

/// Write the shared contract module for the `sample` feature, declaring
/// *traits* protocol classes. The aggregate trait is never counted. When
/// there is more than one class, the last one moves to a second file so the
/// count proves a module spreads its classes over many files.
fn write_protocol_module(dir: &Path, traits: usize) {
    write_protocol_module_in_layout("crates", dir, traits);
}

/// Write the shared contract module for the `sample` feature under *layout*
/// (`crates` | `modules` | `packages`).
fn write_protocol_module_in_layout(layout: &str, dir: &Path, traits: usize) {
    let module = dir.join(layout).join("shared/src/sample");
    fs::create_dir_all(&module).unwrap();
    let split = traits > 1;
    let mut first = String::from("//! sample contract module\n\n");
    for n in 0..traits {
        if split && n + 1 == traits {
            continue;
        }
        // The seam declares `execute`, the method `conforming_frd()` promises
        // in both API tables, so the method-existence check stays silent.
        first.push_str(&format!(
            "pub trait ISample{n}Protocol: Send + Sync {{\n    fn execute(&self);\n}}\n\n"
        ));
    }
    first.push_str("pub trait ISampleAggregate: Send + Sync {\n    fn execute(&self);\n}\n");
    fs::write(module.join("contract_sample_protocol.rs"), first).unwrap();
    if split {
        let last = traits - 1;
        fs::write(
            module.join("contract_sample_extra_protocol.rs"),
            format!(
                "//! second contract file for the sample feature\n\n\
                 pub trait ISample{last}Protocol: Send + Sync {{\n    fn execute(&self);\n}}\n"
            ),
        )
        .unwrap();
    }
}

/// Write the shared contract module with one protocol class in the top file
/// and one in a nested subdirectory, proving the counter walks
/// subdirectories.
fn write_nested_protocol_module(dir: &Path) {
    let module = dir.join("crates/shared/src/sample");
    fs::create_dir_all(module.join("nested")).unwrap();
    fs::write(
        module.join("contract_sample_protocol.rs"),
        "//! sample contract module\n\npub trait ISample0Protocol: Send + Sync {\n    fn execute(&self);\n}\n\npub trait ISampleAggregate: Send + Sync {\n    fn execute(&self);\n}\n",
    )
    .unwrap();
    fs::write(
        module.join("nested/sub.rs"),
        "//! nested sub-module\n\npub trait ISample1Protocol: Send + Sync {\n    fn execute(&self);\n}\n",
    )
    .unwrap();
}

/// A FRD declaring *count* requirements, all structurally conforming.
/// Note: `conforming_frd()` already contains 2 FRs (FR-SAMPLE-001 and FR-SAMPLE-002),
/// so we build from a minimal base when count < 2 to avoid generating extra FRs.
fn frd_with_fr_count(count: usize) -> String {
    if count == 0 {
        return "# FRD\n".to_string();
    }
    let mut frd = conforming_frd();
    // conforming_frd() has 2 FRs. If count <= 2, strip FR-SAMPLE-002.
    if count <= 2 {
        let idx = frd.find("### FR-SAMPLE-002:").unwrap_or(frd.len());
        frd.truncate(idx);
    }
    // Add enough FRs to reach the target count.
    let existing = frd.matches("### FR-SAMPLE-").count();
    for n in (existing + 1)..=count {
        frd.push_str(&format!(
            "\n### FR-SAMPLE-{n:03}: Do Another Thing\n\n\
             - **Description**: The feature performs a second responsibility.\n\
             - **Input**: A request value object.\n\
             - **Output**: A response value object.\n\
             - **Business Rules**: The capability is stateless.\n\
             - **Edge Cases**: An absent input yields a default.\n\
             - **Error Handling**: Failures surface as a reason-coded outcome.\n"
        ));
    }
    frd
}

#[test]
fn aes607_stays_silent_when_fr_count_equals_protocol_class_count() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &frd_with_fr_count(3));
    write_protocol_module(tmp.path(), 3);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "protocol_count_mismatch"),
        "3 requirements and 3 protocol classes are aligned; got: {findings:#?}"
    );
}

#[test]
fn aes607_fires_when_the_frd_declares_more_requirements_than_protocol_classes() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &frd_with_fr_count(3));
    write_protocol_module(tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "protocol_count_mismatch"),
        "3 requirements against 2 protocol classes must fire; got: {findings:#?}"
    );
    let (message, fix) = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES601" && v == "protocol_count_mismatch")
        .map(|(_, _, m, fix)| (m.as_str(), fix.as_str()))
        .unwrap();
    assert!(
        message.contains("3 requirements") && message.contains("2 protocol classes"),
        "the message must state both counts; got: {message}"
    );
    assert!(
        fix.contains("split the methods into more classes")
            && fix.contains("merge the requirements down"),
        "the fix must name both directions; got: {fix}"
    );
}

#[test]
fn aes607_fires_when_the_code_declares_more_protocol_classes_than_requirements() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &frd_with_fr_count(1));
    write_protocol_module(tmp.path(), 3);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "protocol_count_mismatch"),
        "1 requirement against 3 protocol classes must fire; got: {findings:#?}"
    );
    let (_, fix) = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES601" && v == "protocol_count_mismatch")
        .map(|(_, _, m, fix)| (m.as_str(), fix.as_str()))
        .unwrap();
    assert!(
        fix.contains("split the requirements up to match")
            && fix.contains("merge the classes down"),
        "the fix must name both directions for this direction; got: {fix}"
    );
}

#[test]
fn aes607_ignores_aggregate_traits_when_counting_protocol_classes() {
    let tmp = tempfile::tempdir().unwrap();
    // One requirement, one protocol class, and one aggregate: the aggregate
    // is a composite entry point rather than a capability seam, so the
    // counts are aligned and nothing fires.
    write_workspace(tmp.path(), &frd_with_fr_count(1));
    write_protocol_module(tmp.path(), 1);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "protocol_count_mismatch"),
        "aggregates are not capability seams; got: {findings:#?}"
    );
}

#[test]
fn aes607_ignores_doc_h3_headings_that_look_like_protocols() {
    let tmp = tempfile::tempdir().unwrap();
    // The FRD carries `### IParserProtocol (6 operations)` as a level-3
    // heading — the same text shape the old counter's `.contains("Protocol")`
    // could have accidentally inflated from. Only Rust source files are read,
    // so doc headings can never pad the count.
    write_workspace(tmp.path(), &frd_with_fr_count(2));
    let frd = r#"# FRD — sample

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md).

## System Overview

The feature does the work.

## Functional Requirements

### FR-SAMPLE-001: Document one thing

- **Description**: Count the first.
- **Input**: Nothing.
- **Output**: One.
- **Business Rules**: Always one.
- **Edge Cases**: None.
- **Error Handling**: None.

### FR-SAMPLE-002: Document two things

- **Description**: Count the second.
- **Input**: Nothing.
- **Output**: Two.
- **Business Rules**: Always two.
- **Edge Cases**: None.
- **Error Handling**: None.

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | SampleRequest | SampleResponse | — | — | Routes. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | SampleRequest | SampleResponse | Reason-coded | — | Routes to the capability. |

## Integration Points

| System | Direction | Purpose | Failure mode |
| --- | --- | --- | --- |
| Shared domain | in | Supplies value objects | None — types only |

## Non-functional Requirements

| Metric | Target | Measurement method |
| --- | --- | --- |
| Dispatch cost | One call per request | Read the dispatch table |

## Test Scenarios

- A conforming request produces a conforming response.

## Assumptions & Constraints

- The capability is stateless.

## Glossary

- **Sample**: A value object used in this example.

### IParserProtocol (6 operations)

These are doc headings that must never inflate the protocol count.
"#;
    // The FRD above declares 2 FRs and carries doc headings shaped like
    // protocol classes; these headings sit in the Markdown, not in any .rs.
    fs::write(tmp.path().join("crates").join("sample").join("FRD.md"), frd).unwrap();
    fs::write(
        tmp.path().join("crates").join("sample").join("BACKLOG.md"),
        conforming_backlog(),
    )
    .unwrap();
    fs::write(
        tmp.path()
            .join("crates")
            .join("sample")
            .join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();
    fs::write(tmp.path().join("PRD.md"), conforming_prd()).unwrap();
    fs::write(tmp.path().join("ROADMAP.md"), conforming_roadmap()).unwrap();
    // The shared module declares exactly one capability seam.
    write_protocol_module(tmp.path(), 1);
    let findings = audit(tmp.path());
    // The FRD declares 2 FRs but the module carries 1 protocol class, so the
    // mismatch fires — but the doc headings must not have bumped the class
    // count up to 2 and silently silenced it.
    assert!(
        has(&findings, "AES601", "protocol_count_mismatch"),
        "doc headings should not inflate the count; got: {findings:#?}"
    );
}

#[test]
fn aes607_ignores_comments_and_strings_that_mention_protocol() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &frd_with_fr_count(1));
    // The module carries a doc comment, a string literal, and non-matching
    // identifiers that mention Protocol, plus a hidden trait. Only a real
    // `pub trait I*Protocol` line may be counted.
    let module = tmp
        .path()
        .join("crates")
        .join("shared")
        .join("src")
        .join("sample");
    fs::create_dir_all(&module).unwrap();
    fs::write(
        module.join("contract_sample_protocol.rs"),
        "/// Doc comment mentioning `pub trait IFakeProtocol`.\n\
         /// Another doc comment with `IProtocolSomething` in prose.\n\
         pub trait ISampleProtocol: Send + Sync {\n    fn execute(&self);\n}\n\
         \n\
         /// pub trait INotRealProtocol {}\n\
         // pub trait IHiddenProtocol {}\n\
         pub(crate) trait IInvisibleProtocol {}\n\
         pub trait IProtocolFactory {}\n\
         pub trait IProto {}\n\
         pub trait IFooAggregateProtocol {}\n",
    )
    .unwrap();
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "protocol_count_mismatch"),
        "comments, strings, and non-matching names must not inflate the count; got: {findings:#?}"
    );
}

#[test]
fn aes607_stays_silent_when_the_feature_has_no_shared_contract_module() {
    let tmp = tempfile::tempdir().unwrap();
    // No crates/shared/src/sample/ module, so there is nothing to compare
    // against and the check stays out of the way.
    write_workspace(tmp.path(), &frd_with_fr_count(2));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "protocol_count_mismatch"),
        "a feature with no shared contract module cannot mismatch; got: {findings:#?}"
    );
    assert!(
        !has(&findings, "AES601", "api_method_not_found"),
        "an unreadable contract module yields no method finding either; got: {findings:#?}"
    );
}

// ── AES601: Protocol/Aggregate API table methods ─────────────────────────────

#[test]
fn aes601_fires_when_an_api_table_method_is_not_declared_in_the_contract_module() {
    let tmp = tempfile::tempdir().unwrap();
    // The FRD promises a `audit_purity` seam the contract module never
    // declares: the counts still line up (2 requirements, 2 seams), so only
    // the method-existence check can answer this document.
    let frd = conforming_frd().replace(
        "| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Single composite entry point. |",
        "| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Single composite entry point. |\n\
         | `audit_purity` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Promised but never declared. |",
    );
    write_workspace(tmp.path(), &frd);
    write_protocol_module(tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "api_method_not_found"),
        "a promised method the module never declares must fire; got: {findings:#?}"
    );
    let (message, fix) = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES601" && v == "api_method_not_found")
        .map(|(_, _, m, fix)| (m.as_str(), fix.as_str()))
        .unwrap();
    assert!(
        message.contains("`audit_purity`") && message.contains("Protocol API"),
        "the message must name the method and the table it sits in; got: {message}"
    );
    assert!(
        fix.contains("Remove `audit_purity` from the")
            && fix.contains("create the protocol/aggregate method"),
        "the fix must offer both remedies; got: {fix}"
    );
}

#[test]
fn aes601_stays_silent_when_every_api_table_method_is_declared() {
    let tmp = tempfile::tempdir().unwrap();
    // Both tables promise `execute` and both seams declare it.
    write_workspace(tmp.path(), &conforming_frd());
    write_protocol_module(tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "api_method_not_found"),
        "a declared method must satisfy its table row; got: {findings:#?}"
    );
}

#[test]
fn aes601_reports_a_promised_method_missing_from_the_aggregate_table() {
    let tmp = tempfile::tempdir().unwrap();
    // The Aggregate API table promises `dispatch`, which no trait declares.
    let frd = conforming_frd().replace(
        "| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Routes to the capability. |",
        "| `dispatch` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Promised but never declared. |",
    );
    write_workspace(tmp.path(), &frd);
    write_protocol_module(tmp.path(), 2);
    let findings = audit(tmp.path());
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES601" && v == "api_method_not_found")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_else(|| panic!("the Aggregate API row must fire; got: {findings:#?}"));
    assert!(
        message.contains("Aggregate API") && message.contains("`dispatch`"),
        "the finding must name the aggregate table and its method; got: {message}"
    );
}

#[test]
fn aes601_ignores_a_method_mentioned_only_in_prose() {
    let tmp = tempfile::tempdir().unwrap();
    // `renamed` appears in the aggregate table's Description cell but never as
    // a row of its own, so it is prose and no row promises it.
    let frd = conforming_frd().replace(
        "Routes to the capability.",
        "Routes to the renamed `renamed` seam.",
    );
    write_workspace(tmp.path(), &frd);
    write_protocol_module(tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "api_method_not_found"),
        "a method named in prose is not a promised row; got: {findings:#?}"
    );
}

#[test]
fn aes601_reads_the_method_column_by_its_header_not_by_position() {
    let tmp = tempfile::tempdir().unwrap();
    // `auto-fix/FRD.md` leads its table with a `Protocol Trait` column, so the
    // promised method is the second cell. Reading the first cell would report
    // four trait names the code does declare as methods.
    let frd = conforming_frd().replace(
        "| Method | Input | Output | Error | Event | Description |\n| --- | --- | --- | --- | --- | --- |\n| `execute` |",
        "| Protocol Trait | Method | Input | Output | Error | Event | Description |\n|---|---|---|---|---|---|---|\n| `ISample0Protocol` (FR-SAMPLE-001) | `execute` |",
    );
    write_workspace(tmp.path(), &frd);
    write_protocol_module(tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "api_method_not_found"),
        "the Method column is found by its header, not by position; got: {findings:#?}"
    );
}

#[test]
fn aes601_reads_every_method_a_single_cell_promises() {
    let tmp = tempfile::tempdir().unwrap();
    // `file-watch/FRD.md` packs three methods into one cell as
    // `start` / `subscribe` / `stop`. All three are declared, so the cell is
    // satisfied; only one of them missing would fire, naming the one.
    let frd = conforming_frd()
        .replace(
            "| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Single composite entry point. |",
            "| `start` / `stop` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Watch lifecycle. |",
        )
        // The Aggregate API table still promises `execute`, so the module below
        // declares it alongside the two lifecycle methods.
        ;
    write_workspace(tmp.path(), &frd);
    let module = tmp.path().join("crates/shared/src/sample");
    fs::create_dir_all(&module).unwrap();
    fs::write(
        module.join("contract_sample_protocol.rs"),
        "pub trait ISample0Protocol: Send + Sync {\n    fn start(&self);\n    fn stop(&self);\n}\n\n\
         pub trait ISample1Protocol: Send + Sync {\n    fn start(&self);\n    fn stop(&self);\n}\n\n\
         pub trait ISampleAggregate: Send + Sync {\n    fn execute(&self);\n}\n",
    )
    .unwrap();
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "api_method_not_found"),
        "every name in a slash-joined cell is checked; got: {findings:#?}"
    );

    // Drop `stop` from the code and the same document must fire, naming only
    // the method that is actually gone.
    fs::write(
        module.join("contract_sample_protocol.rs"),
        "pub trait ISample0Protocol: Send + Sync {\n    fn start(&self);\n}\n\n\
         pub trait ISample1Protocol: Send + Sync {\n    fn start(&self);\n}\n\n\
         pub trait ISampleAggregate: Send + Sync {\n    fn execute(&self);\n}\n",
    )
    .unwrap();
    let findings = audit(tmp.path());
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES601" && v == "api_method_not_found")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_else(|| panic!("the missing method must fire; got: {findings:#?}"));
    assert!(
        message.contains("`stop`") && !message.contains("`start`"),
        "only the undeclared name is reported; got: {message}"
    );
}

#[test]
fn aes601_skips_an_api_table_with_no_method_header() {
    let tmp = tempfile::tempdir().unwrap();
    // A table with no `Method` header is a shape the rule cannot read, so it
    // is a parse skip rather than a guessed-at column and a false violation.
    let frd = conforming_frd().replace(
        "| Method | Input | Output | Error | Event | Description |\n| --- | --- | --- | --- | --- | --- |\n| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Single composite entry point. |",
        "| Seam | Behaviour |\n| --- | --- |\n| The auditor | Does the work. |",
    );
    write_workspace(tmp.path(), &frd);
    write_protocol_module(tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "api_method_not_found"),
        "a table with no Method column yields no finding; got: {findings:#?}"
    );
}

#[test]
fn aes601_ignores_an_api_table_inside_a_fenced_block() {
    let tmp = tempfile::tempdir().unwrap();
    // A fenced example showing a hypothetical table cannot make the document
    // promise a method the code does not declare.
    let frd = conforming_frd().replace(
        "## Integration Points",
        "```markdown\n### Protocol API\n\n| Method | Input |\n| --- | --- |\n| `never_declared` | `DocRequest` |\n```\n\n## Integration Points",
    );
    write_workspace(tmp.path(), &frd);
    write_protocol_module(tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "api_method_not_found"),
        "a fenced block is an example, not a promise; got: {findings:#?}"
    );
}

#[test]
fn aes601_anchors_the_method_finding_to_the_table_row() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Single composite entry point. |",
        "| `audit_purity` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Promised but never declared. |",
    );
    write_workspace(tmp.path(), &frd);
    write_protocol_module(tmp.path(), 2);
    let frd_text = fs::read_to_string(tmp.path().join("crates/sample/FRD.md")).unwrap();
    let row_line = frd_text
        .lines()
        .position(|l| l.contains("`audit_purity`"))
        .unwrap()
        + 1;
    let findings = audit(tmp.path());
    let (_, _, message, _) = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES601" && v == "api_method_not_found")
        .expect("the promised method must fire");
    assert!(
        message.contains(&format!("line {row_line}")),
        "the finding must anchor to the row's own line {row_line}; got: {message}"
    );
}

// ── Issue #341: DocFinding carries line and severity ─────────────────

/// Run the audit and keep the raw findings so schema fields are inspectable.
fn audit_findings(root: &Path) -> Vec<shared_doc_rules::DocFinding> {
    let orchestrator = RootDocRulesContainer::orchestrator();
    match orchestrator.execute(DocRequest::audit_all(root)) {
        DocResponse::Findings { findings } => findings,
    }
}

#[test]
fn findings_carry_a_1_based_line_and_high_severity() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace("### FR-SAMPLE-001:", "### FR-001:");
    write_workspace(tmp.path(), &frd);
    let findings = audit_findings(tmp.path());
    let finding = findings
        .iter()
        .find(|f| f.violation_type == "id_missing_feature_prefix")
        .expect("the bare FR id must fire");
    assert!(
        finding.line > 0,
        "line-anchored findings must carry a 1-based line; got {}",
        finding.line
    );
    assert!(
        finding.message.contains(&format!("line {}", finding.line)),
        "the message must name the anchored line; got: {}",
        finding.message
    );
    assert_eq!(
        finding.severity,
        shared_common::Severity::HIGH,
        "AES6xx rules are HIGH per RULES_AES"
    );
}

#[test]
fn document_level_findings_use_line_zero() {
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(tmp.path(), "## Security\n\n- Be safe.\n");
    let findings = audit_findings(tmp.path());
    let h1 = findings
        .iter()
        .find(|f| f.violation_type == "h1_count")
        .expect("a document without an H1 must fire h1_count");
    assert!(
        h1.line == 0,
        "document-level findings use line 0; got {}",
        h1.line
    );
    assert_eq!(h1.severity, shared_common::Severity::HIGH);
}

#[test]
fn findings_map_to_violation_items() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace("### FR-SAMPLE-001:", "### FR-001:");
    write_workspace(tmp.path(), &frd);
    let findings = audit_findings(tmp.path());
    let finding = findings
        .iter()
        .find(|f| f.violation_type == "id_missing_feature_prefix")
        .expect("the bare FR id must fire");
    let item = finding.to_violation_item(tmp.path());
    assert_eq!(item.code.code(), "AES601");
    assert_eq!(item.line.value(), finding.line as i64);
    assert_eq!(item.severity, finding.severity);
    assert!(
        item.message
            .value()
            .contains(&format!("line {}", finding.line))
    );
}

#[test]
fn violation_items_reach_the_sarif_path() {
    // Two findings: a line-anchored one and a document-level one. Both must
    // survive the DocFinding -> ViolationItem -> JSON-object round trip that
    // the SARIF/JSON renderers consume via `ViolationItem::from_json_obj`.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(tmp.path(), "## Security\n\n- Be safe.\n");
    let findings = audit_findings(tmp.path());
    assert!(!findings.is_empty());
    for finding in &findings {
        let item = finding.to_violation_item(tmp.path());
        let json = serde_json::json!({
            "code": item.code.code(),
            "file": item.file.value(),
            "line": item.line.value(),
            "column": item.column.value(),
            "message": item.message.value(),
            "severity": item.severity.to_string(),
        });
        let restored = shared_common::ViolationItem::from_json_obj(&json)
            .expect("the mapped item must round-trip through the JSON renderer shape");
        assert_eq!(restored.line, item.line);
        assert_eq!(restored.severity, item.severity);
    }
}

// ── AES601: alternate layouts, nested modules, and FR-ID casing ─────────

#[test]
fn aes607_fires_for_a_modules_layout_feature() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace_in_layout("modules", tmp.path(), &frd_with_fr_count(3));
    write_protocol_module_in_layout("modules", tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "protocol_count_mismatch"),
        "3 requirements against 2 protocol classes under modules/ must fire; \
         got: {findings:#?}"
    );
}

#[test]
fn aes607_fires_for_a_packages_layout_feature() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace_in_layout("packages", tmp.path(), &frd_with_fr_count(3));
    write_protocol_module_in_layout("packages", tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "protocol_count_mismatch"),
        "3 requirements against 2 protocol classes under packages/ must fire; \
         got: {findings:#?}"
    );
}

#[test]
fn aes607_stays_silent_for_aligned_counts_in_modules_layout() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace_in_layout("modules", tmp.path(), &frd_with_fr_count(2));
    write_protocol_module_in_layout("modules", tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "protocol_count_mismatch"),
        "aligned counts under modules/ must stay silent; got: {findings:#?}"
    );
}

#[test]
fn aes607_counts_protocol_traits_in_nested_subdirectories() {
    let tmp = tempfile::tempdir().unwrap();
    // Two requirements, two protocol classes — but one class sits in a
    // nested subdirectory of the shared module. The recursive walk must
    // count both, so the check stays silent.
    write_workspace(tmp.path(), &frd_with_fr_count(2));
    write_nested_protocol_module(tmp.path());
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "protocol_count_mismatch"),
        "nested protocol classes must be counted like top-level ones; got: {findings:#?}"
    );
}

#[test]
fn aes607_fires_when_nested_traits_are_undercounted_by_the_doc() {
    let tmp = tempfile::tempdir().unwrap();
    // One requirement against two nested classes: the doc is the smaller
    // side, so the mismatch fires in the "code has more" direction.
    write_workspace(tmp.path(), &frd_with_fr_count(1));
    write_nested_protocol_module(tmp.path());
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "protocol_count_mismatch"),
        "1 requirement against 2 nested classes must fire; got: {findings:#?}"
    );
}

#[test]
fn aes601_accepts_lower_case_feature_ids() {
    let tmp = tempfile::tempdir().unwrap();
    // The shared FR-ID pattern accepts mixed case; a lowercase feature
    // prefix must not fire `id_missing_feature_prefix` and must still be
    // counted for parity.
    let frd = conforming_frd().replace("FR-SAMPLE-", "fr-sample-");
    write_workspace(tmp.path(), &frd);
    write_protocol_module(tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "id_missing_feature_prefix"),
        "lowercase FR IDs are accepted by the shared pattern; got: {findings:#?}"
    );
}

#[test]
fn aes601_counts_mixed_case_feature_ids_for_parity() {
    let tmp = tempfile::tempdir().unwrap();
    // `FrSample` is mixed case: the shared pattern accepts it, so the two
    // headings are counted and 2 requirements against 2 classes stay
    // aligned. If the pattern ever regressed to `[A-Z0-9]` only, the
    // counter would report 0 requirements and this test would fire.
    let frd = conforming_frd().replace("FR-SAMPLE-", "FR-FrSample-");
    write_workspace(tmp.path(), &frd);
    write_protocol_module(tmp.path(), 2);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "protocol_count_mismatch")
            && !has(&findings, "AES601", "id_missing_feature_prefix"),
        "mixed-case FR IDs are accepted and counted; got: {findings:#?}"
    );
}

#[test]
fn aes601_root_level_frd_is_audited() {
    let tmp = tempfile::tempdir().unwrap();
    // A FRD at the workspace root is a valid feature (the repo dir is the
    // feature name), so the parity check must still run. It has no shared
    // contract module under any layout, so it stays silent.
    write_workspace(tmp.path(), &frd_with_fr_count(1));
    // Move the FRD to the root to simulate a root-level feature.
    let root_frd = tmp.path().join("FRD.md");
    fs::rename(tmp.path().join("crates/sample/FRD.md"), &root_frd).unwrap();
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "protocol_count_mismatch"),
        "a root-level FRD with no contract module must stay silent; got: {findings:#?}"
    );
}

// ── AES605: DESIGN.md heading structure ─────────────────────────────────

/// A DESIGN.md whose H2 set is exactly the template's, so AES605 accepts it.
fn conforming_design() -> String {
    "# UI — DESIGN\n\n\
     ## Brand & Style\n\n\
     | Token | Value | Usage |\n|---|---|---|\n\
     | `--surface-base` | `#000000` | default background |\n\n\
     ## Components\n\n\
     | Variant | Use case | States | Token map |\n|---|---|---|---|\n\
     | `primary` | main action | default / focus | `--color-primary` |\n\n\
     ## Reference\n\n\
     - Backlog: [BACKLOG.md](BACKLOG.md)\n"
        .to_string()
}

fn write_design_md(dir: &Path, text: &str) {
    write_agents_workspace(dir);
    let surface = dir.join("crates/ui");
    fs::create_dir_all(&surface).unwrap();
    fs::write(surface.join("DESIGN.md"), text).unwrap();
    fs::write(surface.join("BACKLOG.md"), conforming_backlog()).unwrap();
}

/// The fenced DESIGN.md template shipped in HOW-TO-MAKE-DESIGN.md.
///
/// Workspace root is two levels above CARGO_MANIFEST_DIR (crates/doc-rules/).
fn howto_design_template() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../shared/skills/aes-docs/references/HOW-TO-MAKE-DESIGN.md");
    let text =
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    let mut body: Vec<&str> = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            if inside {
                break;
            }
            inside = true;
            continue;
        }
        if inside {
            body.push(line);
        }
    }
    assert!(
        !body.is_empty(),
        "HOW-TO-MAKE-DESIGN.md holds no fenced template"
    );
    body.join("\n")
}

#[test]
fn aes605_conforming_design_reports_no_findings() {
    let tmp = tempfile::tempdir().unwrap();
    write_design_md(tmp.path(), &conforming_design());
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES605", "h2_missing") && !has(&findings, "AES605", "h2_unexpected"),
        "conforming DESIGN.md must be accepted; got: {findings:#?}"
    );
}

/// The shipped template must pass its own rule. This is the regression guard
/// for the contract that named "Kind"/"Entry Points"/"States", which no file
/// produced from the template could satisfy.
#[test]
fn aes605_accepts_the_shipped_design_template() {
    let tmp = tempfile::tempdir().unwrap();
    write_design_md(tmp.path(), &howto_design_template());
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES605", "h2_missing") && !has(&findings, "AES605", "h2_unexpected"),
        "a DESIGN.md copied verbatim from HOW-TO-MAKE-DESIGN.md must be accepted; got: {findings:#?}"
    );
}

#[test]
fn aes605_fires_when_required_design_headings_are_absent() {
    let tmp = tempfile::tempdir().unwrap();
    write_design_md(
        tmp.path(),
        "# UI — DESIGN\n\n## Reference\n\n- Backlog: [BACKLOG.md](BACKLOG.md)\n",
    );
    let findings = audit(tmp.path());
    assert!(has(&findings, "AES605", "h2_missing"));
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_missing")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap();
    assert!(
        message.contains("Brand & Style") && message.contains("Components"),
        "message must name the missing DESIGN sections; got: {message}"
    );
}

#[test]
fn aes605_fires_when_design_carries_off_template_h2() {
    let tmp = tempfile::tempdir().unwrap();
    write_design_md(
        tmp.path(),
        &format!("{}\n## History\n\nOld stuff.\n", conforming_design()),
    );
    let findings = audit(tmp.path());
    assert!(has(&findings, "AES605", "h2_unexpected"));
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap();
    assert!(
        message.contains("history"),
        "must name the unexpected section; got: {message}"
    );
}

/// The H2 set is closed, so the surface-behaviour headings an earlier revision
/// of the contract required are now off-template and must be reported.
#[test]
fn aes605_rejects_the_retired_surface_headings() {
    let tmp = tempfile::tempdir().unwrap();
    write_design_md(
        tmp.path(),
        "# UI — DESIGN\n\n\
         ## Kind\n\n`ui` — a surface component.\n\n\
         ## Entry Points\n\n| File | Role |\n|---|---|\n| `x.rs` | entry |\n\n\
         ## States\n\n| State | Condition |\n|---|---|\n| `ready` | idle |\n",
    );
    let findings = audit(tmp.path());
    assert!(has(&findings, "AES605", "h2_unexpected"));
    assert!(
        has(&findings, "AES605", "h2_missing"),
        "a DESIGN.md without the template's H2 set must also report them missing; got: {findings:#?}"
    );
}
