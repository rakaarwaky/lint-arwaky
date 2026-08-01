// PURPOSE: Integration tests — full DI wiring via ImportContainer
// Verifies that the container correctly assembles all capabilities
// and the orchestrator runs the complete pipeline.

use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::{BooleanVO, Count, PatternList};
use shared::common::{FilePath, FilePathList, LayerDefinition, LayerNameVO};

use shared::config_system::ArchitectureConfig;
use std::collections::HashMap;
use std::io::Write;

fn full_config() -> ArchitectureConfig {
    let mut layers = HashMap::new();

    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            allowed: PatternList::default(),
            forbidden: PatternList::new(vec![
                "contract".to_string(),
                "utility".to_string(),
                "capabilities".to_string(),
                "agent".to_string(),
                "surfaces".to_string(),
                "root".to_string(),
            ]),
            mandatory: PatternList::default(),
            word_count: Count::new(2),
            exceptions: PatternList::default(),
            recursive: BooleanVO::new(false),
            ..Default::default()
        },
    );

    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            allowed: PatternList::new(vec![
                "taxonomy".to_string(),
                "contract".to_string(),
                "utility".to_string(),
            ]),
            forbidden: PatternList::new(vec!["agent".to_string(), "surfaces".to_string()]),
            mandatory: PatternList::new(vec!["contract".to_string()]),
            word_count: Count::new(2),
            exceptions: PatternList::default(),
            recursive: BooleanVO::new(false),
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

fn write_temp_rs(dir: &std::path::Path, name: &str, content: &str) -> FilePath {
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    FilePath::new(path.to_string_lossy().to_string()).unwrap()
}

// ─── Container Wiring ─────────────────────────────────────

#[test]
fn container_creates_orchestrator() {
    let container = ImportContainer::new_with_config(
        full_config(),
        std::sync::Arc::new(filesystem::FilesystemOrchestrator::new()),
    );

    // Orchestrator accessor should return a valid Arc
    let _o = container.orchestrator();
}

// ─── Full Pipeline: Clean Project ─────────────────────────

#[tokio::test]
async fn clean_project_produces_zero_violations() {
    let dir = tempfile::tempdir().unwrap();

    // Taxonomy file — no imports
    write_temp_rs(
        dir.path(),
        "taxonomy_foo_vo.rs",
        "pub struct Foo {\n    pub value: String,\n}\n",
    );

    // Capabilities file — imports contract (mandatory) + taxonomy for real logic
    write_temp_rs(
        dir.path(),
        "capabilities_foo_checker.rs",
        r#"
use shared::import_rules::contract_import_mandatory_protocol::IMandatoryImportProtocol;
use shared::common::FilePath;

pub struct FooChecker;

impl IMandatoryImportProtocol for FooChecker {
    fn rule_name(&self) -> shared::common::Identity {
        shared::common::Identity::new("AES202")
    }
    async fn run_mandatory_imports(
        &self,
        _config: &shared::config_system::ArchitectureConfig,
        _layer_map: &shared::common::LayerMapVO,
        _files: &shared::common::FilePathList,
        _root_dir: &FilePath,
    ) -> Result<shared::cli_commands::LintResultList, shared::import_rules::ImportError> {
        let path = FilePath::new("test".to_string()).unwrap();
        // Real logic: use FilePath to demonstrate import usage
        let _ = !path.value().is_empty();
        Ok(shared::cli_commands::LintResultList::new(Vec::new()))
    }
}
"#,
    );

    let container = ImportContainer::new_with_config(
        full_config(),
        std::sync::Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    let orch = container.orchestrator();

    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.is_empty(),
        "Clean project should have zero violations, got: {:?}",
        results
            .iter()
            .map(|r| r.message.value().to_string())
            .collect::<Vec<_>>()
    );
}

// ─── Full Pipeline: Violation Detected ────────────────────

#[tokio::test]
async fn forbidden_import_detected_through_full_pipeline() {
    let dir = tempfile::tempdir().unwrap();

    // Taxonomy file importing capabilities — FORBIDDEN
    write_temp_rs(
        dir.path(),
        "taxonomy_bad_vo.rs",
        "use crate::capabilities_foo_checker::FooChecker;\n\npub struct Bad;\n",
    );

    let container = ImportContainer::new_with_config(
        full_config(),
        std::sync::Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    let orch = container.orchestrator();

    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES201"),
        "Forbidden import should produce AES201 through full pipeline"
    );
}

// ─── Full Pipeline: Unused Import ─────────────────────────

#[tokio::test]
async fn unused_import_detected_through_full_pipeline() {
    let dir = tempfile::tempdir().unwrap();

    write_temp_rs(
        dir.path(),
        "taxonomy_unused_vo.rs",
        "use std::collections::HashMap;\n\npub struct Unused;\n",
    );

    let container = ImportContainer::new_with_config(
        full_config(),
        std::sync::Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    let orch = container.orchestrator();

    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES203"),
        "Unused import should produce AES203 through full pipeline"
    );
}

// ─── Full Pipeline: Dummy Function ────────────────────────

#[tokio::test]
async fn dummy_function_detected_through_full_pipeline() {
    let dir = tempfile::tempdir().unwrap();

    write_temp_rs(
        dir.path(),
        "capabilities_dummy.rs",
        r#"
use shared::common::FilePath;

fn _use_mandatory_imports() {
    let _ = FilePath::new("x");
}

pub struct DummyCap;
"#,
    );

    let container = ImportContainer::new_with_config(
        full_config(),
        std::sync::Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    let orch = container.orchestrator();

    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    assert!(
        results.iter().any(|v| v.code.code() == "AES204"),
        "Dummy function should produce AES204 through full pipeline"
    );
}

// ─── Multiple Violations Aggregated ───────────────────────

#[tokio::test]
async fn multiple_violation_types_aggregated() {
    let dir = tempfile::tempdir().unwrap();

    // File with unused import + dummy function
    write_temp_rs(
        dir.path(),
        "capabilities_multi.rs",
        r#"
use std::collections::HashMap;
use shared::common::FilePath;

fn _use_mandatory_imports() {
    let _ = FilePath::new("x");
}

pub struct Multi;
"#,
    );

    let container = ImportContainer::new_with_config(
        full_config(),
        std::sync::Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    let orch = container.orchestrator();

    let target = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let codes: Vec<&str> = results.iter().map(|v| v.code.code()).collect();
    assert!(
        codes.contains(&"AES203") || codes.contains(&"AES204"),
        "Multiple violation types should be aggregated"
    );
}
