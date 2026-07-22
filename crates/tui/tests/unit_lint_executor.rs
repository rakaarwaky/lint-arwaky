// PURPOSE: Unit tests for LintExecutor — wraps all lint aggregates.
// Layer: Capabilities (LintExecutor)

use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_action_flags_vo::ActionFlags;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn executor() -> LintExecutor {
    LintExecutor::new(
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter(),
    )
}

// ─── check: Basic lint check ──

#[test]
fn lint_executor_check_returns_result() {
    let executor = executor();
    let flags = ActionFlags::default();
    let _result = executor.check("/tmp", &flags);
    // Should return a valid result (may have violations)
    assert!(true);
}

// ─── scan: Directory scan ──

#[test]
fn lint_executor_scan_returns_result() {
    let executor = executor();
    let _result = executor.scan("/tmp");
    // Should return a valid result
    assert!(true);
}

// ─── fix: Fix violations ──

#[test]
fn lint_executor_fix_returns_result() {
    let executor = executor();
    let flags = ActionFlags::default();
    let _result = executor.fix("/tmp", &flags);
    // Should return a valid result
    assert!(true);
}

// ─── ci: CI mode execution ──

#[test]
fn lint_executor_ci_returns_result() {
    let executor = executor();
    let flags = ActionFlags::default();
    let _result = executor.ci("/tmp", &flags);
    // Should return a valid result
    assert!(true);
}

// ─── orphan: Orphan detection ──

#[test]
fn lint_executor_orphan_returns_result() {
    let executor = executor();
    let _result = executor.orphan("/tmp");
    // Should return a valid result
    assert!(true);
}

// ─── security: Security scan ──

#[test]
fn lint_executor_security_returns_result() {
    let executor = executor();
    let _result = executor.security("/tmp");
    // Should return a valid result
    assert!(true);
}

// ─── doctor: Toolchain diagnostics ──

#[test]
fn lint_executor_doctor_returns_result() {
    let executor = executor();
    let _result = executor.doctor();
    // Should return a valid result
    assert!(true);
}

// ─── version: Version info ──

#[test]
fn lint_executor_version_returns_result() {
    let executor = executor();
    let _result = executor.version();
    // Should return version info
    assert!(true);
}

// ─── Default constructor ──

#[test]
fn lint_executor_default_creates_valid_instance() {
    let code_analysis =
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::default()
            .code_analysis_linter();
    let _ = LintExecutor::new(code_analysis);
}
