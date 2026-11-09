// PURPOSE: acceptance test for AES706 — PR-<segment> id to crate folder mapping.
//
// Every crate folder's FRD.md carries FR ids whose feature prefix matches the
// folder name, and every PR-<segment>-001..N id in PRD.md / ROADMAP.md maps 1:1
// to a crate folder by order (the project picks the segment: `LA`, `PSD`,
// `CALC`, …). A mismatch fires AES706 so the linter enforces the 1:1 contract
// between spec ids and crate folders.
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

/// The AES706 findings that answer the `PR-<segment>` id ↔ folder mapping,
/// excluding the FRD-side findings (`pr_missing_frd`, `pr_fr_id_mismatch`):
/// the mapping tests below plant folders without FRD.md on purpose.
fn mapping_findings(findings: &[(String, String, String)]) -> Vec<(String, String, String)> {
    findings
        .iter()
        .filter(|(code, violation, _)| {
            code == "AES706"
                && (violation == "pr_id_mismatch" || violation == "pr_folder_without_id")
        })
        .cloned()
        .collect()
}

/// A conforming workspace: one crate folder with a matching FRD and a PRD that
/// maps PR-LA-001 to `filesystem`.
fn write_conforming_workspace(root: &Path) {
    fs::create_dir_all(root.join("crates/filesystem")).unwrap();
    write(
        &root.join("crates/filesystem/src/agent_filesystem_orchestrator.rs"),
        "//! filesystem orchestrator\n",
    );
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
    fs::create_dir_all(root.join("crates/filesystem/src")).unwrap();
    write(
        &root.join("crates/filesystem/src/agent_filesystem_orchestrator.rs"),
        "//! filesystem orchestrator\n",
    );
    write(
        &root.join("crates/filesystem/FRD.md"),
        "# FRD — filesystem\n\n### FR-005B\n",
    );

    // A crate folder with no FRD at all.
    fs::create_dir_all(root.join("crates/auto-fix/src")).unwrap();
    write(
        &root.join("crates/auto-fix/src/agent_auto_fix_orchestrator.rs"),
        "//! auto-fix orchestrator\n",
    );
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
        types.contains("pr_fr_id_mismatch"),
        "FR-005B in crates/filesystem must fire pr_fr_id_mismatch; got {types:?}"
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
        types.contains("pr_missing_frd"),
        "a crate folder with no FRD.md must fire pr_missing_frd; got {types:?}"
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
        .filter(|(code, violation, _)| code == "AES706" && violation == "pr_id_mismatch")
        .map(|(_, _, file)| file.as_str())
        .collect();

    assert!(
        mismatch_targets.contains(&"PRD.md"),
        "PRD.md titled 'Multi-language scanning' while position 0 is `filesystem` must fire pr_id_mismatch; got {mismatch_targets:?}"
    );
    assert!(
        mismatch_targets.contains(&"ROADMAP.md"),
        "ROADMAP.md titled 'Multi-language scanning' while position 0 is `filesystem` must fire pr_id_mismatch; got {mismatch_targets:?}"
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
        .filter(|(code, violation, _)| code == "AES706" && violation == "pr_id_mismatch")
        .map(|(_, violation, file)| (violation.clone(), file.clone()))
        .collect();

    assert!(
        mismatches.is_empty(),
        "'### PR-LA-001: Filesystem scanning' names crates/filesystem, so no mismatch may fire; got {mismatches:?}"
    );
}

// ─── Sub-issue 1: the violation name carries no hardcoded segment ──────────

/// The project in psd-timelapse names its ids `PR-PSD-001`, not `PR-LA-001`,
/// and rejects a PHASE folder titled with a bytecode cache name. Whatever the
/// segment, the violation the linter reports must be `pr_id_mismatch` — a name
/// that does not bake lint-arwaky's own `LA` into another project's output.
#[test]
fn violation_names_carry_no_hardcoded_project_segment() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write(
        &root.join("crates/viewer/FRD.md"),
        "# FRD — viewer\n\n### FR-Viewer-001\n",
    );
    write(
        &root.join("crates/viewer/src/agent_viewer_orchestrator.rs"),
        "//! viewer orchestrator\n",
    );
    write(
        &root.join("PRD.md"),
        "# PRD\n\n### PR-PSD-001: renderer\n\n### PR-PSD-002: orphan\n",
    );
    write(&root.join("ROADMAP.md"), "# ROADMAP\n\n### PR-PSD-001: renderer\n");

    let aes706 = mapping_findings(&audit(root));

    assert!(
        aes706.iter().any(|(_, violation, _)| violation == "pr_id_mismatch"),
        "a PR-PSD id with no folder at its position must fire 'pr_id_mismatch'; got {aes706:#?}"
    );
    assert!(
        aes706.iter().all(|(_, violation, _)| !violation.contains("_la_")),
        "no AES706 violation name may hardcode the `la` segment; got {aes706:#?}"
    );
}

// ─── Sub-issue 2: phantom folders never take a position in the order ───────

/// A Python member dir holds `__pycache__` beside its real modules once the
/// project has been imported. `_` sorts before every letter, so counting the
/// cache as a crate folder pushes `PR-<segment>-001` onto `animator` and reports
/// every id against its predecessor's folder. The cache must take no position.
#[test]
fn cache_directory_does_not_shift_the_folder_order() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write(
        &root.join("modules/animator/agent_animator_orchestrator.py"),
        "# animator orchestrator\n",
    );
    write(
        &root.join("modules/animator/FRD.md"),
        "# FRD — animator\n\n### FR-Animator-001\n",
    );
    write(
        &root.join("modules/api/agent_api_orchestrator.py"),
        "# api orchestrator\n",
    );
    write(
        &root.join("modules/api/FRD.md"),
        "# FRD — api\n\n### FR-Api-001\n",
    );
    fs::create_dir_all(root.join("modules/__pycache__")).expect("cache dir must be creatable");
    fs::write(root.join("modules/__pycache__/animator.cpython-312.pyc"), "bytecode")
        .expect("cache file must be writable");
    write(
        &root.join("PRD.md"),
        "# PRD\n\n### PR-PSD-001: Animator pipeline\n\n### PR-PSD-002: Api surface\n",
    );
    write(
        &root.join("ROADMAP.md"),
        "# ROADMAP\n\n### PR-PSD-001: Animator pipeline\n\n### PR-PSD-002: Api surface\n",
    );

    let aes706 = mapping_findings(&audit(root));

    assert!(
        aes706.is_empty(),
        "modules/__pycache__ must not take crate position 0; got {aes706:#?}"
    );
}

/// The same shift through a build/dependency directory: `node_modules` in a
/// TypeScript member dir is not a package, and must not be reported as a
/// folder without a `PR-<segment>` id either.
#[test]
fn dependency_directory_is_not_reported_as_a_crate_folder() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    write(
        &root.join("packages/auth/surface_auth_command.ts"),
        "// auth command\n",
    );
    write(
        &root.join("packages/auth/DESIGN.md"),
        "# DESIGN — auth\n",
    );
    write(
        &root.join("packages/client/surface_client_command.ts"),
        "// client command\n",
    );
    write(
        &root.join("packages/client/DESIGN.md"),
        "# DESIGN — client\n",
    );
    fs::create_dir_all(root.join("packages/node_modules/left-pad"))
        .expect("dependency dir must be creatable");
    write(
        &root.join("PRD.md"),
        "# PRD\n\n### PR-TS-001: Auth surface\n\n### PR-TS-002: Client surface\n",
    );
    write(
        &root.join("ROADMAP.md"),
        "# ROADMAP\n\n### PR-TS-001: Auth surface\n\n### PR-TS-002: Client surface\n",
    );

    let findings = audit(root);
    let reporter: Vec<&(String, String, String)> = findings
        .iter()
        .filter(|(_, _, file)| file.contains("node_modules"))
        .collect();

    assert!(
        reporter.is_empty(),
        "packages/node_modules must never be reported as a crate folder; got {reporter:#?}"
    );
}

// ─── Sub-issue 3: one folder order per language ─────────────────────────────

/// A multi-language project documents one `PR-<segment>` sequence per language.
/// Each sequence must be audited against its own member dir's folder order, not
/// against one alphabetically flattened list of every language's folders.
#[test]
fn multi_language_document_scopes_each_sequence_to_its_own_member() {
    let tmp = tempfile::tempdir().expect("temp dir must be creatable");
    let root = tmp.path();
    // Python modules, sorted: animator, api, cli.
    for folder in ["animator", "api", "cli"] {
        write(
            &root.join(format!("modules/{folder}/agent_{folder}_orchestrator.py")),
            "# orchestrator\n",
        );
    }
    // TypeScript packages, sorted: auth, client, config.
    for folder in ["auth", "client", "config"] {
        write(
            &root.join(format!("packages/{folder}/surface_{folder}_command.ts")),
            "// command\n",
        );
        write(
            &root.join(format!("packages/{folder}/DESIGN.md")),
            "# DESIGN\n",
        );
    }
    write(
        &root.join("PRD.md"),
        "# PRD\n\n\
         ### PR-PY-001: Animator pipeline\n\n\
         ### PR-PY-002: Api surface\n\n\
         ### PR-PY-003: Cli surface\n\n\
         ### PR-TS-001: Auth surface\n\n\
         ### PR-TS-002: Client surface\n\n\
         ### PR-TS-003: Config surface\n",
    );
    write(
        &root.join("ROADMAP.md"),
        "# ROADMAP\n\n\
         ### PR-PY-001: Animator pipeline\n\n\
         ### PR-PY-002: Api surface\n\n\
         ### PR-PY-003: Cli surface\n\n\
         ### PR-TS-001: Auth surface\n\n\
         ### PR-TS-002: Client surface\n\n\
         ### PR-TS-003: Config surface\n",
    );

    let mismatches = mapping_findings(&audit(root));

    assert!(
        mismatches.is_empty(),
        "each language's PR sequence must match its own member's folder order; got {mismatches:#?}"
    );
}
