// PURPOSE: integration tests for doc-rules — the real DI container over a real
// workspace on disk.
//
// The unit tests drive the capabilities directly and the contract tests check
// the seams exist. These go through `RootDocRulesContainer`, so the wiring from
// root container to orchestrator to capabilities is what is under test: a broken
// seam shows up here as a capability that never runs.
use doc_rules_lint_arwaky::root_doc_rules_container::RootDocRulesContainer;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;

use std::fs;
use std::path::Path;

/// Write *contents* as a file at *path*, creating parent dirs.
fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("fixture parent dir must be creatable");
    }
    fs::write(path, contents).expect("fixture file must be writable");
}

/// Audit *root* through the public container and collect (code, violation_type).
fn audit(root: &Path) -> Vec<(String, String)> {
    let aggregate = RootDocRulesContainer::orchestrator();
    match aggregate.execute(DocRequest::audit_all(root)) {
        DocResponse::Findings { findings } => findings
            .into_iter()
            .map(|f| (f.code.to_string(), f.violation_type.to_string()))
            .collect(),
    }
}

/// Does any finding carry this (code, violation_type) pair?
fn has(findings: &[(String, String)], code: &str, violation_type: &str) -> bool {
    findings
        .iter()
        .any(|(c, v)| c == code && v == violation_type)
}

/// A feature folder carrying both layers plus its doc pair, so the folder-shape
/// rules stay silent and only the document rules have something to say.
fn write_feature(root: &Path, layout: &str, frd: &str) {
    let feature = root.join(layout).join("sample");
    write(
        &feature.join("src/capabilities_sample_checker.rs"),
        "pub struct SampleChecker;\n",
    );
    write(
        &feature.join("src/agent_sample_orchestrator.rs"),
        "pub struct SampleOrchestrator;\n",
    );
    write(&feature.join("FRD.md"), frd);
    write(
        &feature.join("BACKLOG.md"),
        "# Backlog\n\nFRD: [FRD.md](FRD.md)\n",
    );
    write(&root.join("PRD.md"), "# PRD\n");
    write(&root.join("ROADMAP.md"), "# ROADMAP\n");
}

/// A minimal FRD whose heading shape is legal, so AES605 has nothing to report
/// and every finding in these tests belongs to the rule actually under test.
fn frd_with_one_requirement() -> String {
    String::from(
        "# FRD — sample\n\n\
         ## Reference\n\n\
         - PRD: [PRD.md](../../PRD.md)\n\n\
         ## Functional Requirements\n\n\
         ### FR-SAMPLE-001: Do The Thing\n\n\
         - **Description**: The feature performs a responsibility.\n\
         - **Input**: A request value object.\n\
         - **Output**: A response value object.\n\
         - **Business Rules**: The capability is stateless.\n\
         - **Edge Cases**: An absent input yields a default.\n\
         - **Error Handling**: Failures surface as a reason-coded outcome.\n",
    )
}

#[test]
fn container_runs_the_document_audit_over_a_workspace_on_disk() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_feature(root, "crates", &frd_with_one_requirement());

    let findings = audit(root);
    assert!(
        !findings.is_empty(),
        "a workspace whose FRD declares one requirement against zero contract \
         seams must be reported; an empty result would mean the container is not \
         dispatching to any capability"
    );
}

#[test]
fn container_fires_the_document_rules_across_every_member_layout() {
    // The doc auditor walks each member layout independently, so a wiring gap in
    // one of them would show up as silence rather than as an error.
    for layout in ["crates", "modules", "packages"] {
        let tmp = tempfile::tempdir().expect("temp dir must be creatable");
        let root = tmp.path();
        write_feature(root, layout, &frd_with_one_requirement());

        let findings = audit(root);
        assert!(
            findings.iter().any(|(code, _)| code.starts_with("AES60")),
            "the {layout} layout must reach the document capabilities; got {findings:?}"
        );
    }
}

#[test]
fn container_reports_a_missing_backlog_link() {
    // One named rule end-to-end: the FRD names no backlog, so the crosslink
    // capability has something to report and the container must carry it out.
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_feature(root, "crates", &frd_with_one_requirement());

    let findings = audit(root);
    assert!(
        has(&findings, "AES604", "no_backlog_link"),
        "an FRD with no backlog crosslink must fire AES604 through the container; \
         got {findings:?}"
    );
}

#[test]
fn container_stays_silent_on_a_workspace_with_no_recognized_documents() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write(
        &root.join("crates/sample/src/capabilities_sample_checker.rs"),
        "pub struct SampleChecker;\n",
    );

    assert_eq!(
        audit(root),
        Vec::new(),
        "a workspace carrying no recognized document is not a document defect"
    );
}
