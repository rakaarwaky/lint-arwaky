use code_analysis_lint_arwaky::root_code_analysis_container::{
    CodeAnalysisCheckerContainer, CodeAnalysisContainer,
};

#[test]
fn checker_container_default_constructs() {
    let container = CodeAnalysisCheckerContainer::default();
    let analyzer = container.analyzer();
    let _config = analyzer.config();
    // All checkers should be accessible
    let _bypass = container.bypass_checker();
    let _line = container.line_checker();
    let _class = container.class_checker();
    let _dup = container.duplication_checker();
    let _dead = container.dead_inheritance_checker();
}

#[test]
fn checker_container_detect_layer_returns_none_for_empty_analyzer() {
    let container = CodeAnalysisCheckerContainer::default();
    let layer = container.detect_layer("src/foo.rs", ".");
    assert!(layer.is_none());
}

#[test]
fn container_default_constructs() {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let _ = linter;
}

#[test]
fn container_with_analyzer_constructs() {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let results = linter.run_code_analysis_path("/nonexistent/path/xyz");
    assert!(results.is_empty());
}

#[test]
fn container_default_is_same_as_new() {
    let c1 = CodeAnalysisContainer::new();
    let c2 = CodeAnalysisContainer::default();
    let r1 = c1
        .code_analysis_linter()
        .run_code_analysis_path("/nonexistent");
    let r2 = c2
        .code_analysis_linter()
        .run_code_analysis_path("/nonexistent");
    assert_eq!(r1.len(), r2.len());
}

#[test]
fn container_new_with_analyzer_provides_custom_analyzer() {
    use std::sync::Arc;
    // Use a custom placeholder analyzer
    let custom = Arc::new(TestAnalyzer);
    let container = CodeAnalysisContainer::new_with_analyzer(custom);
    let _linter = container.code_analysis_linter();
}

/// Minimal ILayerDetectionProtocol implementation for testing container wiring
struct TestAnalyzer;

impl shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol
    for TestAnalyzer
{
    fn config(&self) -> &shared::config_system::taxonomy_config_vo::ArchitectureConfig {
        static C: std::sync::OnceLock<
            shared::config_system::taxonomy_config_vo::ArchitectureConfig,
        > = std::sync::OnceLock::new();
        C.get_or_init(|| {
            let mut config =
                shared::config_system::taxonomy_config_vo::ArchitectureConfig::default();
            config.enabled = shared::taxonomy_common_vo::BooleanVO::new(false);
            config
        })
    }
    fn detect_layer(&self, _file_path: &str, _root_dir: &str) -> Option<String> {
        None
    }
    fn get_layer_def(
        &self,
        _layer: &str,
    ) -> Option<shared::common::taxonomy_definition_vo::LayerDefinition> {
        None
    }
    fn get_orphan_entry_points(&self) -> Vec<String> {
        Vec::new()
    }
    fn extract_layer_from_prefix(&self, _filename: &str) -> Option<String> {
        None
    }
    fn resolve_specialized_layer(&self, base_layer: &str, _file_path: &str) -> String {
        base_layer.to_string()
    }
    fn detect_module_layer(&self, _module: &str) -> Option<String> {
        None
    }
    fn refine_module_layer(&self, base_name: &str, _parts: &[&str]) -> String {
        base_name.to_string()
    }
}
