// PURPOSE: Verify all trait implementations exist and are correctly wired.
// Contract tests confirm that concrete types satisfy their protocol traits
// at compile time — no runtime behavior is exercised here.

use external_lint_lint_arwaky::*;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::common::contract_executor_protocol::ICommandExecutorProtocol;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
use shared::external_lint::contract_external_lint_language_detector_protocol::IExternalLintLanguageDetectorProtocol;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

// ─── ILinterAdapterProtocol Implementations ───────────────

#[test]
fn eslint_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<ESLintAdapter>();
}

#[test]
fn prettier_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<PrettierAdapter>();
}

#[test]
fn tsc_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<TSCAdapter>();
}

#[test]
fn bandit_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<BanditAdapter>();
}

#[test]
fn mypy_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<MyPyAdapter>();
}

#[test]
fn ruff_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<RuffAdapter>();
}

#[test]
fn cargo_audit_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<CargoAuditAdapter>();
}

#[test]
fn rustfmt_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<RustFmtAdapter>();
}

#[test]
fn clippy_adapter_implements_linter_adapter_protocol() {
    fn assert_trait<T: ILinterAdapterProtocol>() {}
    assert_trait::<RustLinterAdapter>();
}

// ─── ICommandExecutorProtocol Implementation ──────────────

#[test]
fn stdio_client_implements_command_executor_protocol() {
    fn assert_trait<T: ICommandExecutorProtocol>() {}
    assert_trait::<StdioClient>();
}

// ─── IExternalLintExecutorProtocol Implementation ─────────

#[test]
fn external_lint_executor_implements_executor_protocol() {
    fn assert_trait<T: IExternalLintExecutorProtocol>() {}
    assert_trait::<ExternalLintExecutor>();
}

// ─── IExternalLintAggregate Implementation ────────────────

#[test]
fn orchestrator_implements_external_lint_aggregate() {
    fn assert_trait<T: IExternalLintAggregate>() {}
    assert_trait::<agent_external_lint_orchestrator::ExternalLintOrchestrator>();
}

// ─── IExternalLintSelectorProtocol Implementation ─────────

#[test]
fn selector_implements_selector_protocol() {
    fn assert_trait<T: IExternalLintSelectorProtocol>() {}
    assert_trait::<capabilities_external_lint_selector::CapabilitiesExternalLintSelector>();
}

// ─── IExternalLintLanguageDetectorProtocol Implementation ─

#[test]
fn language_detector_implements_detector_protocol() {
    fn assert_trait<T: IExternalLintLanguageDetectorProtocol>() {}
    assert_trait::<capabilities_language_detector_adapter::ExternalLintLanguageDetectorAdapter>();
}

// ─── Send + Sync Bounds ──────────────────────────────────

#[test]
fn all_adapters_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ESLintAdapter>();
    assert_send_sync::<PrettierAdapter>();
    assert_send_sync::<TSCAdapter>();
    assert_send_sync::<BanditAdapter>();
    assert_send_sync::<MyPyAdapter>();
    assert_send_sync::<RuffAdapter>();
    assert_send_sync::<CargoAuditAdapter>();
    assert_send_sync::<RustFmtAdapter>();
    assert_send_sync::<RustLinterAdapter>();
    assert_send_sync::<StdioClient>();
    assert_send_sync::<ExternalLintExecutor>();
    assert_send_sync::<ExternalLintContainer>();
}
