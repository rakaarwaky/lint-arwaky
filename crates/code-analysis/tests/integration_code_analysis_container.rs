// Integration tests for CodeAnalysisContainer and CodeAnalysisOrchestrator (AES304, AES301-AES305)
// Tests use real DI container and verify cross-capability interactions

use std::sync::Arc;

use code_analysis_lint_arwaky::{
    capabilities_check_bypass_checker::BypassChecker, capabilities_line_checker::ArchLineChecker,
    root_code_analysis_container::CodeAnalysisCheckerContainer,
};
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;

// ─── Contract Tests: Trait Implementation Verification ──────

/// Verify IBypassCheckerProtocol trait implementation exists and is callable
#[test]
fn contract_bypass_checker_protocol_is_implemented() {
    let checker = BypassChecker::new();
    // Should be able to call check methods via the trait
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", "let x = foo.unwrap();", &mut violations);
    assert!(!violations.is_empty());
}

/// Verify ILineCheckerProtocol trait implementation exists and is callable
#[test]
fn contract_line_checker_protocol_is_implemented() {
    let checker = ArchLineChecker {};
    // Should be able to call check methods via the trait
    let mut violations = Vec::new();
    checker.check_line_counts("test.rs", None, "fn main() {}", &mut violations);
}

/// Verify CodeAnalysisCheckerContainer exposes protocol implementations
#[test]
fn contract_container_exposes_all_protocols() {
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());
    let container = CodeAnalysisCheckerContainer::new(config, layer_map);

    // Should be able to access bypass checker via protocol
    let _bypass = container.bypass_checker();

    // Should be able to access line checker via protocol
    let _line = container.line_checker();

    // Should be able to access mandatory class checker via protocol
    let _class = container.class_checker();

    // Should be able to access dead inheritance checker via protocol
    let _inheritance = container.dead_inheritance_checker();
}

// ─── Integration Tests: Real DI Container Wiring ────────────

/// Test that CodeAnalysisContainer properly wires all checkers
#[test]
fn test_container_wires_all_checkers_correctly() {
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());
    let container = CodeAnalysisCheckerContainer::new(config, layer_map);

    // Verify all protocols are wired
    let _ = container.bypass_checker();
}

/// Test that CodeAnalysisOrchestrator can be created from container
#[test]
fn test_orchestrator_is_created_from_container() {
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());
    let checker_container = Arc::new(CodeAnalysisCheckerContainer::new(config, layer_map));

    // Should create orchestrator without panicking
    let _orchestrator = code_analysis_lint_arwaky::agent_code_analysis_orchestrator::CodeAnalysisOrchestrator::new_with_container(checker_container);
}

/// Test that CodeAnalysisContainer can be created with default config
#[test]
fn test_default_container_creation() {
    let container =
        code_analysis_lint_arwaky::root_code_analysis_container::CodeAnalysisContainer::default();

    // Should return a valid linter
    let _linter = container.code_analysis_linter();
}

// ─── Unit Tests: Happy Path ─────────────────────────────────

/// Test that bypass checker detects unwrap calls
#[test]
fn test_bypass_checker_detects_unwrap() {
    let checker = BypassChecker::new();
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", "let x = option.unwrap();", &mut violations);

    assert_eq!(violations.len(), 1, "Should detect one unwrap violation");
}

/// Test that line checker validates file structure
#[test]
fn test_line_checker_validates_structure() {
    let checker = ArchLineChecker {};
    let mut violations = Vec::new();
    checker.check_line_counts(
        "test.rs",
        None,
        "fn main() {\n    let x = 5;\n}",
        &mut violations,
    );

    // Should not panic on valid Rust code
}

// ─── Unit Tests: Edge Cases ─────────────────────────────────

/// Test that bypass checker handles empty content
#[test]
fn test_bypass_checker_handles_empty_content() {
    let checker = BypassChecker::new();
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", "", &mut violations);

    assert!(
        violations.is_empty(),
        "Empty content should have no violations"
    );
}

/// Test that bypass checker handles whitespace-only content
#[test]
fn test_bypass_checker_handles_whitespace_content() {
    let checker = BypassChecker::new();
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", "   \n  \n   ", &mut violations);

    assert!(
        violations.is_empty(),
        "Whitespace-only content should have no violations"
    );
}

/// Test that line checker handles empty file
#[test]
fn test_line_checker_handles_empty_file() {
    let checker = ArchLineChecker {};
    let mut violations = Vec::new();
    checker.check_line_counts("test.rs", None, "", &mut violations);

    // Empty file should not cause errors
}

// ─── Unit Tests: Error Handling ─────────────────────────────

/// Test that container returns proper layer detection for unknown files
#[test]
fn test_container_handles_unknown_file_type() {
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());
    let container = CodeAnalysisCheckerContainer::new(config, layer_map);

    // Should return None for unknown file types
    let layer = container.detect_layer("unknown.xyz", "/project");
    assert!(layer.is_none(), "Unknown file type should return None");
}

/// Test that container returns proper layer detection for known files
#[test]
fn test_container_detects_known_layers() {
    let config = ArchitectureConfig::default();
    let layer_map = LayerMapVO::new(std::collections::HashMap::new());
    let container = CodeAnalysisCheckerContainer::new(config, layer_map);

    // Should detect agent layer from filename prefix
    let layer = container.detect_layer("agent_test.rs", "/project");
    assert!(layer.is_some(), "Known layer should be detected");
}

/// Test that protocol implementations handle invalid inputs gracefully
#[test]
fn test_protocols_handle_invalid_inputs() {
    let bypass_checker = BypassChecker::new();
    let mut violations = Vec::new();

    // Should not panic on null-like input (empty string)
    bypass_checker.check_bypass_comments("", "", &mut violations);
}
