// PURPOSE: contract tests for doc-rules — each invariant must fire on a bad
// document and stay silent on a conforming one. Findings carry a code
// (AES601–AES606) plus a violation_type, and both are asserted.
use doc_rules_lint_arwaky::root_doc_rules_container::RootDocRulesContainer;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;

use std::fs;
use std::path::Path;

/// A FRD that satisfies every structural invariant: the FRD contract H2 set
/// (`Feature Overview`, `Functional Requirements`, `Public Interface`,
/// `Reference`) in FRD_SECTION_ORDER, each entry table carrying the twelve
/// FR_FIELDS rows, and a Reference table linking every artifact the AES604
/// required set names.
fn conforming_frd() -> String {
    r#"# FRD — sample

## Feature Overview

The feature does the work a surface delegates to.

## Functional Requirements

### FR-SAMPLE-001: Document Invariant Enforcement

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | Every `.md` document satisfies the AES heading and section contract. |
| Business Rules | Validates FR-ID format, section structure, spec purity, crosslinks, heading structure, and parity. |
| Edge Cases     | Root-level legacy `BACKLOG.md` is accepted during migration. |
| Capability     | doc_rules::DocStructure                     |
| Protocol       | doc_rules::DocStructure                     |
| Method         | check_doc_heading                           |
| Input          | A workspace root and the list of recognized document paths. |
| Output         | `Vec<LintResult>` carrying one finding per invariant violation. |
| Error          | Unreadable files produce no findings.        |
| Event          | Finding emitted                               |
| Test           | `lint-arwaky-cli docs .` reports 0 findings |
| Metric         | 0 doc-rule findings                         |

### FR-SAMPLE-002: Audit Orchestration

| Aspect         | Detail                                      |
|----------------|---------------------------------------------|
| Description    | The doc auditor is reachable through a single aggregate entry point dispatching to the invariant checker. |
| Business Rules | The aggregate orchestrates all invariants under one seam so consumers need only invoke `execute`. |
| Edge Cases     | A request with an empty root produces no findings rather than an error. |
| Capability     | doc_rules::DocStructure                     |
| Protocol       | doc_rules::DocStructure                     |
| Method         | execute                                     |
| Input          | A `DocRequest` describing the workspace root to audit. |
| Output         | A `DocResponse` carrying a deduplicated, stable-sorted list of findings. |
| Error          | Failures inside the checker are propagated as `DocResponse::Findings` with no partial results. |
| Event          | Finding emitted                               |
| Test           | `lint-arwaky-cli docs .` reports 0 findings |
| Metric         | Deterministic finding order                 |

## Public Interface

| Aspect     | Detail            |
|------------|-------------------|
| Agent      | doc_rules::Agent  |
| Aggregate  | doc_rules::DocStructure |
| Method     | execute           |
| Request    | `DocRequest`      |
| Response   | `DocResponse`     |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | {Business goals, user stories, and scope}  | [PRD.md](../../PRD.md) |
| Backlog      | {Task tracking, epics, and implementation status} | [BACKLOG.md](BACKLOG.md) |
| Architecture | {System boundaries, tech stack, and infra} | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
| Roadmap      | {What ships next, in order}                | [ROADMAP.md](../../ROADMAP.md) |

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

## Project Requirements

- Fewer defects.

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| ROADMAP      | What ships next, in order                  | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | {System boundaries, tech stack, and infra} | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
"
}

/// A ROADMAP.md that satisfies its own AES605 H2 contract.
fn conforming_roadmap() -> &'static str {
    r"# ROADMAP — sample

## Progress Overview

- Todo: none
- In Progress: None
- Blocked: None

## Roadmap Status

| ID | Feature | Priority | State | Dependencies | Next Action | Updated |
|---|---|---|---|---|---|---|
| WS-1 | Docs | P0 | Done | — | — | 2026-09-29 |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | What the product does and why              | [PRD.md](../../PRD.md) |
| Architecture | {System boundaries, tech stack, and infra} | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
"
}

/// A conforming BACKLOG.md that satisfies the AES605 H2 contract.
fn conforming_backlog() -> &'static str {
    r"# Feature Backlog: Sample

FRD: [FRD.md](FRD.md)
Architecture: [ARCHITECTURE.md](../../ARCHITECTURE.md)
State: values from root [ROADMAP.md](../../ROADMAP.md) — do not redefine here.
Last Updated: 2026-09-29

## Progress Overview

- Done: nothing yet
- In Progress: None
- Blocked: None
- Next Action: get started

## Backlog Status

### {FR-SAMPLE-001}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | —                                           |
| In Progress  | None                                        |
| Risk         | None                                        |
| Updated      | 2026-09-29                                  |

### {FR-SAMPLE-002}

| Aspect       | Detail                                      |
|--------------|---------------------------------------------|
| State        | Ready                                       |
| Done         | —                                           |
| In Progress  | None                                        |
| Risk         | None                                        |
| Updated      | 2026-09-29                                  |

## Reference

| Artifact     | Purpose                                    | Link |
|--------------|--------------------------------------------|------|
| PRD          | {Business goals, user stories, and scope}  | [PRD.md](../../PRD.md) |
| FRD          | {Functional requirements specification}    | [FRD.md](FRD.md) |
| Roadmap      | {Timeline, phasing, and dependencies}      | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | {System boundaries, tech stack, and infra} | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
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

/// A feature BACKLOG whose `Progress Overview` section restates the root-only
/// `Roadmap Status` section — the negative case `state_vocab_restated` must
/// catch.
fn roadmap_status_backlog() -> &'static str {
    r"# BACKLOG — sample

## Progress Overview

- Done: nothing
- In Progress: None
- Blocked: None

## Roadmap Status

| Priority | State | Dependencies | Updated |
|---|---|---|---|
| P0 | Ready | — | 2026-09-29 |
"
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
        "---\n\
trigger: always\n\
description: \"Sample.\"\n\
---\n\
# Sample AGENTS.md\n\n\
## Agent Overview\n\n\
{One paragraph: what AI agents must know before generating code, docs, or suggestions in this workspace.}\n\n\
## Runtime\n\n\
| Aspect          | Detail                                          |\n\
|-----------------|-------------------------------------------------|\n\
| Language        | {language and pinned version}                   |\n\
| Environment     | {environment manager and creation method}       |\n\
| Artifacts       | {reviewable artifact location}                  |\n\
| Package Manager | {package manager and lockfile}                  |\n\
| Version Probe   | `{version probe command}`                       |\n\
| Setup           | `{environment setup command}`                    |\n\
| Activation      | `{optional environment activation command}`      |\n\n\
## Project Structure\n\n\
| Path       | Purpose              |\n\
|------------|----------------------|\n\
| {path}     | {what lives here}    |\n\n\
## Commands\n\n\
| Category    | Command                              | Scope      | CI Gate              |\n\
|-------------|--------------------------------------|------------|----------------------|\n\
| Test (all)  | `{whole-workspace test command}`     | All        | ci.yml: {job name}   |\n\
| Test (unit) | `{single-package test command}`      | One unit   | ci.yml: {job name}   |\n\
| Test (file) | `{single-file test command}`         | One file   | ci.yml: {job name}   |\n\
| Lint        | `{formatter or linter command}`      | All        | ci.yml: {job name}   |\n\
| Type Check  | `{type checker command}`             | All        | ci.yml: {job name}   |\n\
| Arch Scan   | `{architecture scanner command}`     | All        | ci.yml: {job name}   |\n\n\
## Reference\n\n\
| Artifact     | Purpose                                    | Link |\n\
|--------------|--------------------------------------------|------|\n\
| README       | What the tool does, every command, install | [README.md](README.md) |\n\
| CONTRIBUTING | Branch, commit, and PR conventions         | [CONTRIBUTING.md](CONTRIBUTING.md) |\n\
| ARCHITECTURE | Layer contract for every crate             | [ARCHITECTURE.md](ARCHITECTURE.md) |\n\
| ROADMAP      | What ships next, in order                  | [ROADMAP.md](ROADMAP.md) |\n\
| PRD          | What the product does and why              | [PRD.md](PRD.md) |\n",
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
fn aes601_does_not_validate_fr_row_labels() {
    // AES601 retired `field_missing` (Decision 9): entry row-label
    // validation belongs to AES606 `entry_missing_row`. A requirement block
    // missing all of its rows must therefore not fire an AES601 finding.
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd()
        .replace(
            "- **Description**: Every `.md` document satisfies the AES heading and section contract.\n\
             - **Input**: A workspace root and the list of recognized document paths.\n\
             - **Output**: `Vec<LintResult>` carrying one finding per invariant violation.\n\
             - **Business Rules**: Validates FR-ID format, section structure, spec purity, crosslinks, heading structure, and parity.\n\
             - **Edge Cases**: Root-level legacy `BACKLOG.md` is accepted during migration.\n\
             - **Error Handling**: Unreadable files produce no findings. Missing documents are skipped silently.\n",
            "",
        );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        !findings
            .iter()
            .any(|(c, v, _, _)| c == "AES601" && v == "field_missing"),
        "AES601 no longer owns row-label validation; got: {findings:#?}"
    );
}

/// The 9-doc FRD contract closed off `api_no_subsection` with the retired
/// `## API Contract` H2 (AES602 now only answers `order_violation`): the
/// sanctioned `### Aggregate API` subsection no longer exists, so an old FRD
/// that still names the section carries an H2 outside `required ∪ allowed`.
/// The AES605 closed set is the rule that answers it.
#[test]
fn aes602_fires_when_aggregate_api_is_absent() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace("### Aggregate API", "## Aggregate API");
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "expected h2_unexpected for the absent subsection's stand-in section, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("aggregate api"),
        "the finding must name the inherited heading; got: {message}"
    );
}

// ── AES602: contract H2 set closed off the API Contract H3 shapes ───

/// A per-protocol heading narrating a split the single API table refused to
/// make: AES602 no longer polices level-3 shape, and written at level 2 — the
/// shape a legacy FRD gives it far more often than 10 — the heading sits
/// outside the FRD contract's H2 set, so AES605's `h2_unexpected` answers.
#[test]
fn aes602_fires_when_api_contract_has_a_per_protocol_h3() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "## Public Interface",
        "## IParserProtocol\n\n\
             | Method | Input | Output | Error | Event | Description |\n\
             | --- | --- | --- | --- | --- | --- |\n\
             | `parse` | `DocSource` | `Result<Tree>` | — | — | Parse one document. |\n\n\
             ## Public Interface",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "expected h2_unexpected, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("iparserprotocol"),
        "the finding must name the off-contract heading; got: {message}"
    );
}

#[test]
fn aes602_fires_when_api_contract_has_two_extra_h3s() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "## Public Interface",
        "## IGraphProtocol\n\n\
             | Method | Input | Output | Error | Event | Description |\n\
             | --- | --- | --- | --- | --- | --- |\n\
             | `build` | `Vec<Edge>` | `Graph` | — | — | Build. |\n\n\
             ## IParserProtocol\n\n\
             | Method | Input | Output | Error | Event | Description |\n\
             | --- | --- | --- | --- | --- | --- |\n\
             | `parse` | `DocSource` | `Result<Tree>` | — | — | Parse one document. |\n\n\
             ## Public Interface",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    // The closed H2 set reports one finding naming every off-contract heading.
    let count = findings
        .iter()
        .filter(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .count();
    assert_eq!(
        count, 1,
        "expected 1 h2_unexpected naming both headings, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("igraphprotocol") && message.contains("iparserprotocol"),
        "the finding must name both off-contract headings; got: {message}"
    );
}

/// The 9-doc FRD contract's H2 set is closed: `## Assumptions & Constraints`
/// left the layout, so a protocol table parked under it is an off-contract
/// section the AES605 closed set answers with `h2_unexpected`.
#[test]
fn aes602_fires_when_a_protocol_table_is_parked_under_assumptions() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "## Reference",
        "## Assumptions & Constraints\n\n\
         The capability is stateless.\n\n\
         ### IToolResolutionProtocol (12 operations)\n\n\
         | Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | `has_local_bin` | `&Path, &ToolName` | `bool` | — | — | Local bin. |\n\n\
         ## Reference",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "expected h2_unexpected for a protocol table parked outside the contract, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("assumptions"),
        "the finding must name the retired section; got: {message}"
    );
}

/// The closed H2 set answers an invented SCEN section: `## Test Scenarios`
/// left the 9-doc layout, so a scenario promoted out of the bullet list into
/// its own section — here with a level-3 item heading — fires `h2_unexpected`
/// naming the section it was parked in.
#[test]
fn aes602_fires_when_a_scen_heading_is_promoted_below_test_scenarios() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "## Reference",
        "## Test Scenarios\n\n\
         - A conforming request produces a conforming response.\n\n\
         ### SCEN-001: Dispatch\n\n\
         | # | Scenario | Expected |\n\
         | --- | --- | --- |\n\
         | 1 | Conforming request | Conforming response |\n\n\
         ## Reference",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "expected h2_unexpected for an invented Test Scenarios section, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("test scenarios"),
        "the finding must name the retired section; got: {message}"
    );
}

/// A FRD carrying every contract H2 in the correct order reports no
/// AES602 finding.
#[test]
fn aes602_accepts_the_three_sanctioned_h3_shapes() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES602", "order_violation"),
        "a contract-conforming FRD must not fire order_violation, got: {findings:#?}"
    );
}

/// The remedy the order message names: a document that puts every contract
/// H2 back in template order must stay silent.
#[test]
fn aes602_accepts_a_demoted_h4_heading() {
    let tmp = tempfile::tempdir().unwrap();
    // Demoting detail to H4 under the contract sections is free-form.
    let frd = conforming_frd().replace(
        "## Feature Overview\n\nThe feature does the work a surface delegates to.",
        "## Feature Overview\n\n\
         The feature does the work a surface delegates to.\n\n\
         #### Tool resolution details\n\n\
         - Resolution order is local bin, then system path.",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES602", "order_violation"),
        "a level-4 heading is detail, not a section; got: {findings:#?}"
    );
}

/// Headings inside fenced code blocks are not sections, so the order check
/// must not read a fenced heading as a scrambled template section.
#[test]
fn aes602_h3_parity_ignores_headings_inside_a_fenced_block() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "## Feature Overview\n\nThe feature does the work a surface delegates to.",
        "## Feature Overview\n\n\
         The feature does the work a surface delegates to.\n\n\
         ```bash\n\
         ### not-a-heading\n\
         # also not a heading\n\
         ```",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES602", "order_violation"),
        "fenced headings must be ignored, got: {findings:#?}"
    );
}

/// `api_subsection_no_table` left with the 9-doc layout: an API table that a
/// prose paragraph replaced now reads as an off-contract `## Protocol API`
/// section (the H2 a legacy FRD promotes it to), which the closed H2 set
/// answers with `h2_unexpected`.
#[test]
fn aes602_fires_when_protocol_api_is_prose_only() {
    let tmp = tempfile::tempdir().unwrap();
    // Author dumps every method into prose under Protocol API instead of a table.
    let frd = conforming_frd().replace(
        "### Protocol API\n\n\
         | Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Single composite entry point. |",
        "## Protocol API\n\n\
         Every method is documented in prose rather than in a table.",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "expected h2_unexpected for the prose-only API section, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("protocol api"),
        "the finding must name the prose-only section; got: {message}"
    );
}

#[test]
fn aes602_fires_when_aggregate_api_is_prose_only() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "### Aggregate API\n\n\
         | Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Routes to the capability. |",
        "## Aggregate API\n\n\
         A single entry point routes requests, in prose rather than in a table.",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "expected h2_unexpected for the prose-only API section, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("aggregate api"),
        "the finding must name the prose-only section; got: {message}"
    );
}

#[test]
fn aes602_fires_when_aggregate_api_is_duplicated() {
    let tmp = tempfile::tempdir().unwrap();
    // The second copy is promoted to level 2, which is the shape the closed
    // H2 set judges; the level-3 original stays where the template put it.
    let frd = conforming_frd().replace(
        "### Aggregate API",
        "## Aggregate API\n\n\
         | Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | `execute` | `SampleRequest` | `SampleResponse` | Reason-coded | — | Routes to the capability. |\n\n\
         ### Aggregate API",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "expected h2_unexpected for the duplicated API section, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("aggregate api"),
        "the finding must name the duplicated section; got: {message}"
    );
}

/// Level-3 subsection order left AES602's scope with the 9-doc layout; the
/// order the rule still answers is the section order, so an API surface
/// hoisted above the requirement block is the scramble it reports.
#[test]
fn aes602_fires_when_api_subsections_are_out_of_order() {
    let tmp = tempfile::tempdir().unwrap();
    // Swap `## Public Interface` ahead of `## Functional Requirements`.
    let scrambled = conforming_frd()
        .replace("## Functional Requirements", "## @@MARK@@")
        .replace("## Public Interface", "## Functional Requirements")
        .replace("## @@MARK@@", "## Public Interface");
    write_workspace(tmp.path(), &scrambled);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES602", "order_violation"),
        "expected order_violation, got: {findings:#?}"
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

/// `integration_not_table` left AES602 with the 9-doc layout; a bullet-only
/// Integration Points block parked at level 2 now reads as an H2 outside the
/// closed FRD set, and AES605's `h2_unexpected` is the finding that answers.
#[test]
fn aes602_fires_when_integration_points_is_a_bullet_list() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "## Reference",
        "## Integration Points\n\n\
- Shared domain supplies value objects.\n\
- Orchestrator dispatches the check.\n\n\
## Reference",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "expected h2_unexpected for the bullet-only integration block, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("integration points"),
        "the finding must name the section; got: {message}"
    );
}

/// `nfr_not_table` left AES602 with the 9-doc layout; prose NFR narration
/// hoisted to level 2 is now an H2 outside the closed FRD set, answered by
/// AES605's `h2_unexpected`.
#[test]
fn aes602_fires_when_nfr_is_prose() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "## Reference",
        "## Non-Functional Requirements\n\n\
Dispatch cost stays one call per request.\n\n\
## Reference",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "expected h2_unexpected for the prose NFR block, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("non functional requirements"),
        "the finding must name the section; got: {message}"
    );
}

#[test]
fn aes602_fires_when_sections_are_out_of_order() {
    let tmp = tempfile::tempdir().unwrap();
    // Swap `## Reference` ahead of `## Public Interface`.
    let scrambled = conforming_frd()
        .replace("## Public Interface", "## @@MARK@@")
        .replace("## Reference", "## Public Interface")
        .replace("## @@MARK@@", "## Reference");
    write_workspace(tmp.path(), &scrambled);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES602", "order_violation"),
        "expected order_violation, got: {findings:#?}"
    );
}

/// `scenarios_empty` left AES602 with the 9-doc layout; a bullet-free
/// Test Scenarios block at level 2 is an H2 outside the closed FRD set,
/// answered by AES605's `h2_unexpected`.
#[test]
fn aes602_fires_when_scenarios_have_no_bullets() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "## Reference",
        "## Test Scenarios\n\n\
The scenario list carries no bullets, only a sentence.\n\n\
## Reference",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "expected h2_unexpected for the bullet-free scenarios block, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("test scenarios"),
        "the finding must name the section; got: {message}"
    );
}

/// `glossary_empty` left AES602 with the 9-doc layout; a table-only Glossary
/// at level 2 is an H2 outside the closed FRD set, answered by AES605's
/// `h2_unexpected`.
#[test]
fn aes602_fires_when_glossary_has_no_bullets() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace(
        "## Reference",
        "## Glossary\n\n\
| Term | Meaning |\n\
| --- | --- |\n\
| Sample | A value object used in this example. |\n\n\
## Reference",
    );
    write_workspace(tmp.path(), &frd);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES605", "h2_unexpected"),
        "expected h2_unexpected for the table-only glossary, got: {findings:#?}"
    );
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_unexpected")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("glossary"),
        "the finding must name the section; got: {message}"
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
    // The Reference section is a table now; blanking the artifact cell is the
    // row-level equivalent of dropping the bullet the old fixture deleted.
    let frd = conforming_frd().replace("[BACKLOG.md](BACKLOG.md)", "");
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES604", "no_backlog_link"));
}

#[test]
fn aes604_fires_when_the_frd_omits_its_prd_link() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = conforming_frd().replace("[PRD.md](../../PRD.md)", "");
    write_workspace(tmp.path(), &frd);
    assert!(has(&audit(tmp.path()), "AES604", "no_prd_link"));
}

#[test]
fn aes604_fires_when_a_feature_backlog_restates_master_sections() {
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    fs::write(
        tmp.path().join("crates/sample/BACKLOG.md"),
        // `state_vocab_restated` answers only when the feature backlog carries
        // its own `Progress Overview` next to a master-only section; the
        // legacy section set below stays, since restating it is this test's
        // subject.
        "# BACKLOG — sample\n\n\
## Progress Overview\n\n\
- Done: nothing\n\n\
## Roadmap Status\n\n\
| Priority | State | Dependencies | Updated |\n\
|---|---|---|---|\n\
| P0 | Ready | — | 2026-09-29 |\n\n\
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
fn aes604_fires_when_a_feature_backlog_restates_roadmap_status() {
    // A feature BACKLOG that restates the root-only `Roadmap Status` section
    // must fire `state_vocab_restated` even though `Progress Overview` is a
    // legitimate feature-backlog H2.
    let tmp = tempfile::tempdir().unwrap();
    write_workspace(tmp.path(), &conforming_frd());
    fs::write(
        tmp.path().join("crates/sample/BACKLOG.md"),
        roadmap_status_backlog(),
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
        a.0.cmp(&b.0)
            .then_with(|| a.1.cmp(&b.1))
            .then_with(|| a.2.cmp(&b.2))
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
    // Runtime, Documentation Awareness removed, and most others are missing from the required set.
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
        message.contains("Agent Overview") || message.contains("Project Structure"),
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

## Agent Overview

AI agents working in this workspace must know the AES 7-layer naming convention.

## Runtime

- Language: Rust.

## Project Structure

one crate.

## Commands

```bash
true
```

## Reference

- [README.md](README.md).",
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
## Reference\n\
- [README.md](README.md).",
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
        message.contains("Agent Overview") && message.contains("Project Structure"),
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
    // The AGENTS H2 set is closed: the project's own five template sections
    // and nothing else, so a workspace that declares exactly that set passes.
    let tmp = tempfile::tempdir().unwrap();
    write_bad_agents(
        tmp.path(),
        "# Sample AGENTS.md\n\n\
## Agent Overview\n\n\
AI agents must know this workspace's conventions before generating anything.\n\n\
## Runtime\n\n\
| Aspect          | Detail                                          |\n\
|-----------------|-------------------------------------------------|\n\
| Language        | Rust, edition 2024 pinned                       |\n\
| Package Manager | cargo, with Cargo.lock                          |\n\n\
## Project Structure\n\n\
| Path       | Purpose              |\n\
|------------|----------------------|\n\
| crates/    | Rust workspace crates|\n\n\
## Commands\n\n\
| Category    | Command                              | Scope      | CI Gate              |\n\
|-------------|--------------------------------------|------------|----------------------|\n\
| Test (all)  | `cargo nextest run --workspace`      | All        | ci.yml: Test          |\n\n\
## Reference\n\n\
| Artifact | Purpose            | Link        |\n\
|----------|--------------------|-------------|\n\
| README   | How the tool works | [README.md](README.md) |\n",
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
    // A ROADMAP.md without Progress Overview must name the document and the
    // absent section.
    let text = conforming_roadmap().replace("## Progress Overview", "## Progress Summary");
    let findings = audit_root_doc("ROADMAP.md", &text);
    assert!(has(&findings, "AES605", "h2_missing"));
    let message = findings
        .iter()
        .find(|(c, v, _, _)| c == "AES605" && v == "h2_missing")
        .map(|(_, _, m, _)| m.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("ROADMAP.md") && message.contains("Progress Overview"),
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
    // Layout is documented in AGENTS.md. A README that repeats it is a second
    // source of truth, so the section is off-template. The rest of the
    // fixture carries every README_REQUIRED_H2S so only the rebel section
    // drives the findings.
    let text = r#"# Sample README

## Project Overview

What the tool does.

## Prerequisites

- A toolchain.

## Quick Start

| Step | Action | Command |
| --- | --- | --- |
| 1 | Install | `cargo install lint-arwaky-cli` |

## Reference

| Artifact | Purpose | Link |
| --- | --- | --- |
| CONTRIBUTING | Branch and PR conventions | [CONTRIBUTING.md](CONTRIBUTING.md) |

## Project Structure

Where things live.
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

## 12. Finding rendering notes

Notes.

## 13. Rule Inventory Overview

Groups.

## 14. Group 1: Naming

Naming.

## 15. Group 2: Layer & Import Boundary

Boundary.

## 16. Group 3: File & Content Quality

Quality.

## 17. Group 4: Role Violations

Roles.

## 18. Group 5: Orphan Code

Orphans.

## 19. Group 6: Document Invariants

Docs.

## 20. Group 7: Folder Structure

Folders.

## 21. Rule Source of Truth & Parity

Parity.
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
fn backlog_qualifier_mismatch_fires_on_uncarried_qualifier() {
    let tmp = tempfile::tempdir().unwrap();
    // FRD declares FR-SAMPLE-001/002; BACKLOG entry token FR-OTHER-001 is not
    // declared by any FR-ID → backlog_qualifier_mismatch anchored to the entry.
    let backlog = backlog_with_entries(&["FR-OTHER-001"]);
    write_frd_backlog_pair(tmp.path(), &conforming_frd(), Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "backlog_qualifier_mismatch"),
        "an entry token no FR-ID carries fires backlog_qualifier_mismatch; got: {findings:#?}"
    );
    let raw = audit_findings(tmp.path());
    let mismatch = raw
        .iter()
        .find(|f| f.violation_type == "backlog_qualifier_mismatch")
        .expect("the mismatch finding must fire");
    let backlog_text = fs::read_to_string(tmp.path().join("crates/sample/BACKLOG.md")).unwrap();
    let h3_line = backlog_text
        .lines()
        .position(|l| l.trim() == "### {FR-OTHER-001}")
        .unwrap()
        + 1;
    assert_eq!(
        mismatch.line, h3_line,
        "`backlog_qualifier_mismatch` anchors to the entry's H3 line; got {}",
        mismatch.line
    );
}

#[test]
fn backlog_missing_direction_fires_on_uncovered_qualifier() {
    let tmp = tempfile::tempdir().unwrap();
    // FRD declares FR-SAMPLE-001/002 but BACKLOG declares no entry →
    // backlog_id_missing fires once per declared FR ID (2).
    let backlog = backlog_with_entries(&[]);
    write_frd_backlog_pair(tmp.path(), &conforming_frd(), Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "backlog_id_missing"),
        "a spec-declared ID with no BACKLOG entry fires backlog_id_missing; got: {findings:#?}"
    );
    let raw = audit_findings(tmp.path());
    let count = raw
        .iter()
        .filter(|f| f.violation_type == "backlog_id_missing")
        .count();
    assert_eq!(
        count, 2,
        "missing direction fires once per declared FR ID; got {count}"
    );
    let backlog_text = fs::read_to_string(tmp.path().join("crates/sample/BACKLOG.md")).unwrap();
    let section_line = backlog_text
        .lines()
        .position(|l| l.trim() == "## Backlog Status")
        .unwrap()
        + 1;
    assert_eq!(
        raw.iter()
            .find(|f| f.violation_type == "backlog_id_missing")
            .expect("the missing finding must fire")
            .line,
        section_line,
        "`backlog_id_missing` anchors to `## Backlog Status`; got {section_line}"
    );
}

#[test]
fn backlog_stray_id_in_next_fires_not_fr() {
    let tmp = tempfile::tempdir().unwrap();
    // BACKLOG entry FR-SAMPLE-001 `Done` cites FR-OTHER-001, which the FRD
    // never declares → backlog_id_not_fr anchored to the entry H3.
    let backlog = backlog_with_entries(&["FR-SAMPLE-001"])
        .replace("| Done        | —      |", "| Done        | FR-OTHER-001 |");
    write_frd_backlog_pair(tmp.path(), &conforming_frd(), Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "backlog_id_not_fr"),
        "a DONE row token not declared in the FRD fires backlog_id_not_fr; got: {findings:#?}"
    );
    let raw = audit_findings(tmp.path());
    let not_fr = raw
        .iter()
        .find(|f| f.violation_type == "backlog_id_not_fr")
        .expect("the not-fr finding must fire");
    let backlog_text = fs::read_to_string(tmp.path().join("crates/sample/BACKLOG.md")).unwrap();
    let h3_line = backlog_text
        .lines()
        .position(|l| l.trim() == "### {FR-SAMPLE-001}")
        .unwrap()
        + 1;
    assert_eq!(
        not_fr.line, h3_line,
        "`backlog_id_not_fr` anchors to the entry's H3 line; got {}",
        not_fr.line
    );
}

#[test]
fn backlog_qualifier_full_coverage_reports_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let backlog = backlog_with_entries(&["FR-SAMPLE-001", "FR-SAMPLE-002"]);
    write_frd_backlog_pair(tmp.path(), &conforming_frd(), Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "backlog_qualifier_mismatch")
            && !has(&findings, "AES601", "backlog_id_missing")
            && !has(&findings, "AES601", "backlog_id_not_fr"),
        "a fully aligned feature BACKLOG pair reports no qualifier findings; got: {findings:#?}"
    );
}

/// The FRD declares two FR IDs but the Backlog Status carries an entry for
/// only one of them ⇒ exactly one `backlog_id_missing` naming the uncovered ID.
#[test]
fn backlog_missing_direction_fires_once_per_spec_id() {
    let tmp = tempfile::tempdir().unwrap();
    // FRD declares FR-SAMPLE-001/002; BACKLOG carries an entry for FR-SAMPLE-001 only.
    let backlog = backlog_with_entries(&["FR-SAMPLE-001"]);
    write_frd_backlog_pair(tmp.path(), &conforming_frd(), Some(&backlog));
    let findings = audit(tmp.path());
    let missing: Vec<_> = findings
        .iter()
        .filter(|(_, v, _, _)| v == "backlog_id_missing")
        .collect();
    assert_eq!(
        missing.len(),
        1,
        "missing direction fires exactly once per uncovered ID; got {missing:?}"
    );
    assert!(
        missing
            .iter()
            .any(|(_, _, m, _)| m.contains("FR-SAMPLE-002")),
        "the single missing finding must name FR-SAMPLE-002; got: {missing:?}"
    );
}

#[test]
fn backlog_without_backlog_status_section_is_silent() {
    let tmp = tempfile::tempdir().unwrap();
    // No `## Backlog Status` section means there is no entry set to align against.
    let backlog =
        backlog_with_entries(&["FR-SAMPLE-001"]).replace("## Backlog Status\n", "## Other\n");
    write_frd_backlog_pair(tmp.path(), &conforming_frd(), Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "backlog_qualifier_mismatch")
            && !has(&findings, "AES601", "backlog_id_missing"),
        "a BACKLOG without a `## Backlog Status` section stays silent; got: {findings:#?}"
    );
}

// ── AES601: PR-ID parity (PRD ↔ ROADMAP) ───────────────────────────────────

/// A PRD that carries PR-QUAL-001 and PR-QUAL-002 items under `## Project
/// Requirements`.
fn prd_with_pr_ids() -> String {
    "# PRD — sample\n\n\
     ## Problem Statement\n\n\
     The problem.\n\n\
     ## Project Requirements\n\n\
     ### PR-QUAL-001: Implement Scanner\n\n\
     | Aspect | Detail |\n|---|---|\n\
     | Description | Scans documents |\n\n\
     ### PR-QUAL-002: Implement Formatter\n\n\
     | Aspect | Detail |\n|---|---|\n\
     | Description | Formats output |\n\n\
     ## Reference\n\n\
     - Roadmap: [ROADMAP.md](ROADMAP.md)\n"
        .to_string()
}

/// A ROADMAP that carries PR-QUAL-001 and PR-QUAL-002 items under
/// `## Roadmap Status`.
fn roadmap_with_pr_ids() -> String {
    "# ROADMAP — sample\n\n\
     ## Progress Overview\n\n\
     Baseline.\n\n\
     ## Roadmap Status\n\n\
     ### PR-QUAL-001: Implement Scanner\n\n\
     | Aspect | Detail |\n|---|---|\n\
     | Priority | P0 |\n\n\
     ### PR-QUAL-002: Implement Formatter\n\n\
     | Aspect | Detail |\n|---|---|\n\
     | Priority | P1 |\n\n\
     ## Reference\n\n\
     - PRD: [PRD.md](PRD.md)\n"
        .to_string()
}

/// Write a root workspace whose `PRD.md` and `ROADMAP.md` hold `prd` and
/// `roadmap` respectively, so the PR-ID parity sub-check contributes findings.
fn write_pr_roadmap_pair(dir: &Path, prd: &str, roadmap: &str) {
    fs::write(dir.join("PRD.md"), prd).unwrap();
    fs::write(dir.join("ROADMAP.md"), roadmap).unwrap();
    // Feature folder so AES604/605 crosslink and heading checks have a target.
    let feature = dir.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    fs::write(feature.join("FRD.md"), conforming_frd()).unwrap();
    fs::write(feature.join("BACKLOG.md"), conforming_backlog()).unwrap();
    fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();
}

#[test]
fn pr_roadmap_parity_reports_nothing_when_aligned() {
    let tmp = tempfile::tempdir().unwrap();
    write_pr_roadmap_pair(tmp.path(), &prd_with_pr_ids(), &roadmap_with_pr_ids());
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "pr_roadmap_id_missing")
            && !has(&findings, "AES601", "roadmap_pr_id_not_pr"),
        "aligned PRD and ROADMAP report no PR-parity findings; got: {findings:#?}"
    );
}

#[test]
fn pr_id_in_prd_missing_from_roadmap_fires_pr_roadmap_id_missing() {
    let tmp = tempfile::tempdir().unwrap();
    // PRD declares PR-QUAL-002; ROADMAP does not.
    let roadmap = roadmap_with_pr_ids().replace("### PR-QUAL-002: Implement Formatter\n\n", "");
    write_pr_roadmap_pair(tmp.path(), &prd_with_pr_ids(), &roadmap);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "pr_roadmap_id_missing"),
        "a PRD PR-ID absent from the ROADMAP fires pr_roadmap_id_missing; got: {findings:#?}"
    );
    let raw = audit_findings(tmp.path());
    let missing = raw
        .iter()
        .find(|f| f.violation_type == "pr_roadmap_id_missing")
        .expect("the missing finding must fire");
    assert!(
        missing.message.contains("PR-QUAL-002"),
        "the finding must name the missing PR-ID; got: {}",
        missing.message
    );
}

#[test]
fn pr_id_in_roadmap_missing_from_prd_fires_roadmap_pr_id_not_pr() {
    let tmp = tempfile::tempdir().unwrap();
    // ROADMAP declares PR-QUAL-003; PRD does not.
    let roadmap = roadmap_with_pr_ids().replace(
        "## Reference\n",
        "### PR-QUAL-003: Extra Item\n\n| Aspect | Detail |\n|---|---|\n| Priority | P2 |\n\n## Reference\n",
    );
    write_pr_roadmap_pair(tmp.path(), &prd_with_pr_ids(), &roadmap);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "roadmap_pr_id_not_pr"),
        "a ROADMAP PR-ID absent from the PRD fires roadmap_pr_id_not_pr; got: {findings:#?}"
    );
    let raw = audit_findings(tmp.path());
    let not_pr = raw
        .iter()
        .find(|f| f.violation_type == "roadmap_pr_id_not_pr")
        .expect("the not-pr finding must fire");
    assert!(
        not_pr.message.contains("PR-QUAL-003"),
        "the finding must name the untracked PR-ID; got: {}",
        not_pr.message
    );
}

#[test]
fn pr_parity_both_directions_fire_on_misaligned_pair() {
    let tmp = tempfile::tempdir().unwrap();
    let prd = prd_with_pr_ids();
    let roadmap = roadmap_with_pr_ids().replace("### PR-QUAL-002", "### PR-QUAL-003");
    write_pr_roadmap_pair(tmp.path(), &prd, &roadmap);
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "pr_roadmap_id_missing")
            && has(&findings, "AES601", "roadmap_pr_id_not_pr"),
        "one misaligned PR pair reports both directions; got: {findings:#?}"
    );
}

// ── AES601: DS/DT ↔ BACKLOG ID alignment ─────────────────────────────────

/// A DESIGN.md carrying qualified `DS-DISP-NNN:` headings under
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
     | Artifact | Purpose | Link |\n|---|---|---|\n\
     | Backlog | [BACKLOG.md](BACKLOG.md) |\n"
        .to_string()
}

/// A DATA.md carrying a qualified H2 `## DT-SHAR-NNN: <title>` item.
fn conforming_data_with_dt() -> String {
    "# DATA — shared\n\n\
     ## Data Overview\n\n\
     The shared kernel manages value objects.\n\n\
     ## Data Requirements\n\n\
     | Type     | Description                                  |\n\
     |----------|----------------------------------------------|\n\
     | vo       | Lint finding severity level                  |\n\n\
     ## DT-SHAR-001: Severity\n\n\
     | Type  | Description |\n|-------|-------------|\n\
     | vo    | Lint finding severity level |\n\n\
     ## Reference\n\n\
     | Artifact | Purpose | Link |\n|---|---|---|\
     | Backlog | [BACKLOG.md](BACKLOG.md) |\n"
        .to_string()
}

/// A BACKLOG whose `## Backlog Status` entries carry the given H3 full-ID tokens.
fn backlog_with_entries(ids: &[&str]) -> String {
    let entries = ids
        .iter()
        .map(|id| {
            format!(
                "### {{{id}}}\n\n\
                 | Aspect      | Detail |\n\
                 |-------------|--------|\n\
                 | State       | Ready  |\n\
                 | Done        | —      |\n\
                 | In Progress | —      |\n\
                 | Risk        | —      |\n\
                 | Updated     | 2026-09-29 |\n"
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# Feature Backlog: Sample\n\n\
         ## Progress Overview\n\n\
         Baseline.\n\n\
         ## Backlog Status\n\
         {entries}\n\
         ## Reference\n\n\
         - Backlog: [BACKLOG.md](BACKLOG.md)\n"
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
    let backlog = backlog_with_entries(&["DS-DISP-001"]);
    write_design_backlog_pair(tmp.path(), &design, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "ds_backlog_id_not_ds")
            && !has(&findings, "AES601", "ds_backlog_id_missing"),
        "every DS ID has a BACKLOG entry and every entry ID is declared by a DS-ID; got: {findings:#?}"
    );
}

#[test]
fn ds_mismatch_reports_qualifier_mismatch_and_missing() {
    let tmp = tempfile::tempdir().unwrap();
    // DESIGN declares DS-DISP-001 and DS-DISP-003; BACKLOG entry FR-SAMPLE-001
    // is not declared by any DS-ID, and DS-DISP-003 owes no entry.
    let mut design = conforming_design_with_ds();
    // Remove the first item's table so it declares its ID without a body.
    design = design.replace(
        "### DS-DISP-001: Primary Button\n\n| Variant | Use case | Token | Value |\n|---|---|---|---|\n| `primary` | main action | `--color-primary` | `#000000` |",
        "### DS-DISP-001: Primary Button",
    );
    // Insert the second item with its own table before `## Reference`.
    design = design.replace(
        "## Reference\n",
        "### DS-DISP-003: Card\n\n| Variant | Use case | Token | Value |\n|---|---|---|---|\n| `card` | surface | `--color-card` | `#101010` |\n\n## Reference\n",
    );
    let backlog = backlog_with_entries(&["DS-DISP-001", "FR-SAMPLE-001"]);
    write_design_backlog_pair(tmp.path(), &design, Some(&backlog));
    let findings = audit(tmp.path());
    let mismatch = has(&findings, "AES601", "backlog_qualifier_mismatch");
    let missing = findings.iter().any(|(c, v, m, _)| {
        c == "AES601" && v == "ds_backlog_id_missing" && m.contains("DS-DISP-003")
    });
    assert!(
        mismatch && missing,
        "an undeclared entry token fires backlog_qualifier_mismatch; a declared ID without an entry fires ds_backlog_id_missing; got: {findings:#?}"
    );
    // The mismatch finding anchors to the `### {FR-SAMPLE-001}` entry H3 line.
    let raw = audit_findings(tmp.path());
    let mismatch_finding = raw
        .iter()
        .find(|f| f.violation_type == "backlog_qualifier_mismatch")
        .expect("the mismatch finding must fire");
    let backlog_text = fs::read_to_string(tmp.path().join("crates/sample/BACKLOG.md")).unwrap();
    let h3_line = backlog_text
        .lines()
        .position(|l| l.trim() == "### {FR-SAMPLE-001}")
        .unwrap()
        + 1;
    assert_eq!(
        mismatch_finding.line, h3_line,
        "`backlog_qualifier_mismatch` anchors to the entry's H3 line; got {}",
        mismatch_finding.line
    );
}

#[test]
fn ds_no_ids_declared_is_silent_on_stray() {
    let tmp = tempfile::tempdir().unwrap();
    // DESIGN declares no DS-IDs; an empty union is silent on directions (a)/(b).
    let design = "# Design — ui\n\n## Design Overview\n\n- base\n\n## Reference\n\n- Backlog: [BACKLOG.md](BACKLOG.md)\n";
    let backlog = backlog_with_entries(&["DS-DISP-001"]);
    write_design_backlog_pair(tmp.path(), design, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "backlog_qualifier_mismatch")
            && !has(&findings, "AES601", "ds_backlog_id_missing"),
        "a DESIGN with no DS-IDs is silent on alignment; got: {findings:#?}"
    );
}

#[test]
fn ds_backlog_without_backlog_status_section_is_silent() {
    let tmp = tempfile::tempdir().unwrap();
    let design = conforming_design_with_ds();
    let backlog =
        backlog_with_entries(&["DS-DISP-001"]).replace("## Backlog Status\n", "## Other\n");
    write_design_backlog_pair(tmp.path(), &design, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "backlog_qualifier_mismatch")
            && !has(&findings, "AES601", "ds_backlog_id_missing"),
        "a BACKLOG without a `## Backlog Status` section stays silent; got: {findings:#?}"
    );
}

#[test]
fn dt_full_coverage_reports_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let data = conforming_data_with_dt();
    let backlog = backlog_with_entries(&["DT-SHAR-001"]);
    write_data_backlog_pair(tmp.path(), &data, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "dt_backlog_id_not_dt")
            && !has(&findings, "AES601", "dt_backlog_id_missing"),
        "every DT ID has a BACKLOG entry and every entry ID is declared by a DT-ID; got: {findings:#?}"
    );
}

#[test]
fn dt_mismatch_reports_qualifier_mismatch_and_missing() {
    let tmp = tempfile::tempdir().unwrap();
    let data = conforming_data_with_dt().replace(
        "## DT-SHAR-001: Severity\n\n",
        "## DT-SHAR-001: Severity\n\n## DT-SHAR-003: MemberDelta\n\n",
    );
    let backlog = backlog_with_entries(&["DT-SHAR-001", "FR-SAMPLE-001"]);
    write_data_backlog_pair(tmp.path(), &data, Some(&backlog));
    let findings = audit(tmp.path());
    let mismatch = has(&findings, "AES601", "backlog_qualifier_mismatch");
    let missing = findings.iter().any(|(c, v, m, _)| {
        c == "AES601" && v == "dt_backlog_id_missing" && m.contains("DT-SHAR-003")
    });
    assert!(
        mismatch && missing,
        "an undeclared entry token fires backlog_qualifier_mismatch; a declared ID without an entry fires dt_backlog_id_missing; got: {findings:#?}"
    );
}

#[test]
fn dt_stray_id_in_next_fires_not_dt() {
    let tmp = tempfile::tempdir().unwrap();
    // DATA declares DT-SHAR-001; BACKLOG entry `Done` cites DT-MCPS-002, which
    // the DATA doc does not declare → dt_backlog_id_not_dt anchored to H3.
    let data = conforming_data_with_dt();
    let backlog = backlog_with_entries(&["DT-SHAR-001"])
        .replace("| Done        | —      |", "| Done        | DT-MCPS-002 |");
    write_data_backlog_pair(tmp.path(), &data, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        has(&findings, "AES601", "dt_backlog_id_not_dt"),
        "a DONE row token not declared in DATA fires dt_backlog_id_not_dt anchored to the entry H3; got: {findings:#?}"
    );
    let raw = audit_findings(tmp.path());
    let not_dt = raw
        .iter()
        .find(|f| f.violation_type == "dt_backlog_id_not_dt")
        .expect("the not-dt finding must fire");
    let backlog_text = fs::read_to_string(tmp.path().join("crates/shared/BACKLOG.md")).unwrap();
    let h3_line = backlog_text
        .lines()
        .position(|l| l.trim() == "### {DT-SHAR-001}")
        .unwrap()
        + 1;
    assert_eq!(
        not_dt.line, h3_line,
        "`dt_backlog_id_not_dt` anchors to the entry's H3 line; got {}",
        not_dt.line
    );
}

#[test]
fn dt_empty_heading_set_fires_not_dt_on_every_legacy_row() {
    let tmp = tempfile::tempdir().unwrap();
    let data = "# DATA — shared\n\n## Reference\n\n- Backlog: [BACKLOG.md](BACKLOG.md)\n\n## Data Overview\n\nnone.\n";
    let backlog = backlog_with_entries(&["DT-SHAR-001"]);
    write_data_backlog_pair(tmp.path(), data, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "dt_backlog_id_not_dt"),
        "a DATA with no DT-IDs is silent on alignment; got: {findings:#?}"
    );
}

/// A bare `DS-001:` heading (digits only, no NAME qualifier) is no longer an
/// accepted DS-ID.
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
    let backlog = backlog_with_entries(&["DISP"]);
    write_design_backlog_pair(tmp.path(), design, Some(&backlog));
    let findings = audit(tmp.path());
    // The bare heading is not collected, so no DS-ID set is declared and the
    // pair is silent.
    assert!(
        !has(&findings, "AES601", "ds_backlog_id_not_ds"),
        "a bare `DS-001:` heading must not be accepted; an empty DS-ID set is silent; got: {findings:#?}"
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
    let backlog = backlog_with_entries(&["SHAR"]);
    write_data_backlog_pair(tmp.path(), data, Some(&backlog));
    let findings = audit(tmp.path());
    assert!(
        !has(&findings, "AES601", "dt_backlog_id_not_dt"),
        "a bare `DT-001:` heading must not be accepted; an empty DT-ID set is silent; got: {findings:#?}"
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
