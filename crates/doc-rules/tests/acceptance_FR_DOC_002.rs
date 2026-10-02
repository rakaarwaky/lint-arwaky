// PURPOSE: acceptance test for FR-DOC-002 — the document auditor is reachable
// through one aggregate entry point that routes to every invariant capability.
//
// Maps 1:1 to the FRD requirement: a consumer holding only the container must be
// able to run the whole document audit and receive a deduplicated, stable-sorted
// finding list, without naming a single capability.
use doc_rules_lint_arwaky::root_doc_rules_container::RootDocRulesContainer;
use shared_doc_rules::taxonomy_doc_rules_request::DocRequest;
use shared_doc_rules::taxonomy_doc_rules_response::DocResponse;

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

/// Write *contents* as a file at *path*, creating parent dirs.
fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("fixture parent dir must be creatable");
    }
    fs::write(path, contents).expect("fixture file must be writable");
}

/// Audit *root* through the public container and return the findings.
fn audit(root: &Path) -> Vec<(String, String, String)> {
    let aggregate = RootDocRulesContainer::orchestrator();
    match aggregate.execute(DocRequest::audit_all(root)) {
        DocResponse::Findings { findings } => findings
            .into_iter()
            .map(|f| {
                (
                    f.code.to_string(),
                    f.violation_type.to_string(),
                    f.doc.clone(),
                )
            })
            .collect(),
    }
}

/// A feature folder carrying both layers and a document that drifts on every
/// axis at once, so one request exercises more than one capability.
fn write_faulty_workspace(root: &Path) {
    let feature = root.join("crates/sample");
    write(
        &feature.join("src/capabilities_sample_checker.rs"),
        "pub struct SampleChecker;\n",
    );
    write(
        &feature.join("src/agent_sample_orchestrator.rs"),
        "pub struct SampleOrchestrator;\n",
    );
    write(
        &feature.join("BACKLOG.md"),
        "# Backlog\n\nFRD: [FRD.md](FRD.md)\n",
    );
    write(&root.join("PRD.md"), "# PRD\n");
    write(&root.join("ROADMAP.md"), "# ROADMAP\n");
    // Four independent defects in one document: an FR-ID missing its feature
    // prefix (AES601), Test Scenarios placed after Glossary so the sections are
    // out of template order (AES602), a spec naming a source file (AES603), and
    // PRD.md carrying no mandated H2 (AES605).
    write(
        &feature.join("FRD.md"),
        "# FRD — sample\n\n\
         ## Reference\n\n\
         - PRD: [PRD.md](../../PRD.md)\n\
         - Backlog: [BACKLOG.md](BACKLOG.md)\n\n\
         ## System Overview\n\n\
         Implemented in `src/capabilities_sample_checker.rs`.\n\n\
         ## Functional Requirements\n\n\
         ### FR-001: Do The Thing\n\n\
         - **Description**: The feature performs a responsibility.\n\
         - **Input**: A request value object.\n\
         - **Output**: A response value object.\n\
         - **Business Rules**: The capability is stateless.\n\
         - **Edge Cases**: An absent input yields a default.\n\
         - **Error Handling**: Failures surface as a reason-coded outcome.\n\n\
         ## API Contract\n\n\
         ### Protocol API\n\n\
         | Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | execute | request | response | none | none | single entry |\n\n\
         ### Aggregate API\n\n\
         | Method | Input | Output | Error | Event | Description |\n\
         | --- | --- | --- | --- | --- | --- |\n\
         | execute | request | response | none | none | routes to capability |\n\n\
         ## Integration Points\n\n\
         | System | Direction | Purpose | Failure mode |\n\
         | --- | --- | --- | --- |\n\
         | filesystem | out | read source | missing path |\n\n\
         ## Non-functional Requirements\n\n\
         | Metric | Target | Measurement method |\n\
         | --- | --- | --- |\n\
         | audit time | under 1s | wall clock |\n\n\
         ## Assumptions & Constraints\n\n\
         - none\n\n\
         ## Glossary\n\n\
         - **Finding**: a rule violation\n\n\
         ## Test Scenarios\n\n\
         - audits a clean workspace\n",
    );
}

#[test]
fn one_request_runs_every_invariant_and_reports_each_defect() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_faulty_workspace(root);

    let findings = audit(root);
    let codes: BTreeSet<&str> = findings.iter().map(|(code, _, _)| code.as_str()).collect();

    for expected in ["AES601", "AES602", "AES603"] {
        assert!(
            codes.contains(expected),
            "a single request must reach the {expected} capability; the aggregate \
             promises to route every invariant. Codes seen: {codes:?}"
        );
    }
}

#[test]
fn repeated_requests_agree_on_the_finding_list() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_faulty_workspace(root);

    let first = audit(root);
    let second = audit(root);
    assert_eq!(
        first, second,
        "two runs over the same workspace must report the same findings in the same \
         order; the FR promises a deduplicated, stable-sorted list"
    );
}

#[test]
fn repeated_findings_are_collapsed() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_faulty_workspace(root);

    let findings = audit(root);
    let distinct: BTreeSet<(&String, &String, &String)> =
        findings.iter().map(|(c, v, d)| (c, v, d)).collect();
    assert_eq!(
        distinct.len(),
        findings.len(),
        "one defect must produce one finding; a duplicated (code, violation, document) \
         triple means the audit ran the same capability twice"
    );
}
