// PURPOSE: acceptance test for FR-STRUCT-004 — the structure auditor is
// reachable through one aggregate entry point that routes to every folder rule.
//
// Maps 1:1 to the FRD requirement: a consumer holding only the container must be
// able to run the whole folder-layout audit over a workspace and receive a
// deduplicated, stable-sorted finding list, without naming a single capability.
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

/// Audit *root* through the public container and return the findings.
fn audit(root: &Path) -> Vec<(String, String, String)> {
    let aggregate = RootStructureRulesContainer::orchestrator();
    match aggregate.execute(StructureRequest::audit_all(root)) {
        StructureResponse::Findings { findings } => findings
            .into_iter()
            .map(|f| (f.code, f.violation_type, f.file))
            .collect(),
    }
}

/// A workspace carrying one defect per rule, so one request must surface one code
/// from each of the four folder rules the aggregate promises to route to.
fn write_faulty_workspace(root: &Path) {
    // AES701 — a shared kernel holding a capability file.
    write(
        &root.join("crates/shared/src/capabilities_stray_analyzer.rs"),
        "pub struct Stray;\n",
    );
    write(&root.join("crates/shared/DATA.md"), "# DATA\n");
    write(&root.join("crates/shared/BACKLOG.md"), "# BACKLOG\n");

    // AES702 — capabilities with no orchestrator, and no doc pair either.
    write(
        &root.join("crates/split/src/capabilities_split_analyzer.rs"),
        "pub struct Split;\n",
    );

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

    // AES704 — a feature folder with no test suite at all.
    write(
        &root.join("crates/calc/src/capabilities_calc_analyzer.rs"),
        "pub struct Calc;\n",
    );
    write(
        &root.join("crates/calc/src/agent_calc_orchestrator.rs"),
        "pub struct CalcOrchestrator;\n",
    );
    write(&root.join("crates/calc/FRD.md"), "# FRD\n");
    write(&root.join("crates/calc/BACKLOG.md"), "# BACKLOG\n");
}

#[test]
fn one_request_runs_every_folder_rule_and_reports_each_defect() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_faulty_workspace(root);

    let findings = audit(root);
    let codes: BTreeSet<&str> = findings.iter().map(|(code, _, _)| code.as_str()).collect();

    for expected in ["AES701", "AES702", "AES703", "AES704"] {
        assert!(
            codes.contains(expected),
            "a single request must reach the {expected} capability; the aggregate \
             promises to route every folder rule. Codes seen: {codes:?}"
        );
    }
}

#[test]
fn every_finding_names_the_folder_that_carries_the_defect() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write_faulty_workspace(root);

    for (code, violation, file) in audit(root) {
        assert!(
            !file.is_empty(),
            "{code}/{violation} must name the folder it belongs to; an unnamed \
             finding cannot be acted on"
        );
        assert!(
            file.starts_with("crates/"),
            "{code}/{violation} must carry a workspace-relative path so a \
             member-dir scan resolves it; got {file:?}"
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
        findings.iter().map(|(c, v, f)| (c, v, f)).collect();
    assert_eq!(
        distinct.len(),
        findings.len(),
        "one defect must produce one finding; a duplicated (code, violation, \
         folder) triple means the audit ran the same capability twice"
    );
}
