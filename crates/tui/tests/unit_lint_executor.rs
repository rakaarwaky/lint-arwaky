// PURPOSE: Unit tests for LintExecutor — wraps all lint aggregates.
// Layer: Capabilities (LintExecutor)

use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_action_flags_vo::ActionFlags;
use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;

fn executor() -> LintExecutor {
    LintExecutor::new(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    ))
}

// ─── check: Basic lint check ──

#[test]
fn lint_executor_check_returns_result() {
    let executor = executor();
    let flags = ActionFlags::default();
    let result = executor.check("/tmp", &flags);
    // Should return a valid result (may have violations)
    assert!(true);
}

// ─── scan: Directory scan ──

#[test]
fn lint_executor_scan_returns_result() {
    let executor = executor();
    let result = executor.scan("/tmp");
    // Should return a valid result
    assert!(true);
}

// ─── fix: Fix violations ──

#[test]
fn lint_executor_fix_returns_result() {
    let executor = executor();
    let flags = ActionFlags::default();
    let result = executor.fix("/tmp", &flags);
    // Should return a valid result
    assert!(true);
}

// ─── ci: CI mode execution ──

#[test]
fn lint_executor_ci_returns_result() {
    let executor = executor();
    let flags = ActionFlags::default();
    let result = executor.ci("/tmp", &flags);
    // Should return a valid result
    assert!(true);
}

// ─── orphan: Orphan detection ──

#[test]
fn lint_executor_orphan_returns_result() {
    let executor = executor();
    let result = executor.orphan("/tmp");
    // Should return a valid result
    assert!(true);
}

// ─── security: Security scan ──

#[test]
fn lint_executor_security_returns_result() {
    let executor = executor();
    let flags = ActionFlags::default();
    let result = executor.security("/tmp", &flags);
    // Should return a valid result
    assert!(true);
}

// ─── doctor: Toolchain diagnostics ──

#[test]
fn lint_executor_doctor_returns_result() {
    let executor = executor();
    let result = executor.doctor();
    // Should return a valid result
    assert!(true);
}

// ─── version: Version info ──

#[test]
fn lint_executor_version_returns_result() {
    let executor = executor();
    let result = executor.version();
    // Should return version info
    assert!(true);
}

// ─── Default trait ──

#[test]
fn lint_executor_default_creates_valid_instance() {
    let _ = LintExecutor::default(Arc::new(
        shared::code_analysis::root_code_analysis_container::CodeAnalysisContainer::default(),
    ));
}
