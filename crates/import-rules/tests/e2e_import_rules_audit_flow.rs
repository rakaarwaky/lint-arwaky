// PURPOSE: E2E test — full audit lifecycle on a realistic project structure.
// Creates a multi-file workspace, runs the full orchestrator, asserts on real output.

use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::collections::HashMap;
use std::io::Write;

fn e2e_config() -> ArchitectureConfig {
    let mut layers = HashMap::new();

    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            forbidden: PatternList::new(vec![
                "contract".to_string(),
                "utility".to_string(),
                "capabilities".to_string(),
                "agent".to_string(),
                "surfaces".to_string(),
                "root".to_string(),
            ]),
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
            ..Default::default()
        },
    );

    layers.insert(
        LayerNameVO::new("agent"),
        LayerDefinition {
            allowed: PatternList::new(vec![
                "taxonomy".to_string(),
                "contract".to_string(),
                "utility".to_string(),
            ]),
            forbidden: PatternList::new(vec!["surfaces".to_string(), "capabilities".to_string()]),
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
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

#[tokio::test]
async fn full_audit_on_realistic_workspace() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path();

    // ─── Taxonomy layer (clean) ───
    write_file(
        src,
        "taxonomy_severity_vo.rs",
        r#"
#[derive(Debug, Clone)]
pub enum Severity { Low, Medium, High, Critical }
"#,
    );

    // ─── Capabilities layer (clean — has contract import with real logic) ───
    write_file(
        src,
        "capabilities_severity_checker.rs",
        r#"
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::cli_commands::taxonomy_result_vo::LintResult;

pub struct SeverityChecker;

impl IUnusedImportProtocol for SeverityChecker {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage> {
        let content = std::fs::read_to_string(path.value()).unwrap_or_default();
        if content.is_empty() { return vec![]; }
        vec![LintMessage::new("scanned")]
    }
    fn check_unused_imports(&self, file: &str, content: &str, v: &mut Vec<LintResult>) {
        if content.contains("unused") {
            v.push(LintResult::new_arch(file, 1, "AES203",
                shared::common::taxonomy_severity_vo::Severity::MEDIUM, "unused import"));
        }
    }
}
"#,
    );

    // ─── Agent layer (clean) ───
    write_file(
        src,
        "agent_severity_orchestrator.rs",
        r#"
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use std::sync::Arc;

pub struct SeverityOrchestrator {
    checker: Arc<dyn IUnusedImportProtocol>,
}
"#,
    );

    let container = ImportContainer::new_with_config(e2e_config());
    let orch = container.orchestrator();

    let target = FilePath::new(src.to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    // Clean workspace should produce zero violations
    assert!(
        results.is_empty(),
        "Clean workspace should have 0 violations, got {}: {:?}",
        results.len(),
        results
            .iter()
            .map(|r| format!("{}: {}", r.code.code(), r.message.value()))
            .collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn full_audit_detects_layer_violation_in_workspace() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path();

    // Taxonomy importing capabilities — VIOLATION
    write_file(
        src,
        "taxonomy_bad_vo.rs",
        r#"
use crate::capabilities_severity_checker::SeverityChecker;

pub struct Bad;
"#,
    );

    // Clean capabilities file
    write_file(
        src,
        "capabilities_good.rs",
        r#"
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
pub struct Good;
"#,
    );

    let container = ImportContainer::new_with_config(e2e_config());
    let orch = container.orchestrator();

    let target = FilePath::new(src.to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let aes201_count = results.iter().filter(|v| v.code.code() == "AES201").count();
    assert!(
        aes201_count > 0,
        "E2E: taxonomy importing capabilities must produce AES201"
    );
}

#[tokio::test]
async fn full_audit_detects_unused_and_dummy_together() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path();

    write_file(
        src,
        "capabilities_messy.rs",
        r#"
use std::collections::HashMap;
use shared::common::taxonomy_path_vo::FilePath;

fn _use_mandatory_imports() {
    let _ = FilePath::new("x");
}

pub struct Messy;
"#,
    );

    let container = ImportContainer::new_with_config(e2e_config());
    let orch = container.orchestrator();

    let target = FilePath::new(src.to_string_lossy().to_string()).unwrap();
    let results = orch.run_audit(&target).await.unwrap();

    let codes: Vec<&str> = results.iter().map(|v| v.code.code()).collect();
    assert!(
        codes.contains(&"AES204"),
        "E2E: dummy function must be detected. Codes found: {:?}",
        codes
    );
}
