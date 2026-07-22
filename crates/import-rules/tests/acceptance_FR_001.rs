// PURPOSE: Acceptance test for FR-001: Layer Dependency Violation (AES201)
// Requirement: Lower layers must never import higher layers.

use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::collections::HashMap;
use std::io::Write;

fn fr001_config() -> ArchitectureConfig {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            forbidden: PatternList::new(vec![
                String::from("contract"),
                String::from("utility"),
                String::from("capabilities"),
                String::from("agent"),
                String::from("surfaces"),
                String::from("root"),
            ]),
            ..Default::default()
        },
    );
    layers.insert(
        LayerNameVO::new("contract"),
        LayerDefinition {
            forbidden: PatternList::new(vec![
                String::from("utility"),
                String::from("capabilities"),
                String::from("agent"),
                String::from("surfaces"),
                String::from("root"),
            ]),
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

/// FR-001: taxonomy_ must not import contract_, utility_, capabilities_, agent_, surface_, root_
#[tokio::test]
async fn fr001_taxonomy_importing_capabilities_emits_aes201() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_violation_vo.rs",
        "use crate::capabilities_checker::Checker;\npub struct V;\n",
    );

    let container = ImportContainer::new_with_config(fr001_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES201"),
        "FR-001: taxonomy importing capabilities must emit AES201"
    );
}

/// FR-001: contract_ must not import utility_, capabilities_, agent_, surface_, root_
#[tokio::test]
async fn fr001_contract_importing_agent_emits_aes201() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "contract_bad_protocol.rs",
        "use crate::agent_orchestrator::Orch;\npub trait Bad {}\n",
    );

    let container = ImportContainer::new_with_config(fr001_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES201"),
        "FR-001: contract importing agent must emit AES201"
    );
}

/// FR-001: Valid unidirectional import passes
#[tokio::test]
async fn fr001_valid_unidirectional_import_passes() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "taxonomy_clean_vo.rs", "pub struct Clean;\n");

    let container = ImportContainer::new_with_config(fr001_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(results.is_empty(), "FR-001: valid import must pass");
}

/// FR-001: AES201 diagnostic includes file path
#[tokio::test]
async fn fr001_diagnostic_includes_file_path() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_x_vo.rs",
        "use crate::capabilities_y::Y;\npub struct X;\n",
    );

    let container = ImportContainer::new_with_config(fr001_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    if let Some(v) = results.iter().find(|v| v.code.code() == "AES201") {
        assert!(
            v.file.value().contains("taxonomy_x_vo.rs"),
            "FR-001: diagnostic must include the violating file path"
        );
    } else {
        panic!("FR-001: expected AES201 violation");
    }
}

// ─── Multi-language: Python ────────────────────────────────

/// FR-001: Python taxonomy file importing from capabilities emits AES201
#[tokio::test]
async fn fr001_python_taxonomy_importing_capabilities_emits_aes201() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_vo.py",
        "from capabilities_checker import Checker\n\nclass V:\n    pass\n",
    );

    let container = ImportContainer::new_with_config(fr001_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES201"),
        "FR-001: Python taxonomy importing capabilities must emit AES201"
    );
}

// ─── Multi-language: JavaScript/TypeScript ─────────────────

/// FR-001: TypeScript taxonomy file importing from agent emits AES201
#[tokio::test]
async fn fr001_typescript_taxonomy_importing_agent_emits_aes201() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_vo.ts",
        "import { Orchestrator } from '../agent/orchestrator';\nexport interface V {}\n",
    );

    let container = ImportContainer::new_with_config(fr001_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES201"),
        "FR-001: TypeScript taxonomy importing agent must emit AES201"
    );
}

// ─── Negative: Conditional imports ─────────────────────────

/// FR-001: #[cfg(test)] import block should NOT emit AES201
#[tokio::test]
async fn fr001_cfg_test_import_not_flagged() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_cfg_vo.rs",
        "#[cfg(test)]\nuse crate::capabilities_test_helper::Helper;\npub struct V;\n",
    );

    let container = ImportContainer::new_with_config(fr001_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    // The #[cfg(test)] block should be skipped, so no AES201 from that import
    let cfg_violations: Vec<_> = results
        .iter()
        .filter(|v| v.code.code() == "AES201" && v.file.value().contains("taxonomy_cfg_vo.rs"))
        .collect();
    assert!(
        cfg_violations.is_empty(),
        "FR-001: #[cfg(test)] import should not emit AES201"
    );
}

/// FR-001: #[cfg(feature = "x")] import block should NOT emit AES201
#[tokio::test]
async fn fr001_cfg_feature_import_not_flagged() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_feature_vo.rs",
        "#[cfg(feature = \"testing\")]\nuse crate::capabilities_test_helper::Helper;\npub struct V;\n",
    );

    let container = ImportContainer::new_with_config(fr001_config());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let cfg_violations: Vec<_> = results
        .iter()
        .filter(|v| v.code.code() == "AES201" && v.file.value().contains("taxonomy_feature_vo.rs"))
        .collect();
    assert!(
        cfg_violations.is_empty(),
        "FR-001: #[cfg(feature)] import should not emit AES201"
    );
}

// ─── Negative: Exception handling ──────────────────────────

/// FR-001: File in exceptions list should skip AES201 check
#[tokio::test]
async fn fr001_excepted_file_skips_check() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_excepted_vo.rs",
        "use crate::capabilities_checker::Checker;\npub struct V;\n",
    );

    let mut config = fr001_config();
    config.rules.push(
        shared::config_system::taxonomy_config_vo::ArchitectureRule {
            name: shared::common::taxonomy_suggestion_vo::DescriptionVO::new("AES201".to_string()),
            exceptions: PatternList::new(vec!["taxonomy_excepted_vo.rs".to_string()]),
            ..Default::default()
        },
    );

    let container = ImportContainer::new_with_config(config);
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let excepted_violations: Vec<_> = results
        .iter()
        .filter(|v| v.code.code() == "AES201" && v.file.value().contains("taxonomy_excepted_vo.rs"))
        .collect();
    assert!(
        excepted_violations.is_empty(),
        "FR-001: excepted file should not produce AES201 violations"
    );
}
