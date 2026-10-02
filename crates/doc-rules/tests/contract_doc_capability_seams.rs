// PURPOSE: contract tests for the five capability seams — each answers exactly
// one AES doc rule, and each is reachable only through its own protocol trait.
//
// The aggregate tests in `contract_doc_rules.rs` prove the rules fire end to
// end. These prove the thing the split introduced: a capability wired to the
// wrong trait, or answering a second rule, is caught here rather than by a
// consumer's surprise report.
use doc_rules_lint_arwaky::capabilities_doc_crosslink_checker::CrosslinkChecker;
use doc_rules_lint_arwaky::capabilities_doc_fr_format_checker::FrFormatChecker;
use doc_rules_lint_arwaky::capabilities_doc_heading_structure_checker::HeadingStructureChecker;
use doc_rules_lint_arwaky::capabilities_doc_section_structure_checker::SectionStructureChecker;
use doc_rules_lint_arwaky::capabilities_doc_spec_purity_checker::SpecPurityChecker;
use shared_doc_rules::contract_doc_protocol::{
    ICrosslinkProtocol, IDocHeadingProtocol, IFrFormatProtocol, ISectionStructureProtocol,
    ISpecPurityProtocol,
};
use shared_doc_rules::taxonomy_doc_audit_context_vo::DocAuditContext;
use shared_doc_rules::taxonomy_doc_rules_request::DocFinding;
use shared_doc_rules::utility_doc_collector;

use std::path::Path;

/// Build the audit context the agent builds: walk the chain, find the master,
/// and hand both to the capability.
fn context_for(root: &Path) -> DocAuditContext {
    let documents = utility_doc_collector::collect_documents(root);
    let master = utility_doc_collector::master_text(&documents);
    DocAuditContext::new(root, documents, master)
}

/// The five capabilities as one uniform callable, so a test can run every
/// seam over one document set and attribute each finding to its owner.
fn codes_from_all(root: &Path) -> Vec<&'static str> {
    let context = context_for(root);
    let mut findings: Vec<DocFinding> = Vec::new();
    findings.extend(FrFormatChecker::new().audit_fr_format(&context));
    findings.extend(SectionStructureChecker::new().audit_section_structure(&context));
    findings.extend(SpecPurityChecker::new().audit_spec_purity(&context));
    findings.extend(CrosslinkChecker::new().audit_crosslinks(&context));
    findings.extend(HeadingStructureChecker::new().audit_doc_heading(&context));
    findings.into_iter().map(|f| f.code).collect()
}

/// An FRD that violates exactly one rule: it names a source file, so only the
/// AES603 seam should answer. Every other invariant holds.
fn frd_violating_only_spec_purity() -> String {
    r"# FRD — sample

## Reference

- PRD: [PRD.md](../../PRD.md)
- Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature.

## System Overview

The feature reads its settings from main.rs at startup.

## Functional Requirements

### FR-SAMPLE-001: Document Invariant Enforcement

- **Description**: Every `.md` document satisfies the AES heading and section contract.
- **Input**: A workspace root and the list of recognized document paths.
- **Output**: `Vec<DocFinding>` carrying one finding per invariant violation.
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
| `audit` | `DocRequest` | `DocResponse` | Reason-coded | — | Single composite entry point. |

### Aggregate API

| Method | Input | Output | Error | Event | Description |
| --- | --- | --- | --- | --- | --- |
| `execute` | `DocRequest` | `DocResponse` | Reason-coded | — | Routes to the capability. |

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
"
    .to_string()
}

/// A PRD that violates exactly one rule: it drops its BACKLOG link is not
/// possible on a PRD, so instead it drops a required H2, exercising AES605
/// alone.
fn prd_missing_required_h2() -> String {
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
"
    .to_string()
}

/// Lay down the minimal feature folder the audit expects to find.
fn write_feature(root: &Path) {
    let feature = root.join("crates/sample");
    std::fs::create_dir_all(feature.join("src")).unwrap();
    std::fs::write(
        feature.join("BACKLOG.md"),
        "# Feature Backlog: Sample\n\n## Current Condition\n\n- Done: nothing yet\n\n## Backlog\n\n| ID | Priority | State | Health | Dependencies | Next Action | Updated |\n|---|---:|---|---|---|---|---|\n| SAM-01 | P0 | Ready | On Track | — | — | 2026-09-29 |\n\n## Scenario Evidence\n\n| Scenario | Kind | Test file | Test name | Last verified |\n|---|---|---|---|---|\n| nothing yet | Automated | tests/ | test_nothing | 2026-09-29 |\n\n## Blockers\n\nNone\n\n## Dependencies\n\nNone\n\n## Release Readiness\n\n| Area | Status | Notes |\n|---|---|---|\n| Tests | Done | nothing |\n\n## Deferred\n\nNone\n\n## Change Log\n\n| Date | Change | By |\n|---|---|---|\n| 2026-09-29 | init | @raka |\n",
    )
    .unwrap();
    std::fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();
}

#[test]
fn the_spec_purity_seam_is_the_only_one_answering_a_source_file_reference() {
    let tmp = tempfile::tempdir().unwrap();
    write_feature(tmp.path());
    std::fs::write(
        tmp.path().join("crates/sample/FRD.md"),
        frd_violating_only_spec_purity(),
    )
    .unwrap();

    let context = context_for(tmp.path());
    let purity = SpecPurityChecker::new().audit_spec_purity(&context);
    assert!(
        purity.iter().any(|f| f.code == "AES603"),
        "the AES603 seam answers a source-file reference; got: {purity:#?}"
    );
    // Every other seam must stay silent: the document satisfies all of them.
    for finding in FrFormatChecker::new().audit_fr_format(&context) {
        assert_eq!(finding.code, "AES601", "an FR-ID finding: {finding:#?}");
    }
    assert!(
        SectionStructureChecker::new()
            .audit_section_structure(&context)
            .is_empty(),
        "the AES602 seam must stay silent on a conforming section shape"
    );
    assert!(
        CrosslinkChecker::new()
            .audit_crosslinks(&context)
            .is_empty(),
        "the AES604 seam must stay silent when both Reference links are present"
    );
    assert!(
        HeadingStructureChecker::new()
            .audit_doc_heading(&context)
            .is_empty(),
        "the AES605 seam must stay silent on a conforming heading structure"
    );
}

#[test]
fn the_doc_heading_seam_is_the_only_one_answering_a_missing_h2() {
    let tmp = tempfile::tempdir().unwrap();
    write_feature(tmp.path());
    std::fs::write(tmp.path().join("PRD.md"), prd_missing_required_h2()).unwrap();

    let context = context_for(tmp.path());
    let heading = HeadingStructureChecker::new().audit_doc_heading(&context);
    assert!(
        heading
            .iter()
            .any(|f| f.code == "AES605" && f.violation_type == "h2_missing"),
        "the AES605 seam answers a missing H2; got: {heading:#?}"
    );
    assert!(
        FrFormatChecker::new().audit_fr_format(&context).is_empty(),
        "the AES601 seam must stay silent on a document with no requirement headings"
    );
    assert!(
        SectionStructureChecker::new()
            .audit_section_structure(&context)
            .is_empty(),
        "the AES602 seam is scoped to requirement documents, not a PRD"
    );
    assert!(
        CrosslinkChecker::new()
            .audit_crosslinks(&context)
            .is_empty(),
        "the AES604 seam is scoped to requirement documents, not a PRD"
    );
}

#[test]
fn running_all_five_seams_over_a_clean_workspace_reports_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    write_feature(tmp.path());
    std::fs::write(
        tmp.path().join("crates/sample/FRD.md"),
        frd_violating_only_spec_purity().replace("main.rs", "the settings role"),
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("PRD.md"),
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
",
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("ROADMAP.md"),
        r"# ROADMAP — sample

## Current Condition

- Todo: none

## State Definitions

| State | Meaning |
|---|---|
| Done | Shipped. |

## Status Policy

A row is Done with a command.

## Feature Roll-up

| ID | Item |
|---|---|
| WS-1 | Docs. |

## Risk Register

| Risk | Mitigation |
|---|---|
| None | — |
",
    )
    .unwrap();

    assert!(
        codes_from_all(tmp.path()).is_empty(),
        "five seams over a conforming workspace must report nothing"
    );
}

#[test]
fn the_spec_purity_seam_answers_specs_only_and_never_a_backlog() {
    // A backlog's job is to report progress, so a status leak there is that
    // document working as intended. Only the promise-bearing documents are held
    // to purity, and this is the test that pins that scoping down.
    let tmp = tempfile::tempdir().unwrap();
    let feature = tmp.path().join("crates/sample");
    std::fs::create_dir_all(feature.join("src")).unwrap();
    std::fs::write(
        feature.join("FRD.md"),
        frd_violating_only_spec_purity().replace("main.rs", "the settings role"),
    )
    .unwrap();
    std::fs::write(
        feature.join("BACKLOG.md"),
        "# Feature Backlog: Sample\n\n## Current Condition\n\n- [x] the FR-ID rule landed\n- SAM-01 is 90% complete and shipped in v3.7\n\n## Backlog\n\n| ID | Priority | State | Health | Dependencies | Next Action | Updated |\n|---|---:|---|---|---|---|---|\n| SAM-02 | P0 | Ready | On Track | — | — | 2026-09-29 |\n\n## Scenario Evidence\n\n| Scenario | Kind | Test file | Test name | Last verified |\n|---|---|---|---|---|\n| nothing yet | Automated | tests/ | test_nothing | 2026-09-29 |\n\n## Blockers\n\nNone\n\n## Dependencies\n\nNone\n\n## Release Readiness\n\n| Area | Status | Notes |\n|---|---|---|\n| Tests | Done | nothing |\n\n## Deferred\n\nNone\n\n## Change Log\n\n| Date | Change | By |\n|---|---|---|\n| 2026-09-29 | init | @raka |\n",
    )
    .unwrap();
    std::fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();

    let purity = SpecPurityChecker::new().audit_spec_purity(&context_for(tmp.path()));
    assert!(
        purity.is_empty(),
        "a backlog reporting progress is not a spec-purity violation; got: {purity:#?}"
    );
}

#[test]
fn a_document_violating_two_rules_reports_both_codes() {
    // The fan-out is the agent's job: two seams must both answer rather than
    // the first one short-circuiting the rest.
    let tmp = tempfile::tempdir().unwrap();
    write_feature(tmp.path());
    let frd = frd_violating_only_spec_purity().replace("- PRD: [PRD.md](../../PRD.md)\n", "");
    std::fs::write(tmp.path().join("crates/sample/FRD.md"), frd).unwrap();
    std::fs::write(tmp.path().join("PRD.md"), prd_missing_required_h2()).unwrap();

    let codes = codes_from_all(tmp.path());
    assert!(
        codes.contains(&"AES603"),
        "the source-file reference must be reported; got: {codes:?}"
    );
    assert!(
        codes.contains(&"AES604"),
        "the missing PRD crosslink must be reported; got: {codes:?}"
    );
    assert!(
        codes.contains(&"AES605"),
        "the missing PRD H2 must be reported; got: {codes:?}"
    );
}
