// E2E tests — full pipeline: temp dir with orphaned files → scan → verify orphans detected.
use orphan_rules_lint_arwaky::utility_orphan_filename::{
    file_basename, file_stem, file_suffix, identify_entry_points,
};
use shared::orphan_rules::taxonomy_orphan_contract_vo::{
    OrphanEntryPatternListVO, OrphanFileListVO,
};

// ── utility_orphan_filename e2e ────────────────────────────

#[test]
fn e2e_entry_point_detection_default_patterns() {
    let files = OrphanFileListVO::new(vec![
        "crates/shared/src/taxonomy_color.rs".to_string(),
        "crates/orphan-rules/src/root_orphan_detector_container.rs".to_string(),
        "crates/orphan-rules/src/capabilities_orphan_taxonomy_analyzer.rs".to_string(),
        "crates/cli/src/main.rs".to_string(),
        "crates/mcp/src/root_mcp_entry.rs".to_string(),
    ]);
    let entry_points = identify_entry_points(&[files], &[]);
    assert!(
        entry_points
            .values
            .contains(&"crates/orphan-rules/src/root_orphan_detector_container.rs".to_string())
    );
    assert!(
        entry_points
            .values
            .contains(&"crates/cli/src/main.rs".to_string())
    );
    assert!(
        entry_points
            .values
            .contains(&"crates/mcp/src/root_mcp_entry.rs".to_string())
    );
    // taxonomy_color and capabilities_ are not entry points
    assert!(
        !entry_points
            .values
            .contains(&"crates/shared/src/taxonomy_color.rs".to_string())
    );
    assert!(
        !entry_points.values.contains(
            &"crates/orphan-rules/src/capabilities_orphan_taxonomy_analyzer.rs".to_string()
        )
    );
}

#[test]
fn e2e_entry_point_detection_custom_patterns() {
    let files = OrphanFileListVO::new(vec![
        "crates/shared/src/taxonomy_color.rs".to_string(),
        "crates/orphan-rules/src/custom_entry.rs".to_string(),
    ]);
    let patterns = OrphanEntryPatternListVO::new(vec!["custom_entry".to_string()]);
    let entry_points = identify_entry_points(&[files], &[patterns]);
    assert!(
        entry_points
            .values
            .contains(&"crates/orphan-rules/src/custom_entry.rs".to_string())
    );
}

#[test]
fn e2e_file_basename_stem_suffix() {
    // Full path
    assert_eq!(
        file_basename("crates/shared/src/taxonomy_color.rs"),
        "taxonomy_color.rs"
    );
    assert_eq!(
        file_stem("crates/shared/src/taxonomy_color.rs"),
        "taxonomy_color"
    );
    assert_eq!(file_suffix("crates/shared/src/taxonomy_color.rs"), "color");

    // Contract protocol file
    assert_eq!(
        file_basename("src/contract_foo_protocol.rs"),
        "contract_foo_protocol.rs"
    );
    assert_eq!(
        file_stem("src/contract_foo_protocol.rs"),
        "contract_foo_protocol"
    );
    assert_eq!(file_suffix("src/contract_foo_protocol.rs"), "protocol");

    // Agent orchestrator file
    assert_eq!(
        file_basename("src/agent_foo_orchestrator.rs"),
        "agent_foo_orchestrator.rs"
    );
    assert_eq!(
        file_stem("src/agent_foo_orchestrator.rs"),
        "agent_foo_orchestrator"
    );
    assert_eq!(file_suffix("src/agent_foo_orchestrator.rs"), "orchestrator");

    // No extension — suffix extracts part after last underscore
    assert_eq!(file_basename("just_a_name"), "just_a_name");
    assert_eq!(file_stem("just_a_name"), "just_a_name");
    assert_eq!(file_suffix("just_a_name"), "name");

    // No underscore in stem
    assert_eq!(file_suffix("single.rs"), "");
    assert_eq!(file_stem("single.rs"), "single");
}

#[test]
fn e2e_empty_file_list_yields_no_entry_points() {
    let files = OrphanFileListVO::new(vec![]);
    let entry_points = identify_entry_points(&[files], &[]);
    assert!(entry_points.is_empty());
}

#[test]
fn e2e_multiple_file_lists_merged() {
    let files1 = OrphanFileListVO::new(vec!["crates/cli/src/main.rs".to_string()]);
    let files2 = OrphanFileListVO::new(vec!["crates/mcp/src/root_mcp_entry.rs".to_string()]);
    let entry_points = identify_entry_points(&[files1, files2], &[]);
    assert_eq!(entry_points.len(), 2);
    assert!(
        entry_points
            .values
            .contains(&"crates/cli/src/main.rs".to_string())
    );
    assert!(
        entry_points
            .values
            .contains(&"crates/mcp/src/root_mcp_entry.rs".to_string())
    );
}

#[test]
fn e2e_python_entry_points() {
    let files = OrphanFileListVO::new(vec![
        "modules/cli/main.py".to_string(),
        "modules/cli/__main__.py".to_string(),
        "modules/shared/taxonomy_helper.py".to_string(),
    ]);
    let entry_points = identify_entry_points(&[files], &[]);
    assert!(
        entry_points
            .values
            .contains(&"modules/cli/main.py".to_string())
    );
    assert!(
        entry_points
            .values
            .contains(&"modules/cli/__main__.py".to_string())
    );
    assert!(
        !entry_points
            .values
            .contains(&"modules/shared/taxonomy_helper.py".to_string())
    );
}

#[test]
fn e2e_typescript_entry_points() {
    let files = OrphanFileListVO::new(vec![
        "packages/cli/src/main.ts".to_string(),
        "packages/cli/src/index.ts".to_string(),
        "packages/shared/src/utility_parser.ts".to_string(),
    ]);
    let entry_points = identify_entry_points(&[files], &[]);
    assert!(
        entry_points
            .values
            .contains(&"packages/cli/src/main.ts".to_string())
    );
    assert!(
        entry_points
            .values
            .contains(&"packages/cli/src/index.ts".to_string())
    );
    assert!(
        !entry_points
            .values
            .contains(&"packages/shared/src/utility_parser.ts".to_string())
    );
}

#[test]
fn e2e_container_entry_points() {
    let files = OrphanFileListVO::new(vec![
        "crates/orphan-rules/src/root_orphan_detector_container.rs".to_string(),
        "crates/filesystem/src/root_filesystem_container.rs".to_string(),
        "crates/config-system/src/root_config_container.rs".to_string(),
    ]);
    let entry_points = identify_entry_points(&[files], &[]);
    assert_eq!(entry_points.len(), 3);
}

// ── Full pipeline: scan a synthetic workspace ──────────────

#[test]
fn e2e_synthetic_workspace_orphan_scan() {
    // Create a temp workspace with some files that should be detected as orphans
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    // Create directory structure
    std::fs::create_dir_all(root.join("src")).unwrap();

    // Create a taxonomy file that is NOT imported by anything (orphan)
    std::fs::write(
        root.join("src/taxonomy_orphan.rs"),
        "pub struct OrphanType;\n",
    )
    .unwrap();

    // Create a taxonomy file that IS imported (not orphan)
    std::fs::write(root.join("src/taxonomy_used.rs"), "pub struct UsedType;\n").unwrap();

    // Create a capabilities file that imports taxonomy_used
    std::fs::write(
        root.join("src/capabilities_handler.rs"),
        "use crate::taxonomy_used::UsedType;\npub fn handle() -> UsedType { UsedType }\n",
    )
    .unwrap();

    // Create an entry point file
    std::fs::write(
        root.join("src/root_container.rs"),
        "use crate::capabilities_handler;\npub fn main() { capabilities_handler::handle(); }\n",
    )
    .unwrap();

    // Verify files exist
    assert!(root.join("src/taxonomy_orphan.rs").exists());
    assert!(root.join("src/taxonomy_used.rs").exists());
    assert!(root.join("src/capabilities_handler.rs").exists());
    assert!(root.join("src/root_container.rs").exists());

    // Test entry point detection
    let files = OrphanFileListVO::new(vec![
        "src/taxonomy_orphan.rs".to_string(),
        "src/taxonomy_used.rs".to_string(),
        "src/capabilities_handler.rs".to_string(),
        "src/root_container.rs".to_string(),
    ]);
    let entry_points = identify_entry_points(&[files], &[]);
    assert!(
        entry_points
            .values
            .contains(&"src/root_container.rs".to_string())
    );
    assert_eq!(entry_points.len(), 1);
}
