use code_analysis_lint_arwaky::{
    capabilities_check_bypass_checker::BypassChecker, capabilities_line_checker::ArchLineChecker,
};
use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;

// ─── Contract Tests: Trait Implementation Verification ──────

/// Verify IBypassCheckerProtocol trait implementation exists and is callable
#[test]
fn contract_bypass_checker_protocol_is_implemented() {
    let checker = BypassChecker::new();
    let mut violations = Vec::new();
    checker.check_bypass_comments("test.rs", "let x = foo.unwrap();", &mut violations);
    assert!(!violations.is_empty());
}

/// Verify ILineCheckerProtocol trait implementation exists and is callable
#[test]
fn contract_line_checker_protocol_is_implemented() {
    let checker = ArchLineChecker {};
    let mut violations = Vec::new();
    checker.check_line_counts("test.rs", None, "fn main() {}", &mut violations);
}

/// Test that CodeAnalysisContainer can be created with default config
#[test]
fn test_default_container_creation() {
    let container =
        code_analysis_lint_arwaky::root_code_analysis_container::CodeAnalysisContainer::default();
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
}

// ─── Unit Tests: Error Handling ─────────────────────────────

/// Test that protocol implementations handle invalid inputs gracefully
#[test]
fn test_protocols_handle_invalid_inputs() {
    let bypass_checker = BypassChecker::new();
    let mut violations = Vec::new();
    bypass_checker.check_bypass_comments("", "", &mut violations);
}
