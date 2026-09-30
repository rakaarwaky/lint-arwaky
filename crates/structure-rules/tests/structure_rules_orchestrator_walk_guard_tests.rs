// PURPOSE: guard tests for the AES702 orchestrator walk (issue #349) —
// symlink cycles and pathologically deep layouts must terminate.
#![cfg(not(windows))]

use shared::structure_rules::taxonomy_structure_rules_request::StructureRequest;
use shared::structure_rules::taxonomy_structure_rules_response::StructureResponse;
use structure_rules_lint_arwaky::root_structure_rules_container::RootStructureRulesContainer;

use std::fs;
#[cfg(unix)]
use std::os::unix::fs::symlink;
use std::path::Path;

/// Write *contents* as a file at *path*, creating parent dirs.
fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

/// Audit *root* through the public aggregate and collect findings.
fn audit(root: &Path) -> Vec<(String, String, String)> {
    let aggregate = RootStructureRulesContainer::orchestrator();
    let response = aggregate.execute(StructureRequest::audit_all(root));
    match response {
        StructureResponse::Findings { findings } => findings
            .into_iter()
            .map(|f| (f.code, f.violation_type, f.file))
            .collect(),
    }
}

/// A feature folder whose *src* tree contains a symlink pointing back at the
/// folder itself. The walk must terminate without following the link.
#[test]
fn orchestrator_walk_terminates_on_a_symlink_cycle() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let feature = root.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    write(&feature.join("FRD.md"), "# FRD");
    write(&feature.join("BACKLOG.md"), "# BACKLOG");
    write(
        &feature.join("src/agent_sample_orchestrator.rs"),
        "//! orchestrator",
    );
    // Loop back: feature/src/self -> feature
    symlink(feature.join("src"), feature.join("src/self")).unwrap();

    let findings = audit(root);
    // The orchestrator is directly in src/, so the reverse doc-pair check is
    // satisfied even with the cyclic link present.
    assert!(
        !findings
            .iter()
            .any(|(c, v, _)| c == "AES702" && v == "doc_pair_without_orchestrator"),
        "cyclic symlink must not hang the audit; got: {findings:#?}"
    );
}

/// A symlink cycle nested *below* the orchestrator must not shadow the file
/// nor hang the walk: the orchestrator still counts at the shallow level.
#[test]
fn orchestrator_walk_ignores_a_looping_symlink_nested_deeper() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let feature = root.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    write(&feature.join("FRD.md"), "# FRD");
    write(&feature.join("BACKLOG.md"), "# BACKLOG");
    // No orchestrator at the feature root; only inside the looped subtree —
    // which the walk must not reach.
    fs::create_dir_all(feature.join("src/inner")).unwrap();
    // Loop: inner/self -> src (ancestor), creating a back-edge.
    symlink(feature.join("src"), feature.join("src/inner/self")).unwrap();

    let findings = audit(root);
    assert!(
        findings
            .iter()
            .any(|(c, v, _)| c == "AES702" && v == "doc_pair_without_orchestrator"),
        "the doc pair with no reachable orchestrator must still fire; got: {findings:#?}"
    );
}

/// A feature folder nested far deeper than the walk cap must terminate.
#[test]
fn orchestrator_walk_terminates_on_a_pathologically_deep_layout() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let feature = root.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    write(&feature.join("FRD.md"), "# FRD");
    write(&feature.join("BACKLOG.md"), "# BACKLOG");
    // Nest 40 real directories deep — far past the cap of 10.
    let mut current = feature.join("src");
    for i in 0..40 {
        current = current.join(format!("level_{i:02}"));
    }
    fs::create_dir_all(&current).unwrap();
    write(
        &current.join("agent_sample_orchestrator.rs"),
        "//! orchestrator",
    );

    let findings = audit(root);
    // Past the cap the orchestrator is invisible, so the reverse doc-pair
    // check must fire rather than the audit hanging.
    assert!(
        findings
            .iter()
            .any(|(c, v, _)| c == "AES702" && v == "doc_pair_without_orchestrator"),
        "an orchestrator beyond the walk cap must not be found; got: {findings:#?}"
    );
}
