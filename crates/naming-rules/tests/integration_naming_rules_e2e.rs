// PURPOSE: E2E integration test — full naming-rules pipeline on mixed valid/invalid files
//
// Tests the complete flow: container → orchestrator → checkers → results
// Uses a real tempdir with mixed valid and invalid file names

use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::sync::Arc;

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Build a naming container with flexible layers (same as acceptance_FRD_001).
fn build_container() -> NamingContainer {
    let mut layers = std::collections::HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());
    layers.insert(LayerNameVO::new("taxonomy"), LayerDefinition::default());
    layers.insert(LayerNameVO::new("utility"), LayerDefinition::default());
    layers.insert(LayerNameVO::new("agent"), LayerDefinition::default());

    let config = ArchitectureConfig {
        layers: layers.clone(),
        ..Default::default()
    };
    let layer_map = LayerMapVO::new(layers);
    NamingContainer::new(Arc::new(config), Arc::new(layer_map))
}

/// Create a tempdir and write empty files with the given filenames.
fn setup_test_files(filenames: &[&str]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for name in filenames {
        std::fs::write(dir.path().join(*name), "").unwrap();
    }
    dir
}

// ─── Test: Mixed valid and invalid files produce correct violations ──────────

/// E2E: Full pipeline on a directory with mixed files — validates naming convention
/// checks work end-to-end via the orchestrator.
#[tokio::test]
async fn e2e_mixed_files_produces_correct_violations() {
    let dir = setup_test_files(&[
        // Valid file (should produce NO AES101 violations)
        "capabilities_user_checker.rs",
        // Invalid files (should produce AES101 violations)
        "badfile.rs",            // No layer prefix, too few words → AES101
        "Capabilities_Check.rs", // Uppercase → AES101
    ]);

    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    // badfile + Capabilities_Check should fail AES101 (naming convention)
    let aes101: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES101")
        .collect();
    assert!(
        aes101.len() >= 2,
        "Expected at least 2 AES101 violations for badfile.rs and Capabilities_Check.rs, got {}",
        aes101.len()
    );

    // Verify valid file is NOT in results
    let valid_in_results = results
        .iter()
        .any(|r| r.file.value.contains("capabilities_user_checker"));
    assert!(
        !valid_in_results,
        "Valid file 'capabilities_user_checker.rs' should not appear in violations"
    );
}

// ─── Test: Barrel and entry point files are always skipped ────────────────────

/// E2E: A directory with only barrel/entry files should produce zero violations.
#[tokio::test]
async fn e2e_barrel_files_always_skip() {
    let dir = setup_test_files(&[
        "mod.rs",
        "__init__.py",
        "index.ts",
        "index.js",
        "main.rs",
        "lib.rs",
    ]);

    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.is_empty(),
        "Barrel and entry point files should produce zero violations, got {}",
        results.len()
    );
}

// ─── Test: Non-source extensions are filtered out ─────────────────────────────

/// E2E: A directory with non-source files (e.g., .txt, .json) should produce
/// zero violations because they're filtered before checking.
#[tokio::test]
async fn e2e_non_source_extensions_filtered() {
    let dir = setup_test_files(&["bad_file_name.txt", "also_bad.json", "config.yaml"]);

    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.is_empty(),
        "Non-source files should be filtered out and produce zero violations, got {}",
        results.len()
    );
}

// ─── Test: All valid layer prefixes are recognized ────────────────────────────

/// E2E: A directory with one valid file per layer prefix should produce zero AES101.
#[tokio::test]
async fn e2e_all_valid_prefixes_recognized() {
    let dir = setup_test_files(&[
        "taxonomy_user_vo.rs",
        "contract_data_protocol.rs",
        "utility_json_parser.rs",
        "capabilities_user_checker.rs",
        "agent_pipeline_orchestrator.rs",
    ]);

    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let aes101: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES101")
        .collect();
    assert!(
        aes101.is_empty(),
        "All valid layer prefixes should produce zero AES101 violations, got {}",
        aes101.len()
    );
}

// ─── Test: Orchestrator name returns correct value ────────────────────────────

/// E2E: Verify the orchestrator's name() method returns "naming-rules".
#[tokio::test]
async fn e2e_orchestrator_returns_name() {
    let container = build_container();
    let orch = container.orchestrator();
    assert_eq!(orch.name(), "naming-rules");
}
