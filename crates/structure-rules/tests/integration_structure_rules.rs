// PURPOSE: integration tests for structure-rules — the real DI container over
// real workspaces on disk.
//
// The unit tests exercise one capability's helpers; these go through
// `RootStructureRulesContainer` so the wiring from root container to
// orchestrator to the four auditor seams is what is under test. A seam that was
// never wired shows up here as silence rather than as an error.
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

/// Audit *root* through the public container and collect (code, violation_type).
fn audit(root: &Path) -> Vec<(String, String)> {
    let aggregate = RootStructureRulesContainer::orchestrator();
    match aggregate.execute(StructureRequest::audit_all(root)) {
        StructureResponse::Findings { findings } => findings
            .into_iter()
            .map(|f| (f.code, f.violation_type))
            .collect(),
    }
}

/// Does any finding carry this (code, violation_type) pair?
fn has(findings: &[(String, String)], code: &str, violation_type: &str) -> bool {
    findings
        .iter()
        .any(|(c, v)| c == code && v == violation_type)
}

/// A complete feature folder: both layers plus its doc pair.
fn write_feature(root: &Path, layout: &str, name: &str) {
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
}

/// A feature folder whose test suite carries one file per AES704 category.
fn write_complete_test_suite(root: &Path, layout: &str, name: &str) {
    let feature = root.join(layout).join(name);
    let tests = feature.join("tests");
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
            &tests.join(format!("{prefix}{name}.rs")),
            "#[test]\nfn t() {}\n",
        );
    }
    write(
        &feature.join(format!("benches/bench_{name}.rs")),
        "fn b() {}\n",
    );
}

#[test]
fn container_orchestrator_answers_a_request_over_a_workspace() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_feature(root, "crates", "calc");
    write_complete_test_suite(root, "crates", "calc");

    assert_eq!(
        audit(root),
        Vec::new(),
        "a complete feature with a complete suite must pass the whole structure audit"
    );
}

#[test]
fn container_reaches_every_auditor_seam() {
    // One workspace carrying one defect per rule, so the codes seen prove the
    // container routed the request to AES701, AES702, AES703, and AES704 alike.
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();

    // AES701 — a shared kernel holding a capability file.
    write(
        &root.join("crates/shared/src/capabilities_stray_analyzer.rs"),
        "pub struct Stray;\n",
    );
    write(&root.join("crates/shared/DATA.md"), "# DATA\n");
    write(&root.join("crates/shared/BACKLOG.md"), "# BACKLOG\n");
    // AES702 — capabilities with no orchestrator.
    write(
        &root.join("crates/split/src/capabilities_split_analyzer.rs"),
        "pub struct Split;\n",
    );
    write(&root.join("crates/split/FRD.md"), "# FRD\n");
    write(&root.join("crates/split/BACKLOG.md"), "# BACKLOG\n");
    // AES703 — a surface-dominated folder with no DESIGN.md.
    write(
        &root.join("crates/surface/surface_scan_command.rs"),
        "pub fn scan() {}\n",
    );
    write(
        &root.join("crates/surface/surface_fix_command.rs"),
        "pub fn fix() {}\n",
    );
    write(&root.join("crates/surface/BACKLOG.md"), "# BACKLOG\n");
    // AES704 — a feature folder whose suite is present but missing `dogfood_`
    // and its benchmark, so the per-category finding is what fires.
    write_feature(root, "crates", "calc");
    let calc_tests = root.join("crates/calc/tests");
    for prefix in [
        "contract_",
        "unit_",
        "integration_",
        "smoke_",
        "e2e_",
        "acceptance_",
    ] {
        write(
            &calc_tests.join(format!("{prefix}calc.rs")),
            "#[test]\nfn t() {}\n",
        );
    }
    write(
        &root.join("crates/calc/benches/bench_calc.rs"),
        "fn b() {}\n",
    );

    let findings = audit(root);
    for (code, violation) in [
        ("AES701", "shared_has_forbidden_files"),
        ("AES702", "feature_missing_agent"),
        ("AES703", "surface_missing_design_md"),
        ("AES704", "test_suite_missing_category"),
    ] {
        assert!(
            has(&findings, code, violation),
            "the container must reach the {code} seam and report {violation}; got {findings:?}"
        );
    }
}

#[test]
fn container_resolves_up_to_the_workspace_root_from_a_member_directory() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_feature(root, "crates", "split");
    write(
        &root.join("crates/split/src/taxonomy_domain_vo.rs"),
        "pub struct Domain;\n",
    );

    // Scan the member dir itself, the way `scan workspaces-bad/crates` does.
    let findings = audit(&root.join("crates"));
    assert!(
        has(&findings, "AES702", "feature_has_forbidden_files"),
        "a member-dir scan must resolve up to the workspace root and still see the \
         feature folders beneath it; got {findings:?}"
    );
}

#[test]
fn container_stays_silent_on_a_workspace_whose_every_folder_is_complete() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();

    // A shared kernel holding only permitted layers, with its doc pair.
    write(
        &root.join("crates/shared/src/taxonomy_domain_vo.rs"),
        "pub struct Domain;\n",
    );
    write(&root.join("crates/shared/DATA.md"), "# DATA\n");
    write(&root.join("crates/shared/BACKLOG.md"), "# BACKLOG\n");

    // A feature folder with both layers, its doc pair, and all eight categories.
    write_feature(root, "crates", "calc");
    write_complete_test_suite(root, "crates", "calc");

    // A surface crate with its design docs and a kept utility.
    write(
        &root.join("crates/cli/surface_scan_command.rs"),
        "pub fn scan() {}\n",
    );
    write(
        &root.join("crates/cli/surface_fix_command.rs"),
        "pub fn fix() {}\n",
    );
    write(
        &root.join("crates/cli/utility_output_formatter.rs"),
        "pub fn fmt() {}\n",
    );
    write(&root.join("crates/cli/DESIGN.md"), "# DESIGN\n");
    write(&root.join("crates/cli/BACKLOG.md"), "# BACKLOG\n");

    assert_eq!(
        audit(root),
        Vec::new(),
        "a workspace where every folder obeys its shape contract must be clean"
    );
}
