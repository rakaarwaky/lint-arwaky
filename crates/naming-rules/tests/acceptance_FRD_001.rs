// PURPOSE: Acceptance tests for FR-001 — AES101 Naming Convention Consistency
// Maps 1:1 to FRD business rules

use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use std::collections::HashMap;
use std::sync::Arc;

fn build_container() -> NamingContainer {
    let mut layers = HashMap::new();
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

async fn scan_file(filename: &str) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(filename), "").unwrap();

    let container = build_container();
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    orch.run_audit(&target).await.unwrap()
}

// ─── FR-001: Valid snake_case name passes ─────────────────

/// FR-001: Every file stem must be snake_case with at least 3 words.
#[tokio::test]
async fn fr001_valid_snake_case_passes() {
    let results = scan_file("capabilities_user_checker.rs").await;
    let aes101: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES101")
        .collect();
    assert!(
        aes101.is_empty(),
        "Valid snake_case should not trigger AES101"
    );
}

// ─── FR-001: Non-snake_case fails ─────────────────────────

/// FR-001: Must be snake_case (lowercase ASCII + underscores)
#[tokio::test]
async fn fr001_camel_case_fails() {
    let results = scan_file("capabilities_userChecker.rs").await;
    let aes101: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES101")
        .collect();
    assert!(!aes101.is_empty(), "camelCase should trigger AES101");
}

/// FR-001: Must be snake_case (lowercase ASCII + underscores)
#[tokio::test]
async fn fr001_uppercase_fails() {
    let results = scan_file("capabilities_User_Checker.rs").await;
    let aes101: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES101")
        .collect();
    assert!(!aes101.is_empty(), "Uppercase should trigger AES101");
}

// ─── FR-001: Minimum 3 words ──────────────────────────────

/// FR-001: Minimum 3 words (prefix + concept + suffix)
#[tokio::test]
async fn fr001_two_words_fails() {
    let results = scan_file("capabilities_checker.rs").await;
    let aes101: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES101")
        .collect();
    assert!(!aes101.is_empty(), "Two words should trigger AES101");
}

/// FR-001: Minimum 3 words (prefix + concept + suffix)
#[tokio::test]
async fn fr001_three_words_passes() {
    let results = scan_file("capabilities_user_checker.rs").await;
    let aes101: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES101")
        .collect();
    assert!(aes101.is_empty(), "Three words should pass AES101");
}

// ─── FR-001: Exception files pass ─────────────────────────

/// FR-001: Exceptions: main.rs, lib.rs, mod.rs, __init__.py, index.ts, index.js
#[tokio::test]
async fn fr001_exception_main_rs_passes() {
    let results = scan_file("main.rs").await;
    assert!(results.is_empty(), "main.rs is an exception");
}

/// FR-001: Exceptions: main.rs, lib.rs, mod.rs, __init__.py, index.ts, index.js
#[tokio::test]
async fn fr001_exception_lib_rs_passes() {
    let results = scan_file("lib.rs").await;
    assert!(results.is_empty(), "lib.rs is an exception");
}

/// FR-001: Exceptions: main.rs, lib.rs, mod.rs, __init__.py, index.ts, index.js
#[tokio::test]
async fn fr001_exception_mod_rs_passes() {
    let results = scan_file("mod.rs").await;
    assert!(results.is_empty(), "mod.rs is an exception");
}

/// FR-001: Exceptions: main.rs, lib.rs, mod.rs, __init__.py, index.ts, index.js
#[tokio::test]
async fn fr001_exception_init_py_passes() {
    let results = scan_file("__init__.py").await;
    assert!(results.is_empty(), "__init__.py is an exception");
}

/// FR-001: Exceptions: main.rs, lib.rs, mod.rs, __init__.py, index.ts, index.js
#[tokio::test]
async fn fr001_exception_index_ts_passes() {
    let results = scan_file("index.ts").await;
    assert!(results.is_empty(), "index.ts is an exception");
}

// ─── FR-001: Error handling emits AES101 ──────────────────

/// FR-001: Emit AES101 with invalid filename
#[tokio::test]
async fn fr001_emits_aes101_code() {
    let results = scan_file("capabilities_Bad.rs").await;
    assert!(!results.is_empty());
    assert_eq!(results[0].code.code(), "AES101");
}
