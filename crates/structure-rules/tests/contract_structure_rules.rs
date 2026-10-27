// PURPOSE: structure-rules contract tests — AES701–AES703 must fire on a
// misplaced folder layout and stay silent on a conforming one.
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
/// Every finding message, for asserting on what a finding tells the author.
fn message_audit(root: &Path) -> Vec<String> {
    let aggregate = RootStructureRulesContainer::orchestrator();
    let response = aggregate.execute(StructureRequest::audit_all(root));
    match response {
        StructureResponse::Findings { findings } => {
            findings.into_iter().map(|f| f.message).collect()
        }
    }
}

/// The remedy half of each finding. AES705 names the destination folder in
/// its `fix` rather than in `message`, so a test about where a file belongs
/// reads this, not `message_audit`.
fn fix_audit(root: &Path) -> Vec<String> {
    let aggregate = RootStructureRulesContainer::orchestrator();
    let response = aggregate.execute(StructureRequest::audit_all(root));
    match response {
        StructureResponse::Findings { findings } => findings.into_iter().map(|f| f.fix).collect(),
    }
}

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

// ─── AES701: shared purity ─────────────────────────────────────────────────

#[test]
fn aes701_fires_when_shared_holds_a_capability_file() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/shared/src/taxonomy_domain_vo.rs")
            .as_path(),
        "pub struct DomainV;",
    );
    write(
        root.join("crates/shared/src/capabilities_db_adapter.rs")
            .as_path(),
        "pub struct DbAdapter;",
    );
    let findings = audit(root);
    assert!(
        has(&findings, "AES701", "shared_has_forbidden_files"),
        "expected shared_has_forbidden_files, got: {findings:#?}"
    );
}

#[test]
fn aes701_stays_silent_when_shared_holds_only_permitted_layers() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/shared/src/taxonomy_domain_vo.rs")
            .as_path(),
        "pub struct DomainV;",
    );
    write(
        root.join("crates/shared/src/utility_path_resolver.rs")
            .as_path(),
        "pub fn resolve() {}",
    );
    write(
        root.join("crates/shared/src/contract_scan_protocol.rs")
            .as_path(),
        "pub trait ScanProtocol {}",
    );
    fs::write(root.join("crates/shared/DATA.md"), "# DATA\n").unwrap();
    fs::write(root.join("crates/shared/BACKLOG.md"), "# BACKLOG\n").unwrap();
    let findings = audit(root);
    assert!(
        !has(&findings, "AES701", "shared_has_forbidden_files"),
        "shared holding only taxonomy/utility/contract must be clean; got: {findings:#?}"
    );
    assert!(
        !has(&findings, "AES701", "shared_missing_doc_pair"),
        "shared must have DATA.md + BACKLOG.md; got: {findings:#?}"
    );
}

#[test]
fn aes701_reports_each_forbidden_file_with_its_own_kind() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/shared/src/agent_scan_orchestrator.rs")
            .as_path(),
        "pub struct ScanOrchestrator;",
    );
    write(
        root.join("crates/shared/src/surface_scan_command.rs")
            .as_path(),
        "pub fn run() {}",
    );
    let findings = audit(root);
    let count = findings
        .iter()
        .filter(|(c, v, _)| c == "AES701" && v == "shared_has_forbidden_files")
        .count();
    assert_eq!(
        count, 2,
        "one finding per forbidden file; got: {findings:#?}"
    );
}

// ─── AES702: feature health ────────────────────────────────────────────────

#[test]
fn aes702_fires_when_capabilities_lack_an_orchestrator() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/calc/src/capabilities_add_analyzer.rs")
            .as_path(),
        "pub struct AddAnalyzer;",
    );
    let findings = audit(root);
    assert!(
        has(&findings, "AES702", "feature_missing_agent"),
        "expected feature_missing_agent, got: {findings:#?}"
    );
}

#[test]
fn aes702_fires_when_orchestrator_lacks_capabilities() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/calc/src/agent_calc_orchestrator.rs")
            .as_path(),
        "pub struct CalcOrchestrator;",
    );
    let findings = audit(root);
    assert!(
        has(&findings, "AES702", "feature_missing_capability"),
        "expected feature_missing_capability, got: {findings:#?}"
    );
}

#[test]
fn aes702_stays_silent_when_a_feature_has_both() {
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
        !has(&findings, "AES702", "feature_missing_agent")
            && !has(&findings, "AES702", "feature_missing_capability"),
        "a complete feature must be clean; got: {findings:#?}"
    );
}

#[test]
fn aes702_fires_inside_modules_and_packages_members() {
    for member in ["modules", "packages"] {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        write(
            &root.join(format!("{member}/calc/agent_calc_orchestrator.py")),
            "class CalcOrchestrator: pass\n",
        );
        let findings = audit(root);
        assert!(
            has(&findings, "AES702", "feature_missing_capability"),
            "AES702 must hold for {member}; got: {findings:#?}"
        );
    }
}

/// Scanning a member directory (`workspaces-bad/crates`) rather than the
/// workspace root must still reach the sibling members' feature folders —
/// the audit root is one level up, exactly as the other linters resolve it.
#[test]
fn aes702_fires_when_the_audit_root_is_a_member_directory() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/calc/src/agent_calc_orchestrator.rs")
            .as_path(),
        "pub struct CalcOrchestrator;",
    );
    // Scan the member dir itself, the way `scan workspaces-bad/crates` does.
    let findings = audit(&root.join("crates"));
    assert!(
        has(&findings, "AES702", "feature_missing_capability"),
        "a member-dir scan must resolve up to the workspace root; got: {findings:#?}"
    );
}

/// A member-level orchestrator does not answer for the feature folders beneath
/// it. Each feature owns its own orchestration, so a capabilities folder with no
/// agent of its own is still a split feature.
#[test]
fn aes702_fires_when_the_member_holds_the_orchestrator_instead() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("packages/agent_calculator_orchestrator.ts")
            .as_path(),
        "export class CalculatorOrchestrator {}",
    );
    write(
        root.join("packages/addition/src/capabilities_add_analyzer.ts")
            .as_path(),
        "export class AddAnalyzer {}",
    );
    let findings = audit(root);
    assert!(
        has(&findings, "AES702", "feature_missing_agent"),
        "each feature folder owes its own orchestrator; got: {findings:#?}"
    );
}

#[test]
fn aes702_stays_silent_when_a_feature_folder_owns_both_layers() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("packages/addition/src/agent_addition_orchestrator.ts")
            .as_path(),
        "export class AdditionOrchestrator {}",
    );
    write(
        root.join("packages/addition/src/capabilities_add_analyzer.ts")
            .as_path(),
        "export class AddAnalyzer {}",
    );
    let findings = audit(root);
    assert!(
        !has(&findings, "AES702", "feature_missing_agent"),
        "a feature folder owning both layers is complete; got: {findings:#?}"
    );
}

// ─── AES703: surface purity ────────────────────────────────────────────────

#[test]
fn aes703_fires_when_a_surface_folder_holds_a_capability() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // A surface-dominated crate: most files are surface, with one stray
    // capability that belongs in a feature folder.
    write(
        root.join("crates/cli/surface_scan_command.rs").as_path(),
        "pub fn scan() {}",
    );
    write(
        root.join("crates/cli/surface_fix_command.rs").as_path(),
        "pub fn fix() {}",
    );
    write(
        root.join("crates/cli/capabilities_fix_processor.rs")
            .as_path(),
        "pub struct FixProcessor;",
    );
    let findings = audit(root);
    assert!(
        has(&findings, "AES703", "surface_has_misplaced_files"),
        "expected surface_has_misplaced_files, got: {findings:#?}"
    );
}

#[test]
fn aes703_fires_when_surface_folder_holds_utility_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // 创建5个surface文件确保surface占主导
    write(
        root.join("crates/cli/surface_scan_command.rs").as_path(),
        "pub fn scan() {}",
    );
    write(
        root.join("crates/cli/surface_ci_command.rs").as_path(),
        "pub fn ci() {}",
    );
    write(
        root.join("crates/cli/surface_fix_command.rs").as_path(),
        "pub fn fix() {}",
    );
    write(
        root.join("crates/cli/surface_config_command.rs").as_path(),
        "pub fn config() {}",
    );
    write(
        root.join("crates/cli/surface_git_command.rs").as_path(),
        "pub fn git() {}",
    );
    // 添加utility文件（应触发违规）
    write(
        root.join("crates/cli/utility_output_text_formatter.rs")
            .as_path(),
        "pub fn fmt() {}",
    );
    // 添加lib.rs
    write(root.join("crates/cli/lib.rs").as_path(), "");
    fs::write(root.join("crates/cli/DESIGN.md"), "# CLI DESIGN\n").unwrap();
    fs::write(root.join("crates/cli/BACKLOG.md"), "# CLI BACKLOG\n").unwrap();
    let findings = audit(root);
    assert!(
        has(&findings, "AES703", "surface_has_misplaced_files"),
        "utility files inside a surface folder are forbidden; got: {findings:#?}"
    );
}

#[test]
fn aes703_stays_silent_when_a_surface_crate_holds_a_container() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/tui/surface_tui_command.rs").as_path(),
        "pub fn tui() {}",
    );
    write(
        root.join("crates/tui/root_tui_container.rs").as_path(),
        "pub struct TuiContainer;",
    );
    write(
        root.join("crates/tui/utility_file_system.rs").as_path(),
        "pub fn fs() {}",
    );
    fs::write(root.join("crates/tui/DESIGN.md"), "# TUI DESIGN\n").unwrap();
    fs::write(root.join("crates/tui/BACKLOG.md"), "# TUI BACKLOG\n").unwrap();
    let findings = audit(root);
    assert!(
        !has(&findings, "AES703", "surface_has_misplaced_files"),
        "root and utility files inside a surface crate are allowed; got: {findings:#?}"
    );
    assert!(
        !has(&findings, "AES703", "surface_missing_design_md"),
        "surface with DESIGN.md must be clean; got: {findings:#?}"
    );
    assert!(
        !has(&findings, "AES703", "surface_missing_backlog_md"),
        "surface with BACKLOG.md must be clean; got: {findings:#?}"
    );
}

// ─── combined workspace ────────────────────────────────────────────────────

#[test]
fn findings_are_deduplicated_and_ordered() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/shared/src/capabilities_stray.rs")
            .as_path(),
        "pub struct Stray;",
    );
    write(
        root.join("crates/calc/src/agent_calc_orchestrator.rs")
            .as_path(),
        "pub struct CalcOrchestrator;",
    );
    let first = audit(root);
    let second = audit(root);
    assert_eq!(first, second, "two runs must agree");
    // Findings arrive ordered by (file, code, violation_type) so a consumer
    // can stream them without re-sorting. `crates/calc` sorts before
    // `crates/shared`, so the AES702 row leads even though its code is higher.
    let mut expected = first.clone();
    expected.sort_by(|a, b| (&a.2, &a.0, &a.1).cmp(&(&b.2, &b.0, &b.1)));
    assert_eq!(first, expected, "findings must arrive in stable order");
}

/// A workspace shaped like the repo itself: shared holds only permitted
/// layers, feature crates carry both layers, and surface crates keep their
/// barrels and utilities. Nothing may fire.
#[test]
fn conforming_workspace_reports_no_findings() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // shared — permitted layers only, with required docs.
    write(
        root.join("crates/shared/src/taxonomy_domain_vo.rs")
            .as_path(),
        "pub struct DomainV;",
    );
    write(
        root.join("crates/shared/src/utility_path_resolver.rs")
            .as_path(),
        "pub fn resolve() {}",
    );
    write(
        root.join("crates/shared/src/contract_scan_protocol.rs")
            .as_path(),
        "pub trait ScanProtocol {}",
    );
    fs::write(root.join("crates/shared/DATA.md"), "# Shared DATA\n").unwrap();
    fs::write(root.join("crates/shared/BACKLOG.md"), "# Shared BACKLOG\n").unwrap();
    // a complete feature, documented, with a complete test suite.
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
        root.join("crates/calculator/FRD.md").as_path(),
        "# Calculator — FRD",
    );
    write(
        root.join("crates/calculator/BACKLOG.md").as_path(),
        "# Calculator — BACKLOG",
    );
    // the shared kernel mirror that a complete feature owes (AES702).
    write(
        root.join("crates/shared/src/calculator/contract_calculator_protocol.rs")
            .as_path(),
        "pub trait CalculatorProtocol {}",
    );
    // the root container that a complete feature owes (AES702).
    write(
        root.join("crates/calculator/src/root_calculator_container.rs")
            .as_path(),
        "pub struct CalculatorContainer;",
    );
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
            root.join(format!("crates/calculator/tests/{prefix}calculator.rs"))
                .as_path(),
            "#[test]\nfn t() {}",
        );
    }
    write(
        root.join("crates/calculator/benches/bench_calculator.rs")
            .as_path(),
        "fn b() {}",
    );
    // a surface crate keeping its barrels and helpers, documented.
    write(
        root.join("crates/cli/surface_scan_command.rs").as_path(),
        "pub fn scan() {}",
    );
    write(root.join("crates/cli/lib.rs").as_path(), "");
    write(
        root.join("crates/cli/DESIGN.md").as_path(),
        "# CLI — DESIGN",
    );
    write(
        root.join("crates/cli/BACKLOG.md").as_path(),
        "# CLI — BACKLOG",
    );
    write(
        root.join("crates/cli/utility_output_text_formatter.rs")
            .as_path(),
        "pub fn fmt() {}",
    );
    let findings = audit(root);
    assert!(findings.is_empty(), "expected clean, got: {findings:#?}");
}

// ─── AES702: feature folder doc pair ──────────────────────────────────────

#[test]
fn aes702_fires_when_a_feature_folder_lacks_its_doc_pair() {
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
    // no FRD.md, no BACKLOG.md.
    let findings = audit(root);
    assert!(
        has(&findings, "AES702", "feature_missing_doc_pair"),
        "expected the missing doc pair, got: {findings:#?}"
    );
}

#[test]
fn aes702_stays_silent_when_the_doc_pair_is_present() {
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
    write(
        root.join("crates/calculator/FRD.md").as_path(),
        "# Calculator — FRD",
    );
    write(
        root.join("crates/calculator/BACKLOG.md").as_path(),
        "# Calculator — BACKLOG",
    );
    let findings = audit(root);
    assert!(
        !has(&findings, "AES702", "feature_missing_doc_pair"),
        "a documented feature must not fire; got: {findings:#?}"
    );
}

#[test]
fn aes702_stays_silent_for_a_folder_carrying_no_feature_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // utility-only folder: neither capabilities nor agent, so no doc pair needed.
    write(
        root.join("crates/helpers/src/utility_path_normalizer.rs")
            .as_path(),
        "pub fn norm() {}",
    );
    let findings = audit(root);
    assert!(
        !has(&findings, "AES702", "feature_missing_doc_pair"),
        "a folder with no feature files is not a feature; got: {findings:#?}"
    );
}

// ─── AES703: surface folder DESIGN.md ──────────────────────────────────────

#[test]
fn aes703_fires_when_a_surface_folder_lacks_design_md() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/cli/surface_scan_command.rs").as_path(),
        "pub fn scan() {}",
    );
    // no DESIGN.md, no BACKLOG.md.
    let findings = audit(root);
    assert!(
        has(&findings, "AES703", "surface_missing_design_md"),
        "expected the missing DESIGN.md, got: {findings:#?}"
    );
}

#[test]
fn aes703_stays_silent_when_design_md_is_present() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(
        root.join("crates/cli/surface_scan_command.rs").as_path(),
        "pub fn scan() {}",
    );
    write(
        root.join("crates/cli/DESIGN.md").as_path(),
        "# CLI — DESIGN",
    );
    write(
        root.join("crates/cli/BACKLOG.md").as_path(),
        "# CLI — BACKLOG",
    );
    let findings = audit(root);
    assert!(
        !has(&findings, "AES703", "surface_missing_design_md"),
        "a documented surface must not fire; got: {findings:#?}"
    );
    assert!(
        !has(&findings, "AES703", "surface_missing_backlog_md"),
        "a documented surface must not fire; got: {findings:#?}"
    );
}

#[test]
fn aes703_does_not_fire_on_a_feature_folder_that_holds_a_surface() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // One surface among two capabilities: the folder is feature-dominated, not
    // a surface folder, so it owes FRD+BACKLOG rather than DESIGN.md.
    write(
        root.join("crates/calculator/src/surface_scan_command.rs")
            .as_path(),
        "pub fn scan() {}",
    );
    write(
        root.join("crates/calculator/src/capabilities_add_analyzer.rs")
            .as_path(),
        "pub struct AddAnalyzer;",
    );
    write(
        root.join("crates/calculator/src/capabilities_sub_checker.rs")
            .as_path(),
        "pub struct SubChecker;",
    );
    let findings = audit(root);
    assert!(
        !has(&findings, "AES703", "surface_missing_design_md"),
        "a feature-dominated folder is not a surface folder; got: {findings:#?}"
    );
}

// ─── AES702: feature folder health + docs ─────────────────────────────────

/// AES702 reverse direction: a feature folder (one that holds a capabilities
/// or agent file) that carries a doc pair but no orchestrator. A folder with
/// no AES-named source files at all is not a feature folder and is skipped.
#[test]
fn aes702_fires_when_a_doc_pair_folder_has_no_orchestrator() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let feature = root.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    fs::write(feature.join("FRD.md"), "# FRD\n").unwrap();
    fs::write(feature.join("BACKLOG.md"), "# BACKLOG\n").unwrap();
    // A capabilities file so the folder passes the feature-folder gate;
    // the orchestrator is deliberately omitted.
    fs::write(
        feature.join("src/capabilities_sample_analyzer.rs"),
        "//! capability\n",
    )
    .unwrap();
    let findings = audit(root);
    assert!(
        has(&findings, "AES702", "feature_missing_agent"),
        "a feature folder with capabilities but no orchestrator must fire; got: {findings:#?}"
    );
}

#[test]
fn aes702_silent_when_doc_pair_folder_has_orchestrator() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let feature = root.join("crates/sample");
    fs::create_dir_all(feature.join("src")).unwrap();
    fs::write(feature.join("FRD.md"), "# FRD\n").unwrap();
    fs::write(feature.join("BACKLOG.md"), "# BACKLOG\n").unwrap();
    fs::write(
        feature.join("src/agent_sample_orchestrator.rs"),
        "//! sample orchestrator\n",
    )
    .unwrap();
    let findings = audit(root);
    assert!(
        !has(&findings, "AES702", "doc_pair_without_orchestrator"),
        "should be silent when orchestrator is present; got: {findings:#?}"
    );
}

/// A non-agent surface/TS package folder (no capabilities_* or agent_*
/// files) that carries FRD.md + BACKLOG.md must not fire AES702.
/// The reverse doc-pair check is gated on the folder holding at least one
/// AES-named source file, so a surface-only member is skipped entirely.
#[test]
fn aes702_stays_silent_when_a_packages_member_has_no_agent_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let pkg = root.join("packages/auth");
    fs::create_dir_all(pkg.join("src")).unwrap();
    fs::write(&pkg.join("FRD.md"), "# FRD\n").unwrap();
    fs::write(&pkg.join("BACKLOG.md"), "# BACKLOG\n").unwrap();
    // TS source files — no AES-named (capabilities_*/agent_*) files.
    fs::write(pkg.join("src").join("index.ts"), "export const x = 1;\n").unwrap();
    let findings = audit(root);
    assert!(
        !has(&findings, "AES702", "doc_pair_without_orchestrator"),
        "a TS package folder with no agent files must not fire AES702; got: {findings:#?}"
    );
    assert!(
        !has(&findings, "AES702", "feature_missing_agent"),
        "a TS package folder with no agent files must not fire feature_missing_agent; got: {findings:#?}"
    );
}

/// A `modules/` surface folder (e.g. `modules/api`) with no agent files of
/// its own must not fire the AES702 reverse doc-pair check.
#[test]
fn aes702_stays_silent_when_a_modules_surface_has_no_agent_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let surface = root.join("modules/api");
    fs::create_dir_all(surface.join("src")).unwrap();
    fs::write(&surface.join("FRD.md"), "# FRD\n").unwrap();
    fs::write(&surface.join("BACKLOG.md"), "# BACKLOG\n").unwrap();
    fs::write(
        surface.join("src").join("surface_api_command.py"),
        "def run(): ...\n",
    )
    .unwrap();
    let findings = audit(root);
    assert!(
        !has(&findings, "AES702", "doc_pair_without_orchestrator"),
        "a surface module with no agent files must not fire AES702; got: {findings:#?}"
    );
}

#[test]
fn aes701_fires_when_shared_lacks_doc_pair() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    fs::create_dir_all(root.join("crates/shared/src")).unwrap();
    fs::write(
        root.join("crates/shared/src/taxonomy_domain_vo.rs"),
        "pub struct Domain;",
    )
    .unwrap();
    // No DATA.md or BACKLOG.md.
    let findings = audit(root);
    assert!(
        has(&findings, "AES701", "shared_missing_doc_pair"),
        "expected shared_missing_doc_pair, got: {findings:#?}"
    );
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

/// AES702 root container: a feature folder must hold a
/// `root_<module>_container.<ext>` at the folder root or in its `src/`
/// subfolder. Missing it fires feature_missing_root_container.
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
    // No root_calc_container.rs anywhere in the feature folder.
    let findings = audit(root);
    assert!(
        has(&findings, "AES702", "feature_missing_root_container"),
        "expected feature_missing_root_container, got: {findings:#?}"
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
        !has(&findings, "AES702", "feature_missing_root_container"),
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
        root.join("crates/calc/root_calc_container.rs")
            .as_path(),
        "pub struct CalcContainer;",
    );
    let findings = audit(root);
    assert!(
        !has(&findings, "AES702", "feature_missing_root_container"),
        "a feature with a root container at the folder root must not fire; got: {findings:#?}"
    );
}

/// AES702 root container respects the language extension: a Python feature
/// satisfies the check with `root_<module>_container.py`.
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
        !has(&findings, "AES702", "feature_missing_root_container"),
        "a Python feature with a .py root container must not fire; got: {findings:#?}"
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
        !has(&findings, "AES702", "feature_missing_root_container"),
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
