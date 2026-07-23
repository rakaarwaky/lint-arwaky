// PURPOSE: Acceptance test for FR-003: Unused Import Detection (AES203)
// Requirement: Imported symbols never referenced in file body are flagged.

use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::io::Write;

fn write_file(dir: &std::path::Path, name: &str, content: &str) {
    let mut file = std::fs::File::create(dir.join(name)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

/// FR-003: Unused import detected with AES203
#[tokio::test]
async fn fr003_unused_import_emits_aes203() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_unused_vo.rs",
        "use std::collections::HashMap;\n\npub struct Unused;\n",
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES203"),
        "FR-003: unused import must emit AES203"
    );
}

/// FR-003: Used import does not emit AES203
#[tokio::test]
async fn fr003_used_import_passes() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_used_vo.rs",
        r#"
use std::collections::HashMap;

pub struct Used {
    pub data: HashMap<String, i32>,
}
"#,
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        !results.iter().any(|v| v.code.code() == "AES203"),
        "FR-003: used import must not emit AES203"
    );
}

/// FR-003: AES203 diagnostic includes unused symbol name
#[tokio::test]
async fn fr003_diagnostic_includes_symbol_name() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_sym_vo.rs",
        "use std::collections::BTreeMap;\n\npub struct Sym;\n",
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    if let Some(v) = results.iter().find(|v| v.code.code() == "AES203") {
        assert!(
            v.message.value().contains("BTreeMap"),
            "FR-003: diagnostic must include the unused symbol name"
        );
    } else {
        panic!("FR-003: expected AES203 violation");
    }
}

// ─── Multi-language: Python ────────────────────────────────

/// FR-003: Python unused import detected with AES203
#[tokio::test]
async fn fr003_python_unused_import_emits_aes203() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_unused.py",
        "import os\nimport sys\n\ndef main():\n    print(os.getcwd())\n",
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES203"),
        "FR-003: Python unused import must emit AES203"
    );
}

/// FR-003: Python used import does not emit AES203
#[tokio::test]
async fn fr003_python_used_import_passes() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_used.py",
        "import os\n\ndef main():\n    print(os.getcwd())\n",
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let aes203: Vec<_> = results
        .iter()
        .filter(|v| v.code.code() == "AES203" && v.file.value().contains("taxonomy_used.py"))
        .collect();
    assert!(
        aes203.is_empty(),
        "FR-003: Python used import must not emit AES203"
    );
}

// ─── Multi-language: JavaScript/TypeScript ─────────────────

/// FR-003: TypeScript unused import detected with AES203
#[tokio::test]
async fn fr003_typescript_unused_import_emits_aes203() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_unused.ts",
        "import { readFileSync } from 'fs';\nimport { join } from 'path';\n\nexport const getData = () => readFileSync('x');\n",
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES203"),
        "FR-003: TypeScript unused import must emit AES203"
    );
}

// ─── Negative: Conditional imports ─────────────────────────

/// FR-003: #[cfg(test)] import should NOT be flagged as unused
#[tokio::test]
async fn fr003_cfg_test_import_not_flagged() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_cfg.rs",
        "#[cfg(test)]\nuse std::collections::HashMap;\n\npub struct V;\n",
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let cfg_violations: Vec<_> = results
        .iter()
        .filter(|v| v.code.code() == "AES203" && v.file.value().contains("taxonomy_cfg.rs"))
        .collect();
    assert!(
        cfg_violations.is_empty(),
        "FR-003: #[cfg(test)] import should not be flagged as unused"
    );
}

/// FR-003: #[cfg(feature = "x")] import should NOT be flagged as unused
#[tokio::test]
async fn fr003_cfg_feature_import_not_flagged() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_feature.rs",
        "#[cfg(feature = \"testing\")]\nuse std::collections::HashMap;\n\npub struct V;\n",
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let cfg_violations: Vec<_> = results
        .iter()
        .filter(|v| v.code.code() == "AES203" && v.file.value().contains("taxonomy_feature.rs"))
        .collect();
    assert!(
        cfg_violations.is_empty(),
        "FR-003: #[cfg(feature)] import should not be flagged as unused"
    );
}

// ─── Negative: Wildcard imports ────────────────────────────

/// FR-003: Wildcard import is not tracked as unused (current behavior)
/// Wildcard imports (`use foo::*`) are excluded from alias tracking because
/// we cannot determine which symbols from the wildcard are actually used.
#[tokio::test]
async fn fr003_wildcard_import_not_tracked() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "taxonomy_wildcard.rs",
        "use std::collections::*;\n\npub struct V;\n",
    );

    let container = ImportContainer::new_with_config(ArchitectureConfig::default());
    let orch = container.orchestrator();
    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    // Wildcard imports are excluded from unused detection — no AES203 expected
    let aes203: Vec<_> = results
        .iter()
        .filter(|v| v.code.code() == "AES203" && v.file.value().contains("taxonomy_wildcard.rs"))
        .collect();
    assert!(
        aes203.is_empty(),
        "FR-003: wildcard import should not be tracked as unused (current behavior)"
    );
}
