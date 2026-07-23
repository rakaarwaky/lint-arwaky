// PURPOSE: Acceptance test for FR-002: Mandatory Layer Imports (AES202)
// Requirement: Capability files must import their corresponding contract trait.

use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::collections::HashMap;
use std::io::Write;

fn fr002_config() -> ArchitectureConfig {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            mandatory: PatternList::new(vec![String::from("contract")]),
            ..Default::default()
        },
    );
    ArchitectureConfig {
        enabled: BooleanVO::new(true),
        layers,
        rules: Vec::new(),
        naming: shared::common::taxonomy_definition_vo::NamingConfig::new(Count::new(2)),
        ignored_paths: FilePathList { values: vec![] },
        mandatory_class_definition: BooleanVO::new(false),
    }
}

fn write_file(dir: &std::path::Path, name: &str, content: &str) {
    let mut file = std::fs::File::create(dir.join(name)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

/// FR-002: Capability file missing contract import emits AES202
#[tokio::test]
async fn fr002_capability_without_contract_emits_aes202() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "capabilities_no_contract.rs",
        "use std::collections::HashMap;\npub struct NoContract;\n",
    );

    let container = ImportContainer::new_with_config(fr002_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES202"),
        "FR-002: capability without contract import must emit AES202"
    );
}

/// FR-002: Capability file with contract import passes
#[tokio::test]
async fn fr002_capability_with_contract_passes() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "capabilities_with_contract.rs",
        r#"
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
pub struct WithContract;
"#,
    );

    let container = ImportContainer::new_with_config(fr002_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let aes202: Vec<_> = results
        .iter()
        .filter(|v| v.code.code() == "AES202")
        .collect();
    assert!(
        aes202.is_empty(),
        "FR-002: capability with contract import must pass"
    );
}

/// FR-002: AES202 diagnostic includes expected import name
#[tokio::test]
async fn fr002_diagnostic_includes_expected_import() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "capabilities_missing.rs",
        "pub struct Missing;\n",
    );

    let container = ImportContainer::new_with_config(fr002_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    if let Some(v) = results.iter().find(|v| v.code.code() == "AES202") {
        assert!(
            v.message.value().contains("contract"),
            "FR-002: diagnostic must mention the expected import 'contract'"
        );
    } else {
        panic!("FR-002: expected AES202 violation");
    }
}

// ─── Multi-language: Python ────────────────────────────────

/// FR-002: Python capability file missing contract import emits AES202
#[tokio::test]
async fn fr002_python_capability_without_contract_emits_aes202() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "capabilities_no_contract.py",
        "import os\n\nclass NoContract:\n    pass\n",
    );

    let container = ImportContainer::new_with_config(fr002_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES202"),
        "FR-002: Python capability without contract import must emit AES202"
    );
}

// ─── Multi-language: JavaScript/TypeScript ─────────────────

/// FR-002: TypeScript capability file missing contract import emits AES202
#[tokio::test]
async fn fr002_typescript_capability_without_contract_emits_aes202() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "capabilities_handler.ts",
        "import { readFileSync } from 'fs';\nexport class Handler {}\n",
    );

    let container = ImportContainer::new_with_config(fr002_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES202"),
        "FR-002: TypeScript capability without contract import must emit AES202"
    );
}

// ─── Negative: Exception handling ──────────────────────────

/// FR-002: File in exceptions list should skip AES202 check
#[tokio::test]
async fn fr002_excepted_file_skips_check() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "capabilities_excepted.rs",
        "pub struct Excepted;\n",
    );

    let mut config = fr002_config();
    config.rules.push(
        shared::config_system::taxonomy_config_vo::ArchitectureRule {
            name: shared::common::taxonomy_suggestion_vo::DescriptionVO::new("AES202".to_string()),
            exceptions: PatternList::new(vec!["capabilities_excepted.rs".to_string()]),
            ..Default::default()
        },
    );

    let container = ImportContainer::new_with_config(config);
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let excepted_violations: Vec<_> = results
        .iter()
        .filter(|v| {
            v.code.code() == "AES202" && v.file.value().contains("capabilities_excepted.rs")
        })
        .collect();
    assert!(
        excepted_violations.is_empty(),
        "FR-002: excepted file should not produce AES202 violations"
    );
}
