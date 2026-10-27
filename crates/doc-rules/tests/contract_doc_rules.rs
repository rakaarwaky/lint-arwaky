// PURPOSE: contract tests for doc-rules — each invariant must fire on a bad
// document and stay silent on a conforming one. Findings carry a code
// (AES601–AES606) plus a violation_type, and both are asserted.
use doc_rules_lint_arwaky::root_doc_rules_container::RootDocRulesContainer;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;

use std::fs;
use std::path::Path;

/// A FRD that satisfies every structural invariant.
fn conforming_frd() -> String {
    r#"# FRD — sample

## Feature Overview

The feature does the work a surface delegates to.

## Functional Requirements

### FR-SAMPLE-001: Document Invariant Enforcement

| Aspect | Detail |
|---|---|
| Description | Every `.md` document satisfies the AES heading and section contract. |
| Business Rules | Validates FR-ID format, section structure, spec purity, crosslinks, heading structure, and parity. |
| Edge Cases | Root-level legacy `BACKLOG.md` is accepted during migration. |
| Capability | Invariant enforcement |
| Protocol | doc-audit |
| Method | `audit` |
| Input | A workspace root and the list of recognized document paths. |
| Output | `Vec<LintResult>` carrying one finding per invariant violation. |
| Error | Unreadable files produce no findings. |
| Event | None. |
| Test | A conforming workspace reports nothing. |
| Metric | Zero violations from `lint-arwaky-cli check .`. |

### FR-SAMPLE-002: Audit Orchestration

| Aspect | Detail |
|---|---|
| Description | The doc auditor is reachable through a single aggregate entry point dispatching to the invariant checker. |
| Business Rules | The aggregate orchestrates all invariants under one seam so consumers need only invoke `execute`. |
| Edge Cases | A request with an empty root produces no findings rather than an error. |
| Capability | Audit orchestration |
| Protocol | doc-audit |
| Method | `execute` |
| Input | A `DocRequest` describing the workspace root to audit. |
| Output | A `DocResponse` carrying a deduplicated, stable-sorted list of findings. |
| Error | Failures inside the checker are propagated as `DocResponse::Findings`. |
| Event | None. |
| Test | Two runs over the same workspace report the same findings. |
| Metric | One call per request. |

## Public Interface

| Aspect | Detail |
|---|---|
| Agent | doc-auditor |
| Aggregate | doc-orchestrator |
| Method | `execute` |
| Request | `DocRequest` |
| Response | `DocResponse` |

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature.

## API Contract

### Protocol API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Single composite entry point. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Routes to the capability. |
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

| ID | Feature | Priority | State | Dependencies | Next Action | Updated |
|---|---|---|---|---|---|---|
| WS-1 | Docs | P0 | Done | — | — | 2026-09-29 |

## State Definitions

| State | Meaning |
|---|---|
| Done | Shipped. |

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
State: values from root [ROADMAP.md](../../ROADMAP.md) — do not redefine here.
Last Updated: 2026-09-29

## Current Condition

- Done: nothing yet
- In Progress: None
- Blocked: None
- Next Action: get started

## Backlog

| ID | Priority | State | Dependencies | Next Action | Updated |
|---|---:|---|---|---|---|
| FR-SAMPLE-001 | P0 | Ready | — | — | 2026-09-29 |
| FR-SAMPLE-002 | P0 | Ready | — | — | 2026-09-29 |

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
## Runtime\n\n\
- Language: Rust.\n\n\
## Project Quick Facts\n\n\
sample facts.\n\n\
## Pipeline\n\n\
scan → check\n\n\
## Project Structure\n\n\
one crate.\n\n\
## Commands\n\n\
```bash\ncargo nextest run --workspace\n```\n\n\
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
    match orchestrator.execute(DocRequest::audit_all(
        root,
        None,
        shared_doc_rules::StrictModeOverride::UseConfigFile,
    )) {
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
    // The fixture keeps a legacy `## API Contract` for the AES601 method tests,
    // so AES605 reports it as an off-template H2; every other rule must stay
    // silent.
    let unexpected = findings
        .iter()
        .filter(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .count();
    assert_eq!(
        unexpected, 1,
        "the legacy API Contract H2 is the only off-template section; got: {findings:#?}"
    );
    assert!(
        findings
            .iter()
            .filter(|(c, _, _, _)| c != "AES605")
            .all(|(_, v, _, _)| v != "order_violation"),
        "no order finding may fire on a conforming FRD, got: {findings:#?}"
    );
    assert!(
        findings
            .iter()
            .all(|(c, v, _, _)| c != "AES601" && c != "AES602" && c != "AES604"),
        "no FR-format, section-order, or crosslink finding may fire, got: {findings:#?}"
    );
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
            "| Edge Cases | Root-level legacy `BACKLOG.md` is accepted during migration. |\n",
            "",
        )
        .replace("| Event | None. |\n", "");
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    let field_findings: Vec<&str> = findings
        .iter()
        .filter(|(c, v, _, _)| c == "AES601" && v == "field_missing")
        .map(|(_, _, m, _)| m.as_str())
        .collect();
    // The table-form fixture has no bold `**{field}**` markers, so every FR
    // heading reports all twelve FR_FIELDS missing; one finding per FR.
    assert_eq!(
        field_findings.len(),
        2,
        "one finding per FR heading with missing fields: {findings:#?}"
    );
    for message in &field_findings {
        assert!(
            message.contains("Edge Cases") && message.contains("Event"),
            "each finding must name every missing field, got: {message}"
        );
    }
}

// ── AES602: FRD section order ─────────────────────────────────────────

#[test]
fn aes602_fires_when_sections_are_out_of_order() {
    let tmp = tempfile::tempdir().unwrap();
    let scrambled = conforming_frd()
        .replace("## Feature Overview", "## @@MARK@@")
        .replace("## Functional Requirements", "## Feature Overview")
        .replace("## @@MARK@@", "## Functional Requirements");
    write_workspace(tmp.path(), &scrambled);
    assert!(has(&audit(tmp.path()), "AES602", "order_violation"));
}

#[test]
fn aes602_fires_when_public_interface_precedes_feature_overview() {
    let tmp = tempfile::tempdir().unwrap();
    let scrambled = conforming_frd()
        .replace("## Feature Overview", "## @@MARK@@")
        .replace("## Public Interface", "## Feature Overview")
        .replace("## @@MARK@@", "## Public Interface");
    write_workspace(tmp.path(), &scrambled);
    assert!(has(&audit(tmp.path()), "AES602", "order_violation"));
}

#[test]
fn aes602_accepts_free_form_h3_headings() {
    // H3s are free-form under the new layout: the order check reads level-2
    // headings only, so an invented H3 is not a section-structure defect.
    let frd = conforming_frd().replace(
        "## Public Interface",
        "### Dispatch details\n\nProse about the dispatch.\n\n## Public Interface",
    );
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES602", "order_violation"),
        "a free-form H3 must not fire AES602, got: {findings:#?}"
    );
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
| ID | Priority | State | Dependencies | Next Action | Updated |\n\
|---|---:|---|---|---|---|\n\
| SAM-01 | P0 | Ready | — | — | 2026-09-29 |\n\n\
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
    // Runtime, Project Quick Facts, and most others are missing from the required set.
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
        message.contains("Project Quick Facts") || message.contains("Pipeline"),
        "the message must name the absent section(s); got: {message}"
    );
}

#[test]
fn aes605_allows_extra_and_free_h3_headings() {
    // Extra H3 headings are allowed; required H2s must still be present.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "# Sample AGENTS.md

## Runtime

- Language: Rust.

## Project Quick Facts

sample facts.

## Pipeline

scan → check
lint
run by CI

## Project Structure

one crate.

## Commands

```bash
true
```

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
        "# Sample AGENTS.md\n\n\
## Runtime\n\
- Language: Rust.\n\n\
## Commands\n\
```bash\ntrue\n```\n\n\
## Related Documents\n\
- [PRD.md](PRD.md).",
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
        message.contains("Project Quick Facts") && message.contains("Pipeline"),
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
        "# Sample AGENTS.md\n\n\
## Runtime\n\
- Language: Rust.\n\n\
## Architecture\n\
See [ARCHITECTURE.md](ARCHITECTURE.md) for the layer specification.\n\n\
## Contributing\n\
See [CONTRIBUTING.md](CONTRIBUTING.md).\n\n\
## License\n\
MIT. See [LICENSE](LICENSE).\n\n\
## Random Notes\n\
Anything the author wanted to jot down.\n\n\
## Commands\n\
```bash\ntrue\n```\n\n\
## Related Documents\n\
- [PRD.md](PRD.md).\n",
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
    // The AGENTS H2 set is closed: the template's exact H2s and nothing else.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "# Sample AGENTS.md

## Runtime

- Language: Rust.

## Project Quick Facts

sample facts.

## Pipeline

scan → check
lint
run by CI

## Project Structure

one crate.

## Commands

```bash
true
```

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

/// Layout is documented in AGENTS.md. A README that repeats it is a second
/// source of truth, so the section is off-template and must be reported even
/// though it is a real section of this project.
#[test]
fn aes605_rejects_project_structure_in_a_readme() {
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
"#;
    let findings = audit_root_doc("README.md", text);
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "Project Structure belongs to AGENTS.md, not README.md; got: {findings:#?}"
    );
    let (message, fix) = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, fix)| (m.as_str(), fix.as_str()))
        .unwrap_or_default();
    assert!(
        message.contains("project structure") && fix.contains("level-3"),
        "message must name the heading and the fix must name the remedy; got: {message} / {fix}"
    );
    assert!(
        !has(&findings, "AES605", "h2_missing"),
        "every remaining README section is present; got: {findings:#?}"
    );
}

/// The README contract is read from the rule itself, so a future edit to
/// `DOC_HEADING_CONTRACTS` cannot silently leave a shipped section required
/// while the template omits it.
#[test]
fn aes605_accepts_a_readme_with_the_contract_sections_only() {
    let (_, required, _) = shared_doc_rules::taxonomy_doc_rules_constant::DOC_HEADING_CONTRACTS
        .iter()
        .find(|(name, _, _)| *name == "README.md")
        .expect("README.md must have a heading contract");
    let mut readme = String::from("# Sample README\n\n");
    for heading in *required {
        readme.push_str(&format!("## {heading}\n\nBody.\n\n"));
    }
    let findings = audit_root_doc("README.md", &readme);
    assert!(
        !has(&findings, "AES605", "h2_missing") && !has(&findings, "AES605", "h2_unexpected"),
        "a README built from the contract must be accepted; got: {findings:#?}"
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
             | Aspect | Detail |\n\
             |---|---|\n\
             | Description | The feature performs a second responsibility. |\n\
             | Business Rules | The capability is stateless. |\n\
             | Edge Cases | An absent input yields a default. |\n"
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

## Feature Overview

The feature does the work.

## Functional Requirements

### FR-SAMPLE-001: Document one thing

| Aspect | Detail |
|---|---|
| Description | Count the first. |
| Business Rules | Always one. |
| Edge Cases | None. |

### FR-SAMPLE-002: Document two things

| Aspect | Detail |
|---|---|
| Description | Count the second. |
| Business Rules | Always two. |
| Edge Cases | None. |

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
    match orchestrator.execute(DocRequest::audit_all(
        root,
        None,
        shared_doc_rules::StrictModeOverride::UseConfigFile,
    )) {
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

// ── AES601: Python shared-tree layout (issue #1062) ─────────────────────

/// A Python monorepo keeps every feature's protocol ABCs in a shared tree
/// (`modules/shared/src/<feature>/contract_*.py`), not in a per-feature
/// crate. The detector must count `@abc.abstractmethod` classes there so
/// the FRD requirement count matches the code count instead of reading as
/// 0.
#[test]
fn aes601_counts_python_abc_classes_in_a_subfeature_shared_tree() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // Feature folder: `modules/animator/` with FRD + BACKLOG + agent file.
    let feature = root.join("modules").join("animator");
    fs::create_dir_all(feature.join("src")).unwrap();
    let frd = frd_with_fr_count(3).replace("FR-SAMPLE-", "FR-ANIMATOR-");
    fs::write(feature.join("FRD.md"), &frd).unwrap();
    fs::write(feature.join("BACKLOG.md"), conforming_backlog()).unwrap();
    fs::write(
        feature.join("src").join("agent_animator_orchestrator.py"),
        "class AnimatorOrchestrator: ...\n",
    )
    .unwrap();
    // Subfeature-keyed shared tree: `modules/shared/src/animator/`.
    let module = root
        .join("modules")
        .join("shared")
        .join("src")
        .join("animator");
    fs::create_dir_all(&module).unwrap();
    fs::write(
        module.join("contract_animator_protocol.py"),
        "import abc\n\
         \n\
         @abc.abstractmethod\n\
         class IAnimatorOneProtocol(abc.ABC):\n\
             @abc.abstractmethod\n\
             def animate(self) -> None: ...\n\
         \n\
         @abc.abstractmethod\n\
         class IAnimatorTwoProtocol(abc.ABC):\n\
             @abc.abstractmethod\n\
             def animate(self) -> None: ...\n\
         \n\
         @abc.abstractmethod\n\
         class IAnimatorThreeProtocol(abc.ABC):\n\
             @abc.abstractmethod\n\
             def animate(self) -> None: ...\n\
         \n\
         class IAnimatorAggregate:\n\
             def execute(self) -> None: ...\n",
    )
    .unwrap();
    let findings = audit(root);
    assert!(
        !has(&findings, "AES601", "protocol_count_mismatch"),
        "3 requirements and 3 Python ABC protocol classes + 1 aggregate must align; \
         got: {findings:#?}"
    );
}

/// A Python shared tree with fewer ABC classes than the FRD declares must
/// still fire `protocol_count_mismatch` in the "code has fewer" direction.
#[test]
fn aes601_fires_when_python_abc_count_is_below_the_fr_count() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let feature = root.join("modules").join("animator");
    fs::create_dir_all(feature.join("src")).unwrap();
    let frd = frd_with_fr_count(3).replace("FR-SAMPLE-", "FR-ANIMATOR-");
    fs::write(feature.join("FRD.md"), &frd).unwrap();
    fs::write(feature.join("BACKLOG.md"), conforming_backlog()).unwrap();
    fs::write(
        feature.join("src").join("agent_animator_orchestrator.py"),
        "class AnimatorOrchestrator: ...\n",
    )
    .unwrap();
    let module = root
        .join("modules")
        .join("shared")
        .join("src")
        .join("animator");
    fs::create_dir_all(&module).unwrap();
    fs::write(
        module.join("contract_animator_protocol.py"),
        "import abc\n\
         \n\
         @abc.abstractmethod\n\
         class IAnimatorOneProtocol(abc.ABC):\n\
             @abc.abstractmethod\n\
             def animate(self) -> None: ...\n\
         \n\
         @abc.abstractmethod\n\
         class IAnimatorTwoProtocol(abc.ABC):\n\
             @abc.abstractmethod\n\
             def animate(self) -> None: ...\n",
    )
    .unwrap();
    let findings = audit(root);
    assert!(
        has(&findings, "AES601", "protocol_count_mismatch"),
        "3 requirements against 2 Python ABC protocol classes must fire; \
         got: {findings:#?}"
    );
}

// ── AES601: config suppression (issue #1062) ───────────────────────────

/// `architecture.rules.AES601.enabled: false` must put `AES601` in the
/// suppressed set the dispatcher's exact-match helper consults, so the doc
/// audit's AES601 findings are dropped when the rule is disabled.
#[test]
fn architecture_rules_disabled_code_enters_the_suppressed_set() {
    let content = "architecture:\n  rules:\n    AES601:\n      enabled: false\n";
    let (config, _) =
        shared_config_system::utility_config_parser::parse_config_yaml_with_warnings(content);
    let suppressed = dispatcher::surface_suppression_hook::collect_suppressed_codes(&config);
    assert!(
        dispatcher::surface_suppression_hook::is_rule_suppressed(&suppressed, "AES601"),
        "AES601 must be suppressed when architecture.rules.AES601.enabled is false; got: {suppressed:#?}"
    );
}

/// `ignored_rules: [AES601]` in `lint_arwaky.config.yaml` must put `AES601`
/// in the suppressed set via the shared YAML parser, exactly as the external
/// lint path consumes the same key.
#[test]
fn ignored_rules_entry_enters_the_suppressed_set() {
    let content = "ignored_rules:\n  - AES601\n";
    let suppressed =
        shared_config_system::utility_config_parser::parse_ignored_rules_from_yaml(content);
    assert!(
        dispatcher::surface_suppression_hook::is_rule_suppressed(&suppressed, "AES601"),
        "AES601 must appear in the suppressed set when ignored_rules lists it; got: {suppressed:#?}"
    );
}

/// `architecture.rules.AES702.enabled: false` must put `AES702` in the
/// suppressed set so the structure audit can suppress AES702 findings.
#[test]
fn architecture_rules_disabled_code_aes702_enters_the_suppressed_set() {
    let content = "architecture:\n  rules:\n    AES702:\n      enabled: false\n";
    let (config, _) =
        shared_config_system::utility_config_parser::parse_config_yaml_with_warnings(content);
    let suppressed = dispatcher::surface_suppression_hook::collect_suppressed_codes(&config);
    assert!(
        dispatcher::surface_suppression_hook::is_rule_suppressed(&suppressed, "AES702"),
        "AES702 must appear in the suppressed set when disabled; got: {suppressed:#?}"
    );
}

// ── AES605: DESIGN.md heading structure ─────────────────────────────────

/// A DESIGN.md whose H2 set is exactly the template's, so AES605 accepts it.
fn conforming_design() -> String {
    "# Design — ui\n\n\
     ## Design Overview\n\n\
     The surface is a terminal UI built on token rows.\n\n\
     ## Design Requirements\n\n\
     ### DS-DISP-001: Primary Button\n\n\
     | Variant | Use case | Token | Value |\n|---|---|---|---|\n\
     | `primary` | main action | `--color-primary` | `#000000` |\n\n\
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
        message.contains("Design Overview") && message.contains("Design Requirements"),
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

// ── AES601: FR-ID ↔ BACKLOG ID alignment ────────────────────────────────

/// Set up a conforming `crates/sample` workspace carrying `frd`, then point
/// its BACKLOG at `backlog` (removing the file when `None`), so only the
/// pair under test contributes findings.
fn write_frd_backlog_pair(dir: &Path, frd: &str, backlog: Option<&str>) {
    write_workspace_in_layout("crates", dir, frd);
    let backlog_path = dir.join("crates/sample/BACKLOG.md");
    match backlog {
        Some(backlog) => fs::write(&backlog_path, backlog).unwrap(),
        None => fs::remove_file(&backlog_path).unwrap(),
    }
}

#[test]
fn backlog_id_mismatch_reports_both_directions() {
    let tmp = tempfile::tempdir().unwrap();
    // FRD declares 001 and 003; BACKLOG carries 001 and a bogus 002:
    // 002 is not an FR-ID, and 003 has no row.
    let frd = conforming_frd().replace("### FR-SAMPLE-002:", "### FR-SAMPLE-003:");
    let backlog = conforming_backlog().replace(
        "| FR-SAMPLE-001 | P0 | Ready | — | — | 2026-09-29 |",
        "| FR-SAMPLE-002 | P0 | Ready | — | — | 2026-09-29 |",
    );
    write_frd_backlog_pair(tmp.path(), &frd, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "backlog_id_not_fr")
            && has(&findings, "AES601", "backlog_id_missing"),
        "one misaligned pair reports both directions; got: {findings:#?}"
    );
    // The not-fr finding anchors to the BACKLOG row, the missing finding to
    // the `## Backlog` heading; both carry the why and fix of their type.
    let raw = audit_findings(tmp.path());
    let not_fr = raw
        .iter()
        .find(|f| f.violation_type == "backlog_id_not_fr")
        .expect("the non-FR row must fire");
    let backlog_text = fs::read_to_string(tmp.path().join("crates/sample/BACKLOG.md")).unwrap();
    let row_line = backlog_text
        .lines()
        .position(|l| l.starts_with("| FR-SAMPLE-002"))
        .unwrap()
        + 1;
    let heading_line = backlog_text
        .lines()
        .position(|l| l.starts_with("## Backlog"))
        .unwrap()
        + 1;
    assert_eq!(
        not_fr.line, row_line,
        "`backlog_id_not_fr` anchors to the row's own line; got {}",
        not_fr.line
    );
    assert!(not_fr.why.contains("traceable"), "got: {}", not_fr.why);
    assert!(
        not_fr.fix.contains("FR-SAMPLE-001") && not_fr.fix.contains("FR-SAMPLE-003"),
        "the fix must list the FRD's valid FR-IDs; got: {}",
        not_fr.fix
    );
    let missing = raw
        .iter()
        .find(|f| f.violation_type == "backlog_id_missing" && f.message.contains("FR-SAMPLE-003"))
        .expect("the untracked FR-ID must fire");
    assert_eq!(
        missing.line, heading_line,
        "`backlog_id_missing` anchors to the `## Backlog` heading; got {}",
        missing.line
    );
    assert!(
        missing.fix.contains("`FR-SAMPLE-003`"),
        "the fix must name the untracked FR-ID; got: {}",
        missing.fix
    );
}

#[test]
fn backlog_id_full_coverage_reports_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    write_frd_backlog_pair(tmp.path(), &conforming_frd(), Some(conforming_backlog()));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "backlog_id_not_fr")
            && !has(&findings, "AES601", "backlog_id_missing"),
        "every FR-ID has a BACKLOG row and every row is an FR-ID; got: {findings:#?}"
    );
}

#[test]
fn backlog_without_frd_is_silent() {
    let tmp = tempfile::tempdir().unwrap();
    // An FRD that declares no FR-IDs has nothing the BACKLOG can align to.
    let frd = conforming_frd()
        .replace(
            "### FR-SAMPLE-001: Document Invariant Enforcement",
            "### Something Else",
        )
        .replace(
            "### FR-SAMPLE-002: Audit Orchestration",
            "### Another Thing",
        );
    write_frd_backlog_pair(tmp.path(), &frd, Some(conforming_backlog()));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "backlog_id_not_fr")
            && !has(&findings, "AES601", "backlog_id_missing"),
        "an FRD with no FR-IDs stays silent on alignment; got: {findings:#?}"
    );
}

#[test]
fn backlog_without_backlog_section_is_silent() {
    let tmp = tempfile::tempdir().unwrap();
    // No `## Backlog` section means there is no row set to align against.
    let backlog = conforming_backlog()
        .replace("## Backlog\n", "## Other Section\n")
        .replace("| FR-SAMPLE-001 | P0 | Ready | — | — | 2026-09-29 |\n", "");
    write_frd_backlog_pair(tmp.path(), &conforming_frd(), Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "backlog_id_not_fr")
            && !has(&findings, "AES601", "backlog_id_missing"),
        "a BACKLOG without a `## Backlog` section stays silent; got: {findings:#?}"
    );
}

// ── AES601: DS/DT ↔ BACKLOG ID alignment ─────────────────────────────────

/// A DESIGN.md carrying a qualified `DS-DISP-NNN:` heading under
/// `## Design Requirements`.
fn conforming_design_with_ds() -> String {
    "# Design — ui\n\n\
     ## Design Overview\n\n\
     The surface is a terminal UI built on token rows.\n\n\
     ## Design Requirements\n\n\
     ### DS-DISP-001: Primary Button\n\n\
     | Variant | Use case | Token | Value |\n|---|---|---|---|\n\
     | `primary` | main action | `--color-primary` | `#000000` |\n\n\
     ## Reference\n\n\
     - Backlog: [BACKLOG.md](BACKLOG.md)\n"
        .to_string()
}

/// A DATA.md carrying a qualified H2 `## DT-SHAR-NNN: <title>` item.
fn conforming_data_with_dt() -> String {
    "# DATA — shared\n\n\
     ## Data Overview\n\n\
     The shared kernel manages value objects.\n\n\
     ## DT-SHAR-001: Severity\n\n\
     | Type  | Description |\n|-------|-------------|\n\
     | vo    | Lint finding severity level |\n\n\
     ## Reference\n\n\
     - PRD: [PRD.md](../../PRD.md)\n\
     - Backlog: [BACKLOG.md](BACKLOG.md)\n"
        .to_string()
}

/// A BACKLOG whose `## Backlog` table carries the given row ids.
fn backlog_with_rows(rows: &[&str]) -> String {
    let rows = rows
        .iter()
        .map(|id| format!("| {id} | P0 | Ready | — | — | 2026-09-29 |"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# Feature Backlog: Sample\n\n## Backlog\n\n\
         | ID | Priority | State | Dependencies | Next Action | Updated |\n\
         |---|---:|---|---|---|---|\n{rows}\n"
    )
}

/// Write a surface folder `crates/sample` holding `design` and `backlog`,
/// so only the DESIGN/BACKLOG pair contributes findings.
fn write_design_backlog_pair(dir: &Path, design: &str, backlog: Option<&str>) {
    let feature = dir.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    fs::write(feature.join("DESIGN.md"), design).unwrap();
    fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();
    if let Some(backlog) = backlog {
        fs::write(feature.join("BACKLOG.md"), backlog).unwrap();
    }
}

/// Write a kernel folder `crates/shared` holding `data` and `backlog`,
/// so only the DATA/BACKLOG pair contributes findings.
fn write_data_backlog_pair(dir: &Path, data: &str, backlog: Option<&str>) {
    let kernel = dir.join("crates/shared");
    fs::create_dir_all(kernel.join("src")).unwrap();
    fs::write(kernel.join("DATA.md"), data).unwrap();
    if let Some(backlog) = backlog {
        fs::write(kernel.join("BACKLOG.md"), backlog).unwrap();
    }
}

#[test]
fn ds_full_coverage_reports_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let design = conforming_design_with_ds();
    let backlog = backlog_with_rows(&["DS-DISP-001"]);
    write_design_backlog_pair(tmp.path(), &design, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "ds_backlog_id_not_ds")
            && !has(&findings, "AES601", "ds_backlog_id_missing"),
        "every DS-ID has a BACKLOG row and every row is a DS-ID; got: {findings:#?}"
    );
}

#[test]
fn ds_mismatch_reports_both_directions() {
    let tmp = tempfile::tempdir().unwrap();
    // DESIGN declares DS-DISP-001 and DS-DISP-003; BACKLOG carries DS-DISP-001
    // and a bogus DS-DISP-002.
    let design = conforming_design_with_ds().replace(
        "### DS-DISP-001: Primary Button\n\n",
        "### DS-DISP-001: Primary Button\n\n### DS-DISP-003: Card\n\n",
    );
    let backlog = backlog_with_rows(&["DS-DISP-001", "DS-DISP-002"]);
    write_design_backlog_pair(tmp.path(), &design, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "ds_backlog_id_not_ds")
            && has(&findings, "AES601", "ds_backlog_id_missing"),
        "one misaligned DS pair reports both directions; got: {findings:#?}"
    );
}

#[test]
fn ds_empty_heading_set_fires_not_ds_on_every_legacy_row() {
    let tmp = tempfile::tempdir().unwrap();
    // DESIGN declares no DS-IDs; every legacy BACKLOG row must fire ds_backlog_id_not_ds.
    let design = "# Design — ui\n\n## Design Overview\n\n- base\n\n## Reference\n\n- Backlog: [BACKLOG.md](BACKLOG.md)\n";
    let backlog = backlog_with_rows(&["DISP-01", "DISP-02"]);
    write_design_backlog_pair(tmp.path(), design, Some(&backlog));
    let findings = audit(tmp.path());
    let count = findings
        .iter()
        .filter(|(c, v, _, _)| c == "AES601" && v == "ds_backlog_id_not_ds")
        .count();
    assert_eq!(
        count, 2,
        "every legacy row fires ds_backlog_id_not_ds; got: {findings:#?}"
    );
}

#[test]
fn ds_backlog_without_backlog_section_is_silent() {
    let tmp = tempfile::tempdir().unwrap();
    let design = conforming_design_with_ds();
    let backlog = backlog_with_rows(&["DS-DISP-001"]).replace("## Backlog\n", "## Other\n");
    write_design_backlog_pair(tmp.path(), &design, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "ds_backlog_id_not_ds")
            && !has(&findings, "AES601", "ds_backlog_id_missing"),
        "a BACKLOG without a `## Backlog` section stays silent; got: {findings:#?}"
    );
}

#[test]
fn dt_full_coverage_reports_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let data = conforming_data_with_dt();
    let backlog = backlog_with_rows(&["DT-SHAR-001"]);
    write_data_backlog_pair(tmp.path(), &data, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "dt_backlog_id_not_dt")
            && !has(&findings, "AES601", "dt_backlog_id_missing"),
        "every DT-ID has a BACKLOG row and every row is a DT-ID; got: {findings:#?}"
    );
}

#[test]
fn dt_mismatch_reports_both_directions() {
    let tmp = tempfile::tempdir().unwrap();
    let data = conforming_data_with_dt().replace(
        "## DT-SHAR-001: Severity\n\n",
        "## DT-SHAR-001: Severity\n\n## DT-SHAR-003: MemberDelta\n\n",
    );
    let backlog = backlog_with_rows(&["DT-SHAR-001", "DT-SHAR-002"]);
    write_data_backlog_pair(tmp.path(), &data, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "dt_backlog_id_not_dt")
            && has(&findings, "AES601", "dt_backlog_id_missing"),
        "one misaligned DT pair reports both directions; got: {findings:#?}"
    );
}

#[test]
fn dt_rejects_do_headings() {
    let tmp = tempfile::tempdir().unwrap();
    // `DO-001` is not an accepted DT-ID; the DATA doc declares no DT headings,
    // so the legacy DO row fires dt_backlog_id_not_dt.
    let data = conforming_data_with_dt().replace("## DT-SHAR-001: Severity\n\n", "");
    let backlog = backlog_with_rows(&["DO-001"]);
    write_data_backlog_pair(tmp.path(), &data, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "dt_backlog_id_not_dt"),
        "a DO-001 BACKLOG row with no DT heading must fire dt_backlog_id_not_dt; got: {findings:#?}"
    );
}

#[test]
fn dt_empty_heading_set_fires_not_dt_on_every_legacy_row() {
    let tmp = tempfile::tempdir().unwrap();
    let data = "# DATA — shared\n\n## Reference\n\n- Backlog: [BACKLOG.md](BACKLOG.md)\n\n## Data Overview\n\nnone.\n";
    let backlog = backlog_with_rows(&["SHAR-01", "SHAR-02"]);
    write_data_backlog_pair(tmp.path(), data, Some(&backlog));
    let findings = audit(tmp.path());
    let count = findings
        .iter()
        .filter(|(c, v, _, _)| c == "AES601" && v == "dt_backlog_id_not_dt")
        .count();
    assert_eq!(
        count, 2,
        "every legacy row fires dt_backlog_id_not_dt; got: {findings:#?}"
    );
}

/// A bare `DS-001:` heading (digits only, no NAME qualifier) is no longer an
/// accepted DS-ID, so the legacy BACKLOG row is not a valid item ID and the
/// document declares no item at all.
#[test]
fn ds_bare_id_heading_is_no_longer_accepted() {
    let tmp = tempfile::tempdir().unwrap();
    let design = "# Design — ui\n\n\
     ## Design Overview\n\n\
     The surface is a terminal UI built on token rows.\n\n\
     ## Design Requirements\n\n\
     ### DS-001: Primary Button\n\n\
     | Variant | Use case | Token | Value |\n|---|---|---|---|\n\
     | `primary` | main action | `--color-primary` | `#000000` |\n\n\
     ## Reference\n\n\
     - Backlog: [BACKLOG.md](BACKLOG.md)\n";
    let backlog = backlog_with_rows(&["DS-001"]);
    write_design_backlog_pair(tmp.path(), design, Some(&backlog));
    let findings = audit(tmp.path());
    // The bare heading is not collected, so the `DS-001` BACKLOG row is not a
    // valid item ID (direction b) and no item ID owes a BACKLOG row (no (a)).
    assert!(
        has(&findings, "AES601", "ds_backlog_id_not_ds"),
        "a bare `DS-001:` heading must not be accepted, so the row fires ds_backlog_id_not_ds; got: {findings:#?}"
    );
}

/// A bare `DT-001:` heading (digits only, no NAME qualifier) is no longer an
/// accepted DT-ID.
#[test]
fn dt_bare_id_heading_is_no_longer_accepted() {
    let tmp = tempfile::tempdir().unwrap();
    let data = "# DATA — shared\n\n\
     ## Data Overview\n\n\
     The shared kernel manages value objects.\n\n\
     ## DT-001: Severity\n\n\
     | Type  | Description |\n|-------|-------------|\n\
     | vo    | Lint finding severity level |\n\n\
     ## Reference\n\n\
     - Backlog: [BACKLOG.md](BACKLOG.md)\n";
    let backlog = backlog_with_rows(&["DT-001"]);
    write_data_backlog_pair(tmp.path(), data, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "dt_backlog_id_not_dt"),
        "a bare `DT-001:` heading must not be accepted, so the row fires dt_backlog_id_not_dt; got: {findings:#?}"
    );
}

/// The DATA_DOC H2 contract now requires `Data Overview` + `Reference` and
/// allows only `DT-` item H2s, so a DATA.md without those required H2s fires
/// `h2_missing`.
#[test]
fn aes605_data_doc_requires_data_overview_and_reference() {
    let tmp = tempfile::tempdir().unwrap();
    let kernel = tmp.path().join("crates/shared");
    fs::create_dir_all(kernel.join("src")).unwrap();
    fs::write(
        kernel.join("DATA.md"),
        "# DATA — shared\n\n## Data Domain\n\n\
         ## Assumptions & Constraints\n\n- none\n",
    )
    .unwrap();
    let findings = audit(tmp.path());
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_missing")
        .map(|(_, _, m, _)| m.as_str())
        .expect("the DATA doc must owe its required H2s");
    assert!(
        message.contains("Data Overview") && message.contains("Reference"),
        "the DATA doc must owe Data Overview and Reference; got: {message}"
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
