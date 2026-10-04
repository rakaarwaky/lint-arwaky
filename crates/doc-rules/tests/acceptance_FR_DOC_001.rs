// PURPOSE: acceptance test for FR-DOC-001 — an FRD's requirement count equals
// its feature's count of `I*Protocol` capability-seam classes (AES601). One
// acceptance file per FR, named after the FR ID.
use doc_rules_lint_arwaky::root_doc_rules_container::RootDocRulesContainer;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;

use std::fs;
use std::path::Path;

const CODE: &str = "AES601";
const VIOLATION: &str = "protocol_count_mismatch";

/// A minimal FRD carrying *count* well-formed requirement headings.
fn frd_with_requirements(count: usize) -> String {
    let mut frd = String::from(
        "# FRD — sample\n\n\
         ## Reference\n\n\
         - PRD: [PRD.md](../../PRD.md)\n\
         - Backlog: [BACKLOG.md](BACKLOG.md) — real condition for this feature.\n\n\
         ## Functional Requirements\n",
    );
    for n in 1..=count {
        frd.push_str(&format!(
            "\n### FR-SAMPLE-{n:03}: Do The Thing\n\n\
             - **Description**: The feature performs a responsibility.\n\
             - **Input**: A request value object.\n\
             - **Output**: A response value object.\n\
             - **Business Rules**: The capability is stateless.\n\
             - **Edge Cases**: An absent input yields a default.\n\
             - **Error Handling**: Failures surface as a reason-coded outcome.\n"
        ));
    }
    frd
}

/// A conforming BACKLOG.md for test fixtures.
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

/// Lay down the feature folder and the documents the audit expects to find.
fn write_feature(root: &Path, frd: &str) {
    let feature = root.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    fs::write(feature.join("FRD.md"), frd).unwrap();
    fs::write(feature.join("BACKLOG.md"), conforming_backlog()).unwrap();
    fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();
}

/// Declare *classes* protocol seams plus one aggregate, spread over two files
/// when there is more than one, so the count proves classes are counted and
/// files are not.
fn write_contract_module(root: &Path, classes: usize) {
    let module = root.join("crates/shared/src/sample");
    fs::create_dir_all(&module).unwrap();
    let split = classes > 1;
    let mut first = String::from("//! sample contract module\n\n");
    for n in 0..classes {
        if split && n + 1 == classes {
            continue;
        }
        first.push_str(&format!(
            "pub trait ISample{n}Protocol: Send + Sync {{}}\n\n"
        ));
    }
    first.push_str("pub trait ISampleAggregate: Send + Sync {}\n");
    fs::write(module.join("contract_sample_protocol.rs"), first).unwrap();
    if split {
        let last = classes - 1;
        fs::write(
            module.join("contract_sample_extra_protocol.rs"),
            format!(
                "//! second contract file\n\npub trait ISample{last}Protocol: Send + Sync {{}}\n"
            ),
        )
        .unwrap();
    }
}

/// Run the audit and return `(code, violation_type, message, fix)` tuples.
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

/// The parity finding, if the audit produced one.
fn parity_message(findings: &[(String, String, String, String)]) -> Option<(String, String)> {
    findings
        .iter()
        .find(|(c, v, _, _)| c == CODE && v == VIOLATION)
        .map(|(_, _, m, fix)| (m.clone(), fix.clone()))
}

#[test]
fn fr_doc_001_accepts_equal_requirement_and_protocol_class_counts() {
    let tmp = tempfile::tempdir().unwrap();
    write_feature(tmp.path(), &frd_with_requirements(3));
    write_contract_module(tmp.path(), 3);
    let findings = audit(tmp.path());
    assert!(
        parity_message(&findings).is_none(),
        "3 requirements and 3 protocol classes are aligned; got: {findings:#?}"
    );
}

#[test]
fn fr_doc_001_rejects_more_requirements_than_protocol_classes() {
    let tmp = tempfile::tempdir().unwrap();
    write_feature(tmp.path(), &frd_with_requirements(3));
    write_contract_module(tmp.path(), 2);
    let (message, fix) =
        parity_message(&audit(tmp.path())).expect("3 requirements vs 2 classes must fire");
    assert!(
        message.contains("3 requirements") && message.contains("2 protocol classes"),
        "the message states both counts; got: {message}"
    );
    assert!(
        fix.contains("merge the requirements down")
            && fix.contains("split the methods into more classes"),
        "the fix names both directions; got: {fix}"
    );
}

#[test]
fn fr_doc_001_rejects_more_protocol_classes_than_requirements() {
    let tmp = tempfile::tempdir().unwrap();
    write_feature(tmp.path(), &frd_with_requirements(1));
    write_contract_module(tmp.path(), 3);
    let (message, fix) =
        parity_message(&audit(tmp.path())).expect("1 requirement vs 3 classes must fire");
    assert!(
        message.contains("1 requirement") && message.contains("3 protocol classes"),
        "the message states both counts; got: {message}"
    );
    assert!(
        fix.contains("split the requirements up to match")
            && fix.contains("merge the classes down"),
        "the fix names both directions; got: {fix}"
    );
}

#[test]
fn fr_doc_001_excludes_aggregate_traits_from_the_seam_count() {
    let tmp = tempfile::tempdir().unwrap();
    // One requirement, one protocol class, one aggregate: the aggregate is a
    // composite entry point, not a capability seam, so the counts align.
    write_feature(tmp.path(), &frd_with_requirements(1));
    write_contract_module(tmp.path(), 1);
    assert!(
        parity_message(&audit(tmp.path())).is_none(),
        "an aggregate trait is not a capability seam"
    );
}

#[test]
fn fr_doc_001_leaves_a_feature_without_a_contract_module_alone() {
    let tmp = tempfile::tempdir().unwrap();
    write_feature(tmp.path(), &frd_with_requirements(2));
    assert!(
        parity_message(&audit(tmp.path())).is_none(),
        "a feature with no shared contract module cannot mismatch"
    );
}

#[test]
fn fr_doc_001_anchors_the_finding_to_a_line() {
    let tmp = tempfile::tempdir().unwrap();
    write_feature(tmp.path(), &frd_with_requirements(2));
    write_contract_module(tmp.path(), 1);
    let (message, _) =
        parity_message(&audit(tmp.path())).expect("2 requirements vs 1 class must fire");
    assert!(
        message.starts_with("line "),
        "every doc finding names a line so reports can anchor it; got: {message}"
    );
}

// ── FR-DOC-001: the promised API methods exist ──────────────────────────────

/// An FRD whose `## API Contract` promises *protocol_method* and
/// `execute`, with the requirement headings the other tests rely on.
fn frd_promising_methods(protocol_method: &str) -> String {
    let frd = frd_with_requirements(1);
    format!(
        "{frd}\n\
         ## API Contract\n\n\
         ### Protocol API\n\n\
         | Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | `{protocol_method}` | `SampleRequest` | `SampleResponse` | — | — | One seam. |\n\n\
         ### Aggregate API\n\n\
         | Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | `execute` | `SampleRequest` | `SampleResponse` | — | — | Single entry point. |\n"
    )
}

/// Declare *classes* protocol seams, each carrying *method*, plus an aggregate
/// carrying `execute`.
fn write_contract_module_with_methods(root: &Path, classes: usize, method: &str) {
    let module = root.join("crates/shared/src/sample");
    fs::create_dir_all(&module).unwrap();
    let mut first = String::from("//! sample contract module\n\n");
    for n in 0..classes {
        first.push_str(&format!(
            "pub trait ISample{n}Protocol: Send + Sync {{\n    fn {method}(&self);\n}}\n\n"
        ));
    }
    first.push_str("pub trait ISampleAggregate: Send + Sync {\n    fn execute(&self);\n}\n");
    fs::write(module.join("contract_sample_protocol.rs"), first).unwrap();
}

/// The method-existence finding, if the audit produced one.
fn method_message(findings: &[(String, String, String, String)]) -> Option<(String, String)> {
    findings
        .iter()
        .find(|(c, v, _, _)| c == CODE && v == "api_method_not_found")
        .map(|(_, _, m, fix)| (m.clone(), fix.clone()))
}

#[test]
fn fr_doc_001_accepts_a_promised_method_the_contract_module_declares() {
    let tmp = tempfile::tempdir().unwrap();
    write_feature(tmp.path(), &frd_promising_methods("audit_thing"));
    write_contract_module_with_methods(tmp.path(), 1, "audit_thing");
    assert!(
        method_message(&audit(tmp.path())).is_none(),
        "a method the seams declare satisfies its table row"
    );
}

#[test]
fn fr_doc_001_fires_when_a_promised_method_is_never_declared() {
    let tmp = tempfile::tempdir().unwrap();
    // The FRD promises `audit_ghost`; the seam declares something else. The
    // requirement count still matches the class count, so only the
    // method-existence check can answer this document.
    write_feature(tmp.path(), &frd_promising_methods("audit_ghost"));
    write_contract_module_with_methods(tmp.path(), 1, "audit_thing");
    let (message, _) =
        method_message(&audit(tmp.path())).expect("a promised method the code lacks must fire");
    assert!(
        message.contains("`audit_ghost`") && message.contains("Protocol API"),
        "the message names the method and the table it sits in; got: {message}"
    );
}

#[test]
fn fr_doc_001_offers_both_remedies_for_a_missing_method() {
    let tmp = tempfile::tempdir().unwrap();
    write_feature(tmp.path(), &frd_promising_methods("audit_ghost"));
    write_contract_module_with_methods(tmp.path(), 1, "audit_thing");
    let (_, fix) =
        method_message(&audit(tmp.path())).expect("a promised method the code lacks must fire");
    assert!(
        fix.contains("Remove `audit_ghost` from the"),
        "the fix must offer to delete the promise; got: {fix}"
    );
    assert!(
        fix.contains("create the protocol/aggregate method"),
        "the fix must offer to declare the missing method; got: {fix}"
    );
}

#[test]
fn fr_doc_001_anchors_a_missing_method_to_its_table_row() {
    let tmp = tempfile::tempdir().unwrap();
    let frd = frd_promising_methods("audit_ghost");
    write_feature(tmp.path(), &frd);
    write_contract_module_with_methods(tmp.path(), 1, "audit_thing");
    let row_line = fs::read_to_string(tmp.path().join("crates/sample/FRD.md"))
        .unwrap()
        .lines()
        .position(|l| l.contains("`audit_ghost`"))
        .unwrap()
        + 1;
    let (message, _) = method_message(&audit(tmp.path())).expect("the missing method must fire");
    assert!(
        message.contains(&format!("line {row_line}")),
        "the finding anchors to its own row at line {row_line}; got: {message}"
    );
}

#[test]
fn fr_doc_001_reports_a_missing_aggregate_method_too() {
    let tmp = tempfile::tempdir().unwrap();
    // The Aggregate API table promises `dispatch`, which nothing declares.
    let frd = frd_promising_methods("audit_thing").replace("| `execute` |", "| `dispatch` |");
    write_feature(tmp.path(), &frd);
    write_contract_module_with_methods(tmp.path(), 1, "audit_thing");
    let (message, _) =
        method_message(&audit(tmp.path())).expect("an aggregate method the code lacks must fire");
    assert!(
        message.contains("`dispatch`") && message.contains("Aggregate API"),
        "the aggregate table is audited the same way as the protocol table; got: {message}"
    );
}

#[test]
fn fr_doc_001_leaves_api_methods_alone_without_a_contract_module() {
    let tmp = tempfile::tempdir().unwrap();
    // No shared contract module, so no method can be checked: the finding is
    // a parse skip, never a violation.
    write_feature(tmp.path(), &frd_promising_methods("audit_ghost"));
    assert!(
        method_message(&audit(tmp.path())).is_none(),
        "a feature with no shared contract module cannot mismatch"
    );
}
