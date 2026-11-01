// PURPOSE: structure-rules contract tests — AES702 forbidden files, shared-mirror
// and root-container checks plus AES705 member-root layout.
use shared_structure_rules::taxonomy_structure_rules_request::StructureRequest;
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;
use structure_rules_lint_arwaky::root_structure_rules_container::RootStructureRulesContainer;

use std::fs;
use std::path::Path;

/// Write *contents* as a file at *path*, creating parent dirs.
fn write(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, contents).unwrap();
}

/// Audit *root* through the public aggregate and collect (code, violation_type, file).
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

/// Does any finding carry this (code, violation_type) pair?
fn has(findings: &[(String, String, String)], code: &str, violation_type: &str) -> bool {
    findings
        .iter()
        .any(|(c, v, _)| c == code && v == violation_type)
}

/// Does any finding carry this (code, violation_type, file) triple?
fn has_at(
    findings: &[(String, String, String)],
    code: &str,
    violation_type: &str,
    file: &str,
) -> bool {
    findings
        .iter()
        .any(|(c, v, f)| c == code && v == violation_type && f == file)
}

/// The remedy half of each finding. AES705 names the destination folder in
/// its `fix` rather than in `message`, so a test about where a file belongs
/// reads this, not the message text.
fn fix_audit(root: &Path) -> Vec<String> {
    let aggregate = RootStructureRulesContainer::orchestrator();
    let response = aggregate.execute(StructureRequest::audit_all(root));
    match response {
        StructureResponse::Findings { findings } => findings.into_iter().map(|f| f.fix).collect(),
    }
}

// ─── AES702: feature folder forbidden files ──────────────────────────────────

#[test]
fn aes702_fires_when_a_feature_folder_holds_forbidden_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // A feature folder that also holds a taxonomy file — taxonomy belongs in
    // shared, not in a feature folder.
    write(
        root.join("crates/calculator/src/agent_calc_orchestrator.rs")
            .as_path(),
        "pub struct CalcOrchestrator;",
    );
    write(
        root.join("crates/calculator/src/capabilities_add_analyzer.rs")
            .as_path(),
        "pub struct AddAnalyzer;",
    );
    write(
        root.join("crates/calculator/src/taxonomy_domain_vo.rs")
            .as_path(),
        "pub struct Domain;",
    );
    let findings = audit(root);
    assert!(
        has(&findings, "AES702", "feature_has_forbidden_files"),
        "expected feature_has_forbidden_files, got: {findings:#?}"
    );
}

#[test]
fn aes702_stays_silent_when_only_capabilities_and_agents_are_present() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/calculator/src/agent_calc_orchestrator.rs")
            .as_path(),
        "pub struct CalcOrchestrator;",
    );
    write(
        root.join("crates/calculator/src/capabilities_add_analyzer.rs")
            .as_path(),
        "pub struct AddAnalyzer;",
    );
    let findings = audit(root);
    assert!(
        !has(&findings, "AES702", "feature_has_forbidden_files"),
        "a clean feature folder must not fire; got: {findings:#?}"
    );
}

/// AES702 shared mirror: a feature folder must have a mirror subfolder in
/// the member's shared/src/. Missing it fires feature_missing_shared_subfolder.
#[test]
fn aes702_fires_when_feature_lacks_shared_mirror() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // Feature folder under crates/
    write(
        root.join("crates/calc/src/agent_calc_orchestrator.rs")
            .as_path(),
        "pub struct CalcOrchestrator;",
    );
    write(
        root.join("crates/calc/src/capabilities_add_analyzer.rs")
            .as_path(),
        "pub struct AddAnalyzer;",
    );
    // No crates/shared/src/calc/ subfolder exists.
    let findings = audit(root);
    assert!(
        has(&findings, "AES702", "feature_missing_shared_subfolder"),
        "expected feature_missing_shared_subfolder, got: {findings:#?}"
    );
}

/// AES702 shared mirror stays silent when the shared/src/<feature>/ subfolder
/// exists for the feature.
#[test]
fn aes702_stays_silent_when_feature_has_shared_mirror() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/calc/src/agent_calc_orchestrator.rs")
            .as_path(),
        "pub struct CalcOrchestrator;",
    );
    write(
        root.join("crates/calc/src/capabilities_add_analyzer.rs")
            .as_path(),
        "pub struct AddAnalyzer;",
    );
    // Shared mirror exists (empty dir is enough for presence check)
    fs::create_dir_all(root.join("crates/shared/src/calc")).unwrap();
    let findings = audit(root);
    assert!(
        !has(&findings, "AES702", "feature_missing_shared_subfolder"),
        "a mirrored feature must not fire; got: {findings:#?}"
    );
}

/// AES702 shared mirror respects member scope: feature under modules/
/// mirrors into modules/shared/src/, not crates/shared/src/.
#[test]
fn aes702_shared_mirror_member_scoped() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // Feature under modules/
    write(
        root.join("modules/add/src/agent_add_orchestrator.rs")
            .as_path(),
        "pub struct AddOrchestrator;",
    );
    write(
        root.join("modules/add/src/capabilities_sum_analyzer.rs")
            .as_path(),
        "pub struct SumAnalyzer;",
    );
    // Only modules/shared/src/add/ exists; crates/shared/src/add/ does not.
    fs::create_dir_all(root.join("modules/shared/src/add")).unwrap();
    let findings = audit(root);
    assert!(
        !has(&findings, "AES702", "feature_missing_shared_subfolder"),
        "member-scoped mirror must pass; got: {findings:#?}"
    );
}

/// AES702 shared mirror does not fire for surface folders — they are not
/// feature folders and owe no shared mirror.
#[test]
fn aes702_shared_mirror_silent_for_surface_folder() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // Surface-dominated folder under crates/ (5 surface files, 0 capabilities/agent)
    write(
        root.join("crates/cli/surface_scan_command.rs").as_path(),
        "pub fn scan() {}",
    );
    write(
        root.join("crates/cli/surface_fix_command.rs").as_path(),
        "pub fn fix() {}",
    );
    write(
        root.join("crates/cli/surface_git_command.rs").as_path(),
        "pub fn git() {}",
    );
    write(
        root.join("crates/cli/surface_ci_command.rs").as_path(),
        "pub fn ci() {}",
    );
    write(
        root.join("crates/cli/surface_docs_command.rs").as_path(),
        "pub fn docs() {}",
    );
    // No shared/src/cli/ needed.
    let findings = audit(root);
    assert!(
        !has(&findings, "AES702", "feature_missing_shared_subfolder"),
        "surface folder must not fire feature_missing_shared_subfolder; got: {findings:#?}"
    );
}

/// AES702 root container: a feature folder must hold
/// `root_<module>_container.<ext>`. Missing it fires
/// feature_missing_root_container.
#[test]
fn aes702_fires_when_feature_lacks_root_container() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/calc/src/agent_calc_orchestrator.rs")
            .as_path(),
        "pub struct CalcOrchestrator;",
    );
    write(
        root.join("crates/calc/src/capabilities_add_analyzer.rs")
            .as_path(),
        "pub struct AddAnalyzer;",
    );
    let findings = audit(root);
    assert!(
        has_at(
            &findings,
            "AES702",
            "feature_missing_root_container",
            "crates/calc"
        ),
        "expected feature_missing_root_container at crates/calc, got: {findings:#?}"
    );
}

/// AES702 root container stays silent when `root_<module>_container.<ext>`
/// sits in the feature's `src/` subfolder.
#[test]
fn aes702_stays_silent_when_feature_has_root_container_in_src() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/calc/src/agent_calc_orchestrator.rs")
            .as_path(),
        "pub struct CalcOrchestrator;",
    );
    write(
        root.join("crates/calc/src/capabilities_add_analyzer.rs")
            .as_path(),
        "pub struct AddAnalyzer;",
    );
    write(
        root.join("crates/calc/src/root_calc_container.rs")
            .as_path(),
        "pub struct CalcContainer;",
    );
    let findings = audit(root);
    assert!(
        !has_at(
            &findings,
            "AES702",
            "feature_missing_root_container",
            "crates/calc"
        ),
        "a feature with a root container in src/ must not fire; got: {findings:#?}"
    );
}

/// AES702 root container stays silent when the container sits at the
/// feature folder root.
#[test]
fn aes702_stays_silent_when_feature_has_root_container_at_root() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/calc/src/agent_calc_orchestrator.rs")
            .as_path(),
        "pub struct CalcOrchestrator;",
    );
    write(
        root.join("crates/calc/src/capabilities_add_analyzer.rs")
            .as_path(),
        "pub struct AddAnalyzer;",
    );
    write(
        root.join("crates/calc/root_calc_container.rs").as_path(),
        "pub struct CalcContainer;",
    );
    let findings = audit(root);
    assert!(
        !has_at(
            &findings,
            "AES702",
            "feature_missing_root_container",
            "crates/calc"
        ),
        "a feature with a root container at the folder root must not fire; got: {findings:#?}"
    );
}

/// AES702 root container stays silent when the container sits at the
/// member root, beside the feature folder (the TS/Python member-root layout
/// workspaces-good uses).
#[test]
fn aes702_stays_silent_when_feature_has_root_container_at_member_root() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("modules/calc/src/agent_calc_orchestrator.py")
            .as_path(),
        "class CalcOrchestrator: pass\n",
    );
    write(
        root.join("modules/calc/src/capabilities_add_analyzer.py")
            .as_path(),
        "class AddAnalyzer: pass\n",
    );
    write(
        root.join("modules/root_calc_container.py").as_path(),
        "class CalcContainer: pass\n",
    );
    let findings = audit(root);
    assert!(
        !has_at(
            &findings,
            "AES702",
            "feature_missing_root_container",
            "modules/calc"
        ),
        "a feature with a root container at the member root must not fire; got: {findings:#?}"
    );
}

/// AES702 root container respects the language extension: a Python feature
/// satisfies the check with `root_<module>_container.py`, and the same
/// feature with no container fires.
#[test]
fn aes702_root_container_accepts_language_extension() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("modules/calc/src/agent_calc_orchestrator.py")
            .as_path(),
        "class CalcOrchestrator: pass\n",
    );
    write(
        root.join("modules/calc/src/capabilities_add_analyzer.py")
            .as_path(),
        "class AddAnalyzer: pass\n",
    );
    write(
        root.join("modules/calc/src/root_calc_container.py")
            .as_path(),
        "class CalcContainer: pass\n",
    );
    let findings = audit(root);
    assert!(
        !has_at(
            &findings,
            "AES702",
            "feature_missing_root_container",
            "modules/calc"
        ),
        "a Python feature with a .py root container must not fire; got: {findings:#?}"
    );
}

/// A Python feature with no root container fires the check, so the extension
/// probe discriminates rather than merely tolerating.
#[test]
fn aes702_root_container_fires_when_python_feature_lacks_container() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("modules/calc/src/agent_calc_orchestrator.py")
            .as_path(),
        "class CalcOrchestrator: pass\n",
    );
    write(
        root.join("modules/calc/src/capabilities_add_analyzer.py")
            .as_path(),
        "class AddAnalyzer: pass\n",
    );
    let findings = audit(root);
    assert!(
        has_at(
            &findings,
            "AES702",
            "feature_missing_root_container",
            "modules/calc"
        ),
        "expected feature_missing_root_container for a Python feature without a container; got: {findings:#?}"
    );
}

/// AES702 root container does not fire for surface folders — they are not
/// feature folders and owe no root container.
#[test]
fn aes702_root_container_silent_for_surface_folder() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/cli/surface_scan_command.rs").as_path(),
        "pub fn scan() {}",
    );
    write(
        root.join("crates/cli/surface_fix_command.rs").as_path(),
        "pub fn fix() {}",
    );
    write(
        root.join("crates/cli/surface_git_command.rs").as_path(),
        "pub fn git() {}",
    );
    write(
        root.join("crates/cli/surface_ci_command.rs").as_path(),
        "pub fn ci() {}",
    );
    write(
        root.join("crates/cli/surface_docs_command.rs").as_path(),
        "pub fn docs() {}",
    );
    let findings = audit(root);
    assert!(
        !has_at(
            &findings,
            "AES702",
            "feature_missing_root_container",
            "crates/cli"
        ),
        "surface folder must not fire feature_missing_root_container; got: {findings:#?}"
    );
}

#[test]
fn aes705_reports_a_capability_file_loose_at_a_member_root() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("modules/capabilities_add_analyzer.py").as_path(),
        "class AddAnalyzer: ...",
    );
    let findings = audit(root);
    assert!(
        has(&findings, "AES705", "member_root_has_layer_file"),
        "expected member_root_has_layer_file, got: {findings:#?}"
    );
}

#[test]
fn aes705_names_the_folder_kind_each_prefix_belongs_in() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/agent_calc_orchestrator.rs").as_path(),
        "pub struct O;",
    );
    write(
        root.join("crates/surface_cli_command.rs").as_path(),
        "pub struct C;",
    );
    write(
        root.join("crates/utility_path.rs").as_path(),
        "pub fn p() {}",
    );
    let findings = fix_audit(root);
    for expected in ["feature folder", "surface folder", "shared folder"] {
        assert!(
            findings.iter().any(|m| m.contains(expected)),
            "expected the finding to name '{expected}' as the destination, got: {findings:#?}"
        );
    }
}

#[test]
fn aes705_allows_wiring_and_unclassified_files_at_a_member_root() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(root.join("crates/lib.rs").as_path(), "pub mod x;");
    write(
        root.join("crates/root_entry_container.rs").as_path(),
        "pub struct C;",
    );
    write(root.join("crates/README.md").as_path(), "# Crates");
    let findings = audit(root);
    assert!(
        !has(&findings, "AES705", "member_root_has_layer_file"),
        "wiring and unclassified files are legal at a member root; got: {findings:#?}"
    );
}

#[test]
fn aes705_ignores_a_layer_file_inside_a_feature_folder() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/calculator/src/capabilities_add_analyzer.rs")
            .as_path(),
        "pub struct AddAnalyzer;",
    );
    let findings = audit(root);
    assert!(
        !has(&findings, "AES705", "member_root_has_layer_file"),
        "a file inside a feature folder is exactly where it belongs; got: {findings:#?}"
    );
}
