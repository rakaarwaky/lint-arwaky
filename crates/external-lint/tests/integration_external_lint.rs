// Integration tests — verify individual adapter creation and selector wiring.
//
// These tests construct real adapters with mock executors/filesystems and verify
// that the subsystem wires together correctly. ExternalLintContainer::default()
// panics, so we test individual component creation instead.

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use std::sync::Arc;

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::IExternalLintExecutorProtocol;
use shared::external_lint::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::external_lint::contract_external_lint_selector_protocol::IExternalLintSelectorProtocol;

use mock_filesystem::MockFilesystem;

struct MockLintExecutor;
impl IExternalLintExecutorProtocol for MockLintExecutor {
    fn exec_cmd_scan(
        &self,
        _: Vec<String>,
        _: FilePath,
        _: f64,
        _: Option<AdapterName>,
        _: &FilePath,
    ) -> Result<ResponseData, LinterOperationError> {
        Ok(ResponseData::default())
    }
    fn exec_cmd_adapter(
        &self,
        _: Vec<String>,
        _: FilePath,
        _: f64,
        _: AdapterName,
    ) -> Result<ResponseData, LinterOperationError> {
        Ok(ResponseData::default())
    }
    fn js_apply_fix(
        &self,
        _: &FilePath,
        _: &str,
        _: &str,
    ) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

struct MockCmdExecutor;
impl shared::external_lint::contract_executor_protocol::ICommandExecutorProtocol
    for MockCmdExecutor
{
    fn execute_command(
        &self,
        _: shared::common::taxonomy_common_vo::PatternList,
        _: FilePath,
        _: Option<shared::common::taxonomy_duration_vo::Timeout>,
    ) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::default())
    }
    fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::default())
    }
}

// ─── Integration: Individual adapter creation ─────────────

#[test]
fn create_ruff_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::RuffAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty()); // no Python files in /tmp with mock filesystem
}

#[test]
fn create_bandit_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::BanditAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_mypy_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::MyPyAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_clippy_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::RustLinterAdapter::new(
        Arc::new(MockCmdExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty()); // no Cargo.toml at /tmp
}

#[test]
fn create_rustfmt_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::RustFmtAdapter::new(
        Arc::new(MockCmdExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_cargo_audit_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::CargoAuditAdapter::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty()); // no Cargo.lock at /tmp
}

#[test]
fn create_eslint_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::ESLintAdapter::new(
        Arc::new(MockLintExecutor),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_prettier_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::PrettierAdapter::new(
        Arc::new(MockLintExecutor),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

#[test]
fn create_tsc_adapter_and_scan_returns_empty() {
    let adapter = external_lint_lint_arwaky::TSCAdapter::new(
        Arc::new(MockLintExecutor),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let result = adapter.scan(&path).unwrap();
    assert!(result.values.is_empty());
}

// ─── Integration: Selector with defaults ──────────────────

#[test]
fn selector_with_defaults_selects_all_nine_for_mixed_project() {
    use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;

    let selector = CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, true);
    assert_eq!(selected.len(), 9);
}

#[test]
fn selector_with_custom_lists() {
    use external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector;

    let selector = CapabilitiesExternalLintSelector::new(
        vec![AdapterName::raw("clippy")],
        vec![AdapterName::raw("ruff")],
        vec![],
    );
    let selected = selector.select_adapters(true, true, false);
    assert_eq!(selected.len(), 2); // only rust + python, no js
    let names: Vec<&str> = selected.iter().map(|a| a.value()).collect();
    assert!(names.contains(&"clippy"));
    assert!(names.contains(&"ruff"));
}

// ─── Integration: apply_fix on adapters ───────────────────

#[test]
fn bandit_apply_fix_always_returns_false() {
    let adapter = external_lint_lint_arwaky::BanditAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.apply_fix(&path).unwrap();
    assert!(!status.value); // bandit doesn't fix
}

#[test]
fn mypy_apply_fix_always_returns_false() {
    let adapter = external_lint_lint_arwaky::MyPyAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.apply_fix(&path).unwrap();
    assert!(!status.value); // mypy doesn't fix
}

#[test]
fn tsc_apply_fix_always_returns_false() {
    let adapter = external_lint_lint_arwaky::TSCAdapter::new(
        Arc::new(MockLintExecutor),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.apply_fix(&path).unwrap();
    assert!(!status.value); // tsc doesn't fix
}

#[test]
fn ruff_apply_fix_returns_true() {
    let adapter = external_lint_lint_arwaky::RuffAdapter::new(
        Arc::new(MockLintExecutor),
        None,
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.apply_fix(&path).unwrap();
    assert!(status.value); // ruff --fix exits 0 → true
}

#[test]
fn cargo_audit_apply_fix_returns_true() {
    // cargo-audit has no fix, but returns true (manual update needed)
    let adapter = external_lint_lint_arwaky::CargoAuditAdapter::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let status = adapter.apply_fix(&path).unwrap();
    assert!(status.value);
}
