//! Tests for config_show() — verifies parsed config output via IConfigOrchestrationAggregate.
//!
//! These tests exercise the config_show() method of LintExecutor when a real
//! IConfigOrchestrationAggregate is wired up, confirming that the returned
//! output is non-empty, structurally valid, and contains expected rules/layers.

use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::BooleanVO;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;
use shared::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
use shared::config_system::contract_reader_port::IConfigReaderPort;
use shared::config_system::contract_workspace_detector_port::{
    IWorkspaceDetectorPort, WorkspaceType,
};
use shared::config_system::taxonomy_config_vo::{ArchitectureConfig, ArchitectureRule};
use shared::config_system::taxonomy_source_vo::{ConfigResult, ConfigSource};
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_lint_result_vo::LintExecutionResult;
use std::sync::Arc;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;
use tui_lint_arwaky::capabilities_report_formatter::ReportFormatterHelper;

// ---------------------------------------------------------------------------
// Minimal mocks (config_show does not invoke code analysis)
// ---------------------------------------------------------------------------

struct MockCodeAnalysis;

impl ICodeAnalysisAggregate for MockCodeAnalysis {
    fn run_code_analysis(&self, _: &str) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_dir(&self, _: &str) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_path(&self, _: &str) -> Vec<LintResult> {
        vec![]
    }
    fn calc_score(&self, _: &[LintResult]) -> f64 {
        100.0
    }
    fn check_critical(&self, _: &[LintResult]) -> bool {
        false
    }
    fn format_report(&self, _: &LintResultList, _: &str) -> String {
        String::new()
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

struct MockWorkspaceDetector;

impl IWorkspaceDetectorPort for MockWorkspaceDetector {
    fn detect(&self, _: &FilePath) -> WorkspaceType {
        WorkspaceType::Rust
    }
    fn is_workspace(&self, _: &FilePath) -> bool {
        true
    }
}

struct MockConfigReader;

#[async_trait::async_trait]
impl IConfigReaderPort for MockConfigReader {
    async fn read_config(&self, _: &FilePath, _: &str) -> Option<ConfigSource> {
        None
    }
    async fn list_config_files(&self, _: &FilePath) -> Vec<(String, String)> {
        vec![]
    }
}

/// Returns a stub ConfigResult (for unused trait methods).
fn stub_config_result() -> ConfigResult {
    ConfigResult::new(
        ArchitectureConfig::default(),
        ConfigSource::new("rust", "", ""),
        vec![],
    )
}

// ---------------------------------------------------------------------------
// Rich orchestrator — 2 rules, 2 layers
// ---------------------------------------------------------------------------

struct MockConfigOrchestratorRich;

#[async_trait::async_trait]
impl IConfigOrchestrationAggregate for MockConfigOrchestratorRich {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorPort> {
        Arc::new(MockWorkspaceDetector)
    }
    fn config_reader(&self) -> Arc<dyn IConfigReaderPort> {
        Arc::new(MockConfigReader)
    }
    async fn load_project_config(&self, _: &FilePath) -> ConfigResult {
        let mut layers = std::collections::HashMap::new();
        layers.insert(LayerNameVO::new("presentation"), LayerDefinition::default());
        layers.insert(LayerNameVO::new("domain"), LayerDefinition::default());
        let rules = vec![
            ArchitectureRule {
                name: DescriptionVO::new("AES301 - No cross-layer imports"),
                scope: LayerNameVO::new("presentation"),
                ..Default::default()
            },
            ArchitectureRule {
                name: DescriptionVO::new("AES302 - Domain isolation"),
                scope: LayerNameVO::new("domain"),
                ..Default::default()
            },
        ];
        let config = ArchitectureConfig {
            enabled: BooleanVO::new(true),
            layers,
            rules,
            ..Default::default()
        };
        let source = ConfigSource::new(
            "rust",
            "/test/lint_arwaky.config.rust.yaml",
            "architecture: {enabled: true}",
        );
        ConfigResult::new(config, source, vec![])
    }
    async fn load_config_for_language(&self, _: &FilePath, _: &str) -> ConfigResult {
        stub_config_result()
    }
}

// ---------------------------------------------------------------------------
// Empty orchestrator — empty config with warning
// ---------------------------------------------------------------------------

struct MockConfigOrchestratorEmpty;

#[async_trait::async_trait]
impl IConfigOrchestrationAggregate for MockConfigOrchestratorEmpty {
    fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorPort> {
        Arc::new(MockWorkspaceDetector)
    }
    fn config_reader(&self) -> Arc<dyn IConfigReaderPort> {
        Arc::new(MockConfigReader)
    }
    async fn load_project_config(&self, _: &FilePath) -> ConfigResult {
        ConfigResult::new(
            ArchitectureConfig::default(),
            ConfigSource::new("rust", "/empty.yaml", ""),
            vec!["No config found".to_string()],
        )
    }
    async fn load_config_for_language(&self, _: &FilePath, _: &str) -> ConfigResult {
        stub_config_result()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn config_show_with_orchestrator_returns_rules_and_layers() {
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis), Arc::new(ReportFormatterHelper))
        .with_config(Arc::new(MockConfigOrchestratorRich));
    let result: LintExecutionResult = executor.config_show();

    assert!(result.success, "config_show should succeed");
    assert!(!result.output.is_empty(), "output must not be empty");

    // Structural checks
    assert!(
        result.output.contains("Active Configuration"),
        "missing header"
    );
    assert!(result.output.contains("Rules: 2"), "wrong rule count");
    assert!(result.output.contains("Layers: 2"), "wrong layer count");
    assert!(
        result.output.contains("Enabled: true"),
        "missing enabled flag"
    );

    // Rule content
    assert!(result.output.contains("AES301"), "missing rule AES301");
    assert!(result.output.contains("AES302"), "missing rule AES302");
    assert!(
        result.output.contains("[presentation]"),
        "missing presentation scope"
    );
    assert!(result.output.contains("[domain]"), "missing domain scope");

    // Layer content
    assert!(
        result.output.contains("Architecture Layers"),
        "missing layers section"
    );
    assert!(
        result.output.contains("presentation"),
        "missing presentation layer"
    );
    assert!(result.output.contains("domain"), "missing domain layer");
}

#[test]
fn config_show_with_orchestrator_empty_config_shows_warnings() {
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis), Arc::new(ReportFormatterHelper))
        .with_config(Arc::new(MockConfigOrchestratorEmpty));
    let result: LintExecutionResult = executor.config_show();

    assert!(
        result.success,
        "config_show should succeed even with empty config"
    );
    assert!(
        result.output.contains("Active Configuration"),
        "missing header"
    );
    assert!(result.output.contains("Rules: 0"), "wrong rule count");
    assert!(result.output.contains("Layers: 0"), "wrong layer count");
    assert!(result.output.contains("No config found"), "missing warning");
}
