// PURPOSE: guard tests for the AES702 orchestrator walk (issue #349) —
// symlink cycles and pathologically deep layouts must terminate.
//
// Each test puts the feature folder's only orchestrator *inside* the guarded
// subtree — a looping symlink or 40 levels down — so the guarded walk, which
// never follows symlinks and stops at `MAX_ORCHESTRATOR_WALK_DEPTH` (10),
// cannot reach it. `feature_missing_agent` and `doc_pair_without_orchestrator`
// must both fire: the capabilities file passes the feature-folder gate, but
// the orchestrator is invisible to the walk. A broken guard (an unbounded
// walk) would either hang on the loop or descend past the cap and find the
// orchestrator, so neither AES702 finding would fire and the assertion
// would fail instead of the audit hanging.
#![cfg(not(windows))]

use shared_structure_rules::taxonomy_structure_rules_request::StructureRequest;
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;
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
    let StructureResponse::Findings { findings } = response;
    findings
        .into_iter()
        .map(|f| (f.code, f.violation_type, f.file))
        .collect()
}

/// Does any finding carry this (code, violation_type) pair?
fn has(findings: &[(String, String, String)], code: &str, violation_type: &str) -> bool {
    findings
        .iter()
        .any(|(c, v, _)| c == code && v == violation_type)
}

/// A feature folder with FRD + BACKLOG + a capabilities file at the feature
/// root (passes the feature-folder gate) and the folder's only orchestrator
/// placed *beyond* the walk's reach.
fn feature_with_unreachable_orchestrator(root: &Path, place_orchestrator: impl FnOnce(&Path)) {
    let feature = root.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    write(&feature.join("FRD.md"), "# FRD");
    write(&feature.join("BACKLOG.md"), "# BACKLOG");
    // A capabilities file so the folder passes the feature-folder gate.
    write(
        &feature.join("src/capabilities_sample_analyzer.rs"),
        "//! capability",
    );
    place_orchestrator(&feature);
}

/// The AES702 reverse doc-pair check must fire because the folder's only
/// orchestrator is invisible to the guarded walk; the forward health check
/// (`feature_missing_agent`) fires for the same reason.
fn assert_orchestrator_invisible(findings: &[(String, String, String)]) {
    assert!(
        has(findings, "AES702", "doc_pair_without_orchestrator"),
        "the folder's only orchestrator is unreachable by the guarded walk, so \
         doc_pair_without_orchestrator must fire; got: {findings:#?}"
    );
    assert!(
        has(findings, "AES702", "feature_missing_agent"),
        "the folder's only orchestrator is unreachable by the guarded walk, so \
         feature_missing_agent must fire; got: {findings:#?}"
    );
}

/// A feature folder whose *src* tree carries a symlink back-edge to an
/// ancestor, with the orchestrator nested just past the walk cap (depth 12)
/// so the guarded walk — which stops at depth 10 and never follows symlinks
/// — cannot reach it. A loop sits in the tree so an unbounded walk would
/// also hang, not merely miss the orchestrator.
#[test]
fn orchestrator_walk_terminates_on_a_symlink_cycle() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    feature_with_unreachable_orchestrator(root, |feature| {
        let inner = feature.join("src/inner");
        fs::create_dir_all(&inner).unwrap();
        // Back-edge: inner/self -> src (ancestor), creating a loop the
        // guarded walk never follows.
        symlink(feature.join("src"), inner.join("self")).unwrap();
        // The orchestrator sits 12 levels below src/ — past the cap of 10 —
        // and its only path through the tree is the loop.
        let mut path = feature.join("src/inner");
        for i in 0..12 {
            path = path.join(format!("level_{i:02}"));
        }
        fs::create_dir_all(&path).unwrap();
        write(
            &path.join("agent_sample_orchestrator.rs"),
            "//! orchestrator",
        );
    });

    let findings = audit(root);
    assert_orchestrator_invisible(&findings);
}

/// A symlink cycle nested *below* the only orchestrator's container:
/// `src/deep/self` loops back to `src/`, and the orchestrator sits 12
/// levels below `deep/` — past the cap, and only reachable through the loop.
/// The walk must terminate without following the link, leaving the
/// orchestrator invisible.
#[test]
fn orchestrator_walk_ignores_a_looping_symlink_nested_deeper() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    feature_with_unreachable_orchestrator(root, |feature| {
        let deep = feature.join("src/deep");
        fs::create_dir_all(&deep).unwrap();
        // Loop: deep/self -> src (ancestor), creating a back-edge.
        symlink(feature.join("src"), deep.join("self")).unwrap();
        // No orchestrator at the shallow level; the folder's only
        // orchestrator sits 12 levels below deep/, past the cap of 10, and
        // its only path through the tree is the loop.
        let mut path = deep.join("inner");
        for i in 0..12 {
            path = path.join(format!("level_{i:02}"));
        }
        fs::create_dir_all(&path).unwrap();
        write(
            &path.join("agent_sample_orchestrator.rs"),
            "//! orchestrator",
        );
    });

    let findings = audit(root);
    assert_orchestrator_invisible(&findings);
}

/// A feature folder nested far deeper than the walk cap must terminate. The
/// folder's only orchestrator sits 40 levels down — well past
/// `MAX_ORCHESTRATOR_WALK_DEPTH` (10) — so the guarded walk stops and cannot
/// see it.
#[test]
fn orchestrator_walk_terminates_on_a_pathologically_deep_layout() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    feature_with_unreachable_orchestrator(root, |feature| {
        // Nest the folder's only orchestrator 40 real directories deep — far
        // past the cap of 10 — so the walk stops and never reaches it.
        let mut current = feature.join("src");
        for i in 0..40 {
            current = current.join(format!("level_{i:02}"));
        }
        fs::create_dir_all(&current).unwrap();
        write(
            &current.join("agent_sample_orchestrator.rs"),
            "//! orchestrator",
        );
    });

    let findings = audit(root);
    assert_orchestrator_invisible(&findings);
}
