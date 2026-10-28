// PURPOSE: contract tests for the six capability seams — each answers exactly
// one AES doc rule, and each is reachable only through its own protocol trait.
//
// The aggregate tests in `contract_doc_rules.rs` prove the rules fire end to
// end. These prove the thing the split introduced: a capability wired to the
// wrong trait, or answering a second rule, is caught here rather than by a
// consumer's surprise report.
use doc_rules_lint_arwaky::capabilities_doc_crosslink_checker::CrosslinkChecker;
use doc_rules_lint_arwaky::capabilities_doc_fr_format_checker::FrFormatChecker;
use doc_rules_lint_arwaky::capabilities_doc_heading_structure_checker::HeadingStructureChecker;
use doc_rules_lint_arwaky::capabilities_doc_required_doc_checker::RequiredDocChecker;
use doc_rules_lint_arwaky::capabilities_doc_section_structure_checker::SectionStructureChecker;
use doc_rules_lint_arwaky::capabilities_doc_spec_purity_checker::SpecPurityChecker;
use doc_rules_lint_arwaky::capabilities_doc_template_conformance_checker::TemplateConformanceChecker;
use shared_doc_rules::contract_doc_protocol::{
    ICrosslinkProtocol, IDocHeadingProtocol, IFrFormatProtocol, IRequiredDocProtocol,
    ISectionStructureProtocol, ISpecPurityProtocol, ITemplateConformanceProtocol,
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
    let members = utility_doc_collector::collect_members(root);
    DocAuditContext::new(
        root,
        documents,
        master,
        members,
        None,
        shared_doc_rules::taxonomy_doc_audit_context_vo::StrictModeOverride::UseConfigFile,
    )
}

/// The six capabilities as one uniform callable, so a test can run every
/// seam over one document set and attribute each finding to its owner.
fn codes_from_all(root: &Path) -> Vec<&'static str> {
    let context = context_for(root);
    let mut findings: Vec<DocFinding> = Vec::new();
    findings.extend(FrFormatChecker::new().audit_fr_format(&context));
    findings.extend(SectionStructureChecker::new().audit_section_structure(&context));
    findings.extend(SpecPurityChecker::new().audit_spec_purity(&context));
    findings.extend(CrosslinkChecker::new().audit_crosslinks(&context));
    findings.extend(HeadingStructureChecker::new().audit_doc_heading(&context));
    findings.extend(TemplateConformanceChecker::new().audit_template_conformance(&context));
    findings.extend(RequiredDocChecker::new().audit_required_docs(&context));
    findings.into_iter().map(|f| f.code).collect()
}

/// An FRD that violates exactly one rule: it names a source file, so only the
/// AES603 seam should answer. Every other invariant holds.
fn frd_violating_only_spec_purity() -> String {
    r"# FRD — sample

## Feature Overview

The feature reads its settings from main.rs at startup.

## Functional Requirements

### FR-SAMPLE-001: Document Invariant Enforcement

| Aspect         | Detail                                                          |
|----------------|-----------------------------------------------------------------|
| Description    | Every `.md` document satisfies the heading and section contract. |
| Business Rules | Validates FR-ID format, section order, heading structure.       |
| Edge Cases     | A root-level legacy `BACKLOG.md` is accepted during migration.  |
| Capability     | doc_rules::DocStructure                                         |
| Protocol       | doc_rules::DocStructure                                         |
| Method         | check_doc_heading                                               |
| Input          | A workspace root and recognized document paths.                 |
| Output         | One finding per invariant violation.                            |
| Error          | Unreadable files produce no findings.                           |
| Event          | Finding emitted                                                 |
| Test           | The seam contract tests stay green                               |
| Metric         | Deterministic finding order                                      |

### FR-SAMPLE-002: Audit Orchestration

| Aspect         | Detail                                                          |
|----------------|-----------------------------------------------------------------|
| Description    | The auditor is reachable through a single aggregate entry point. |
| Business Rules | The aggregate orchestrates all invariants under one seam.       |
| Edge Cases     | An empty root produces no findings rather than an error.         |
| Capability     | doc_rules::DocStructure                                         |
| Protocol       | doc_rules::DocStructure                                         |
| Method         | execute                                                         |
| Input          | A request describing the workspace root to audit.                |
| Output         | A response carrying a deduplicated finding list.                 |
| Error          | Failures propagate with no partial results.                      |
| Event          | Finding emitted                                                 |
| Test           | The seam contract tests stay green                               |
| Metric         | Deterministic finding order                                      |

## Public Interface

Implemented in `src/capabilities_sample_checker.rs`.

| Aspect | Detail |
| --- | --- |
| Method | `execute` |

## Reference

| Artifact     | Purpose                          | Link |
|--------------|----------------------------------|------|
| PRD          | Business goals and scope         | [PRD.md](../../PRD.md) |
| Backlog      | Task tracking and status         | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | What ships next, in order        | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries and infra      | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
"
    .to_string()
}

/// The source-file-referencing FRD with every AES603 violation removed: no
/// source-file name in the prose and no implementation-state claim, so a
/// purity-clean workspace can reuse the same section shape.
fn purity_clean_frd() -> String {
    frd_violating_only_spec_purity()
        .replace("main.rs", "the settings role")
        .replace(
            "Implemented in `src/capabilities_sample_checker.rs`.",
            "Delegates the check to the settings role.",
        )
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
        "# Feature Backlog: Sample\n\n\
## Progress Overview\n\n\
Baseline.\n\n\
## Backlog Status\n\n\
### {FR-SAMPLE-001}\n\
| Aspect       | Detail                                      |\n\
|--------------|---------------------------------------------|\n\
| State        | Ready                                       |\n\
| Done         | none                                        |\n\
| In Progress  | None                                        |\n\
| Blocked      | None                                        |\n\
| Next         | Ship the feature                            |\n\
| Risk         | None                                        |\n\
| Updated      | 2026-09-29                                  |\n\n\
### {FR-SAMPLE-002}\n\
| Aspect       | Detail                                      |\n\
|--------------|---------------------------------------------|\n\
| State        | Ready                                       |\n\
| Done         | none                                        |\n\
| In Progress  | None                                        |\n\
| Blocked      | None                                        |\n\
| Next         | Ship the feature                            |\n\
| Risk         | None                                        |\n\
| Updated      | 2026-09-29                                  |\n\n\
## Reference\n\n\
| Artifact     | Purpose                          | Link |\n|--------------|----------------------------------|------|\n| PRD          | Business goals and scope         | [PRD.md](../../PRD.md) |\n| FRD          | Requirements specification       | [FRD.md](FRD.md) |\n| Roadmap      | What ships next, in order        | [ROADMAP.md](../../ROADMAP.md) |\n| Architecture | System boundaries and infra      | [ARCHITECTURE.md](../../ARCHITECTURE.md) |\n",
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
    // The AES601 seam may answer a BACKLOG ID misalignment on the fixture
    // backlog; no FR-format finding is owed for the document itself.
    let fr_findings = FrFormatChecker::new().audit_fr_format(&context);
    assert!(
        fr_findings
            .iter()
            .all(|f| f.violation_type != "id_missing_feature_prefix"
                && f.violation_type != "field_missing"
                && f.violation_type != "protocol_count_mismatch"
                && f.violation_type != "api_method_not_found"),
        "the AES601 seam must stay silent on the FR document; got: {fr_findings:#?}"
    );
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
    std::fs::write(tmp.path().join("crates/sample/FRD.md"), purity_clean_frd()).unwrap();
    std::fs::write(
        tmp.path().join("PRD.md"),
        r"# PRD — sample

## Problem Statement

The problem.

## Project Requirements

- Fewer defects.

## Reference

| Artifact     | Purpose                          | Link |
|--------------|----------------------------------|------|
| ROADMAP      | What ships next, in order        | [ROADMAP.md](../../ROADMAP.md) |
| Architecture | System boundaries and infra      | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
",
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("ROADMAP.md"),
        r"# ROADMAP — sample

## Progress Overview

- Todo: none
- In Progress: None
- Blocked: None

## Roadmap Status

| Priority | State | Dependencies | Updated |
|---|---|---|---|
| P0 | Done | none | 2026-09-29 |

## Reference

| Artifact     | Purpose                          | Link |
|--------------|----------------------------------|------|
| PRD          | Business goals and scope         | [PRD.md](../../PRD.md) |
| Architecture | System boundaries and infra      | [ARCHITECTURE.md](../../ARCHITECTURE.md) |
",
    )
    .unwrap();

    let codes = codes_from_all(tmp.path());
    assert!(
        codes.is_empty(),
        "six seams over a conforming workspace must report nothing; got codes: {codes:?}"
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
    std::fs::write(feature.join("FRD.md"), purity_clean_frd()).unwrap();
    std::fs::write(
        feature.join("BACKLOG.md"),
        "# Feature Backlog: Sample\n\n## Current Condition\n\n- [x] the FR-ID rule landed\n- SAM-01 is 90% complete and shipped in v3.7\n\n## Backlog\n\n| ID | Priority | State | Dependencies | Next Action | Updated |\n|---|---:|---|---|---|---|\n| SAM-02 | P0 | Ready | — | — | 2026-09-29 |\n\n## Scenario Evidence\n\n| Scenario | Kind | Test file | Test name | Last verified |\n|---|---|---|---|---|\n| nothing yet | Automated | tests/ | test_nothing | 2026-09-29 |\n\n## Blockers\n\nNone\n\n## Dependencies\n\nNone\n\n## Release Readiness\n\n| Area | Status | Notes |\n|---|---|---|\n| Tests | Done | nothing |\n\n## Deferred\n\nNone\n\n## Change Log\n\n| Date | Change | By |\n|---|---|---|\n| 2026-09-29 | init | @raka |\n",
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
    let frd = frd_violating_only_spec_purity().replace("[PRD.md](../../PRD.md)", "");
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

/// A DESIGN.md declaring a qualified `DS-DISP-001` heading with a
/// conforming BACKLOG.
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

/// A BACKLOG whose `## Backlog Status` entries carry the given H3 full-ID tokens,
/// each followed by its State/Done/In Progress/Blocked/Next/Risk/Updated rows.
fn backlog_with_entries(ids: &[&str]) -> String {
    let entries = ids
        .iter()
        .map(|id| {
            format!(
                "### {{{id}}}\n\n\
                 | Aspect       | Detail                                      |\n\
                 |--------------|---------------------------------------------|\n\
                 | State        | Ready                                       |\n\
                 | Done         | —                                           |\n\
                 | In Progress  | None                                        |\n\
                 | Blocked      | None                                        |\n\
                 | Next         | Ship {id}                                   |\n\
                 | Risk         | None                                        |\n\
                 | Updated      | 2026-09-29                                  |\n"
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

#[test]
fn the_fr_format_seam_is_the_only_one_answering_a_ds_backlog_mismatch() {
    let tmp = tempfile::tempdir().unwrap();
    let feature = tmp.path().join("crates/sample");
    std::fs::create_dir_all(feature.join("src")).unwrap();
    std::fs::write(feature.join("DESIGN.md"), conforming_design()).unwrap();
    // DESIGN declares DS-DISP-001; BACKLOG entry FR-SAMPLE-001 is not declared
    // by any DS-ID (backlog_qualifier_mismatch), and the declared DS-DISP-001
    // owes no entry (ds_backlog_id_missing).
    std::fs::write(
        feature.join("BACKLOG.md"),
        backlog_with_entries(&["FR-SAMPLE-001"]),
    )
    .unwrap();
    std::fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();

    let context = context_for(tmp.path());
    let fr_findings = FrFormatChecker::new().audit_fr_format(&context);
    let ds_codes: Vec<&str> = fr_findings
        .iter()
        .filter(|f| {
            f.violation_type == "ds_backlog_id_not_ds"
                || f.violation_type == "ds_backlog_id_missing"
        })
        .map(|f| f.code)
        .collect();
    assert!(
        !ds_codes.is_empty(),
        "the AES601 seam must answer a DS BACKLOG mismatch; got: {fr_findings:#?}"
    );
    assert!(
        ds_codes.iter().all(|code| *code == "AES601"),
        "DS alignment findings must carry AES601; got: {ds_codes:?}"
    );
}

#[test]
fn the_fr_format_seam_stays_silent_when_ds_backlog_is_aligned() {
    let tmp = tempfile::tempdir().unwrap();
    let feature = tmp.path().join("crates/sample");
    std::fs::create_dir_all(feature.join("src")).unwrap();
    std::fs::write(feature.join("DESIGN.md"), conforming_design()).unwrap();
    // DESIGN declares DS-DISP-001; BACKLOG carries an entry for it — aligned.
    std::fs::write(
        feature.join("BACKLOG.md"),
        backlog_with_entries(&["DS-DISP-001"]),
    )
    .unwrap();
    std::fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();

    let fr_findings = FrFormatChecker::new().audit_fr_format(&context_for(tmp.path()));
    assert!(
        fr_findings
            .iter()
            .all(|f| f.violation_type != "ds_backlog_id_not_ds"
                && f.violation_type != "ds_backlog_id_missing"),
        "an aligned DS pair must report no ds_* findings; got: {fr_findings:#?}"
    );
}
