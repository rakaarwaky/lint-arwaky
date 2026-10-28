// PURPOSE: acceptance test for AES706 — PR-LA id to crate folder mapping.
//
// Every crate folder's FRD.md carries FR ids whose feature prefix matches the
// folder name, and every PR-LA-001..N id in PRD.md / ROADMAP.md maps 1:1 to a
// crate folder by order (PR-LA-001 → the first folder, `filesystem`). A
// mismatch fires AES706 so the linter enforces the 1:1 contract between spec
// ids and crate folders.
use shared_structure_rules::taxonomy_structure_rules_request::StructureRequest;
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;
use structure_rules_lint_arwaky::root_structure_rules_container::RootStructureRulesContainer;

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

/// Audit *root* through the public aggregate and return the findings.
fn audit(root: &Path) -> Vec<(String, String, String)> {
    let aggregate = RootStructureRulesContainer::orchestrator();
    match aggregate.execute(StructureRequest::audit_all(root)) {
        StructureResponse::Findings { findings } => findings
            .into_iter()
            .map(|f| (f.code, f.violation_type, f.file))
            .collect(),
    }
}

/// A conforming workspace: one crate folder with a matching FRD and a PRD that
/// maps PR-LA-001 to `filesystem`.
fn write_conforming_workspace(root: &Path) {
    fs::create_dir_all(root.join("crates/filesystem")).unwrap();
    write(
        &root.join("crates/filesystem/FRD.md"),
        "# FRD — filesystem\n\n## Functional Requirements\n\n### FR-Filesystem-001\n",
    );
    write(
        &root.join("PRD.md"),
        "# PRD\n\n### PR-LA-001: Filesystem scanning\n",
    );
    write(&root.join("ROADMAP.md"), "# ROADMAP\n\n### PR-LA-001\n");
}

/// A workspace with every AES706 defect at once.
fn write_faulty_workspace(root: &Path) {
    // PR-LA-001 maps to `filesystem` by order — but no crates/filesystem exists.
    write(
        &root.join("PRD.md"),
        "# PRD\n\n### PR-LA-001: Multi-language scanning\n",
    );
    write(&root.join("ROADMAP.md"), "# ROADMAP\n\n### PR-LA-001\n");

    // A crate folder whose FRD carries a wrong-prefix FR id.
    fs::create_dir_all(root.join("crates/filesystem")).unwrap();
    write(
        &root.join("crates/filesystem/FRD.md"),
        "# FRD — filesystem\n\n### FR-005B\n",
    );

    // A crate folder with no FRD at all.
    fs::create_dir_all(root.join("crates/auto-fix")).unwrap();
}

#[test]
fn conforming_workspace_reports_no_aes706_findings() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_conforming_workspace(root);

    let findings = audit(root);
    let codes: BTreeSet<&str> = findings.iter().map(|(code, _, _)| code.as_str()).collect();

    assert!(
        !codes.contains("AES706"),
        "a conforming workspace must report zero AES706 findings; got {codes:?}"
    );
}

#[test]
fn pr_la_id_without_folder_fires_aes706() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_faulty_workspace(root);

    let findings = audit(root);
    let codes: BTreeSet<&str> = findings.iter().map(|(code, _, _)| code.as_str()).collect();

    assert!(
        codes.contains("AES706"),
        "PR-LA-001 with no crates/filesystem must fire AES706; got {codes:?}"
    );
}

#[test]
fn fr_id_wrong_prefix_fires_aes706() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_faulty_workspace(root);

    let findings = audit(root);
    let types: BTreeSet<&str> = findings.iter().map(|(_, v, _)| v.as_str()).collect();

    assert!(
        types.contains("pr_la_fr_id_mismatch"),
        "FR-005B in crates/filesystem must fire pr_la_fr_id_mismatch; got {types:?}"
    );
}

#[test]
fn missing_frd_fires_aes706() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_faulty_workspace(root);

    let findings = audit(root);
    let types: BTreeSet<&str> = findings.iter().map(|(_, v, _)| v.as_str()).collect();

    assert!(
        types.contains("pr_la_missing_frd"),
        "a crate folder with no FRD.md must fire pr_la_missing_frd; got {types:?}"
    );
}

#[test]
fn pr_la_id_position_maps_to_existing_folder() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_conforming_workspace(root);

    let findings = audit(root);
    let codes: BTreeSet<&str> = findings.iter().map(|(code, _, _)| code.as_str()).collect();

    assert!(
        !codes.contains("AES706"),
        "PR-LA-001 mapping to an existing crates/filesystem must be clean; got {codes:?}"
    );
}

/// Folders are all correct, but the spec titles are not: `crates/filesystem`
/// exists at position 0 while both PRD.md and ROADMAP.md title PR-LA-001 with
/// "Multi-language scanning", which names no folder.
fn write_wrong_title_workspace(root: &Path) {
    fs::create_dir_all(root.join("crates/filesystem")).unwrap();
    write(
        &root.join("crates/filesystem/FRD.md"),
        "# FRD — filesystem\n\n## Functional Requirements\n\n### FR-Filesystem-001\n",
    );
    write(
        &root.join("PRD.md"),
        "# PRD\n\n### PR-LA-001: Multi-language scanning (Rust, Python, JS/TS)\n",
    );
    write(
        &root.join("ROADMAP.md"),
        "# ROADMAP\n\n### PR-LA-001: Multi-language scanning (Rust, Python, JS/TS)\n",
    );
}

#[test]
fn pr_la_title_not_naming_folder_fires_in_prd_and_roadmap() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_wrong_title_workspace(root);

    let findings = audit(root);
    let mismatch_targets: Vec<&str> = findings
        .iter()
        .filter(|(code, violation, _)| code == "AES706" && violation == "pr_la_id_mismatch")
        .map(|(_, _, file)| file.as_str())
        .collect();

    assert!(
        mismatch_targets.contains(&"PRD.md"),
        "PRD.md titled 'Multi-language scanning' while position 0 is `filesystem` must fire pr_la_id_mismatch; got {mismatch_targets:?}"
    );
    assert!(
        mismatch_targets.contains(&"ROADMAP.md"),
        "ROADMAP.md titled 'Multi-language scanning' while position 0 is `filesystem` must fire pr_la_id_mismatch; got {mismatch_targets:?}"
    );
}

#[test]
fn title_naming_its_folder_reports_no_mismatch() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_conforming_workspace(root);

    let findings = audit(root);
    let mismatches: Vec<(String, String)> = findings
        .iter()
        .filter(|(code, violation, _)| code == "AES706" && violation == "pr_la_id_mismatch")
        .map(|(_, violation, file)| (violation.clone(), file.clone()))
        .collect();

    assert!(
        mismatches.is_empty(),
        "'### PR-LA-001: Filesystem scanning' names crates/filesystem, so no mismatch may fire; got {mismatches:?}"
    );
}
