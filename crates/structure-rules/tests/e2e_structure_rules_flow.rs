// PURPOSE: e2e test for structure-rules — the full request lifecycle over real
// workspaces, from folders on disk through the aggregate to a named finding.
//
// The integration test proves the container reaches each seam; this proves one
// complete journey per rule: build a workspace whose layout drifts, run the real
// container over it, and read findings that name both the defect and the folder
// it sits in.
use shared_structure_rules::taxonomy_structure_rules_request::StructureRequest;
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;
use structure_rules_lint_arwaky::root_structure_rules_container::RootStructureRulesContainer;

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
///
/// The message is carried alongside the code so a case can assert on *which*
/// category was named — a `test_suite_missing_category` finding locates the
/// folder in `file` and names the absent prefix in its message.
fn audit(root: &Path) -> Vec<(String, String, String, String)> {
    let aggregate = RootStructureRulesContainer::orchestrator();
    match aggregate.execute(StructureRequest::audit_all(root)) {
        StructureResponse::Findings { findings } => findings
            .into_iter()
            .map(|f| (f.code, f.violation_type, f.file, f.message))
            .collect(),
    }
}

/// The findings carrying a given (code, violation_type) pair.
fn matching<'a>(
    findings: &'a [(String, String, String, String)],
    code: &str,
    violation: &str,
) -> Vec<&'a (String, String, String, String)> {
    findings
        .iter()
        .filter(|(c, v, _, _)| c == code && v == violation)
        .collect()
}

/// A complete feature folder with a complete eight-category test suite.
fn write_conforming_feature(root: &Path, layout: &str, name: &str) {
    let feature = root.join(layout).join(name);
    write(
        &feature.join(format!("src/capabilities_{name}_analyzer.rs")),
        "pub struct Analyzer;\n",
    );
    write(
        &feature.join(format!("src/agent_{name}_orchestrator.rs")),
        "pub struct Orchestrator;\n",
    );
    write(&feature.join("FRD.md"), "# FRD\n");
    write(&feature.join("BACKLOG.md"), "# BACKLOG\n");
    for prefix in [
        "contract_",
        "unit_",
        "integration_",
        "dogfood_",
        "smoke_",
        "e2e_",
        "acceptance_",
    ] {
        write(
            &feature.join(format!("tests/{prefix}{name}.rs")),
            "#[test]\nfn t() {}\n",
        );
    }
    write(
        &feature.join(format!("benches/bench_{name}.rs")),
        "fn b() {}\n",
    );
}

#[test]
fn e2e_a_conforming_workspace_produces_no_findings_end_to_end() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_conforming_feature(root, "crates", "calc");
    write(
        &root.join("crates/shared/src/taxonomy_domain_vo.rs"),
        "pub struct Domain;\n",
    );
    write(&root.join("crates/shared/DATA.md"), "# DATA\n");
    write(&root.join("crates/shared/BACKLOG.md"), "# BACKLOG\n");

    assert_eq!(
        audit(root),
        Vec::new(),
        "every folder obeying its shape contract must produce no finding"
    );
}

#[test]
fn e2e_each_missing_test_category_is_named_and_located() {
    // One case per category: dropping it must produce a finding that names the
    // prefix and points at the folder that owes it.
    for missing in [
        "contract_",
        "unit_",
        "integration_",
        "dogfood_",
        "smoke_",
        "e2e_",
        "acceptance_",
    ] {
        let tmp = tempfile::tempdir().expect("temp dir must be creatable");
        let root = tmp.path();
        write_conforming_feature(root, "crates", "calc");
        std::fs::remove_file(root.join(format!("crates/calc/tests/{missing}calc.rs")))
            .expect("the file under test must exist before it is removed");

        let findings = audit(root);
        let hits = matching(&findings, "AES704", "test_suite_missing_category");
        assert_eq!(
            hits.len(),
            1,
            "dropping {missing} must produce exactly one category finding; got {findings:?}"
        );
        assert!(
            hits[0].3.contains(&format!("'{missing}'")),
            "the finding must name the absent prefix {missing}; got {:?}",
            hits[0].3
        );
        assert_eq!(
            hits[0].2, "crates/calc/tests/",
            "the finding must locate the folder that owes the category; got {:?}",
            hits[0].2
        );
    }
}

#[test]
fn e2e_a_missing_bench_directory_is_reported_once_not_per_category() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_conforming_feature(root, "crates", "calc");
    std::fs::remove_dir_all(root.join("crates/calc/benches")).expect("benches must exist first");

    let findings = audit(root);
    assert_eq!(
        matching(&findings, "AES704", "test_suite_missing_bench_dir").len(),
        1,
        "an absent benches/ is one finding about the directory, not one per \
         category; got {findings:?}"
    );
    assert!(
        matching(&findings, "AES704", "test_suite_missing_category").is_empty(),
        "the seven tests/ categories are all present, so no category may be \
         reported; got {findings:?}"
    );
}

#[test]
fn e2e_a_support_prefix_does_not_satisfy_a_required_category() {
    // `regression_` is a legal name in tests/ (TEST.md §2.0) but is not one of
    // the seven categories, so a folder carrying only regression guards still
    // owes the full set. This is the distinction between AES103 (is the name
    // legal) and AES704 (is the category present).
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_conforming_feature(root, "crates", "calc");
    let tests = root.join("crates/calc/tests");
    for entry in std::fs::read_dir(&tests).expect("tests must exist") {
        let path = entry.expect("directory entry must be readable").path();
        std::fs::remove_file(&path).expect("fixture file must be removable");
    }
    write(&tests.join("regression_guard.rs"), "#[test]\nfn t() {}\n");

    let findings = audit(root);
    assert_eq!(
        matching(&findings, "AES704", "test_suite_missing_category").len(),
        7,
        "a regression guard satisfies no category; all seven must still be owed. \
         Got {findings:?}"
    );
}

#[test]
fn e2e_a_folder_owning_no_feature_source_owes_no_test_suite() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    // A utility-only folder: no capabilities, no orchestrator, so it is not a
    // feature and the AES704 gate — the same one AES702 uses — never opens.
    write(
        &root.join("crates/helpers/src/utility_path_normalizer.rs"),
        "pub fn norm() {}\n",
    );

    let findings = audit(root);
    assert!(
        matching(&findings, "AES704", "test_suite_missing_tests_dir").is_empty(),
        "a folder holding no feature source must not be told to write a test \
         suite; got {findings:?}"
    );
}
