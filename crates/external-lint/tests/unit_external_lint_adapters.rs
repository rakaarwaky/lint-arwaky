// PURPOSE: Unit tests for adapter constructors, name() methods, and
// file-extension filtering logic (no subprocess execution).

use external_lint_lint_arwaky::{
    BanditAdapter, CargoAuditAdapter, ESLintAdapter, MyPyAdapter, PrettierAdapter, RuffAdapter,
    RustFmtAdapter, RustLinterAdapter, TSCAdapter,
};
use shared::code_analysis::ILinterAdapterProtocol;
use shared::common::{
    AdapterName, FilePath, ICommandExecutorProtocol, PatternList, ResponseData, Timeout,
};

use shared::external_lint::IExternalLintExecutorProtocol;
use std::sync::Arc;

// ─── Mock Executor (returns empty JSON) ───────────────────

struct EmptyJsonExecutor;

#[async_trait::async_trait]
impl ICommandExecutorProtocol for EmptyJsonExecutor {
    async fn execute_command(
        &self,
        _command: PatternList,
        _working_dir: FilePath,
        _timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        Ok(ResponseData {
            stdout: "[]".to_string(),
            stderr: String::new(),
            returncode: 0,
            ..Default::default()
        })
    }
    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}

struct EmptyLintExecutor;

#[async_trait::async_trait]
impl IExternalLintExecutorProtocol for EmptyLintExecutor {
    async fn exec_cmd_scan(
        &self,
        _args: Vec<String>,
        _working_dir: FilePath,
        _timeout_secs: f64,
        _adapter_name: Option<AdapterName>,
        _path: &FilePath,
    ) -> Result<ResponseData, shared::code_analysis::taxonomy_operation_error::LinterOperationError>
    {
        Ok(ResponseData {
            stdout: "[]".to_string(),
            ..Default::default()
        })
    }
    async fn exec_cmd_adapter(
        &self,
        _args: Vec<String>,
        _working_dir: FilePath,
        _timeout_secs: f64,
        _adapter_name: AdapterName,
    ) -> Result<ResponseData, shared::code_analysis::taxonomy_operation_error::LinterOperationError>
    {
        Ok(ResponseData {
            stdout: "[]".to_string(),
            ..Default::default()
        })
    }
    async fn js_apply_fix(
        &self,
        _path: &FilePath,
        _tool: &str,
        _fix_arg: &str,
    ) -> Result<
        shared::common::taxonomy_message_vo::ComplianceStatus,
        shared::code_analysis::taxonomy_operation_error::LinterOperationError,
    > {
        Ok(shared::common::taxonomy_message_vo::ComplianceStatus::new(
            false,
        ))
    }
}

fn mock_lint_executor() -> Arc<dyn IExternalLintExecutorProtocol> {
    Arc::new(EmptyLintExecutor)
}

fn mock_cmd_executor() -> Arc<dyn ICommandExecutorProtocol> {
    Arc::new(EmptyJsonExecutor)
}

// ─── Adapter Names ────────────────────────────────────────

#[test]
fn eslint_adapter_name() {
    let adapter = ESLintAdapter::new(mock_lint_executor());
    assert_eq!(adapter.name().value(), "eslint");
}

#[test]
fn prettier_adapter_name() {
    let adapter = PrettierAdapter::new(mock_lint_executor());
    assert_eq!(adapter.name().value(), "prettier");
}

#[test]
fn tsc_adapter_name() {
    let adapter = TSCAdapter::new(mock_lint_executor());
    assert_eq!(adapter.name().value(), "tsc");
}

#[test]
fn bandit_adapter_name() {
    let adapter = BanditAdapter::new(
        mock_lint_executor(),
        None,
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    assert_eq!(adapter.name().value(), "bandit");
}

#[test]
fn mypy_adapter_name() {
    let adapter = MyPyAdapter::new(
        mock_lint_executor(),
        None,
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    assert_eq!(adapter.name().value(), "mypy");
}

#[test]
fn ruff_adapter_name() {
    let adapter = RuffAdapter::new(
        mock_lint_executor(),
        None,
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    assert_eq!(adapter.name().value(), "ruff");
}

#[test]
fn cargo_audit_adapter_name() {
    let adapter = CargoAuditAdapter::new(Arc::new(filesystem::FilesystemOrchestrator::new()));
    assert_eq!(adapter.name().value(), "cargo-audit");
}

#[test]
fn rustfmt_adapter_name() {
    let adapter = RustFmtAdapter::new(
        mock_cmd_executor(),
        None,
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    assert_eq!(adapter.name().value(), "rustfmt");
}

#[test]
fn clippy_adapter_name() {
    let adapter = RustLinterAdapter::new(
        mock_cmd_executor(),
        None,
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    assert_eq!(adapter.name().value(), "clippy");
}

// ─── File Extension Filtering ─────────────────────────────

#[tokio::test]
async fn eslint_skips_non_js_file() {
    let adapter = ESLintAdapter::new(mock_lint_executor());
    let path = FilePath::new("/tmp/test.rs".to_string()).unwrap();
    let result = adapter.scan(&path).await.unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn prettier_skips_non_js_file() {
    let adapter = PrettierAdapter::new(mock_lint_executor());
    let path = FilePath::new("/tmp/test.py".to_string()).unwrap();
    let result = adapter.scan(&path).await.unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn tsc_skips_non_ts_file() {
    let adapter = TSCAdapter::new(mock_lint_executor());
    let path = FilePath::new("/tmp/test.js".to_string()).unwrap();
    let result = adapter.scan(&path).await.unwrap();
    assert!(result.is_empty());
}

// ─── Constructors with bin_path ───────────────────────────

#[test]
fn ruff_adapter_with_custom_bin_path() {
    let bin = FilePath::new("/usr/local/bin/ruff".to_string()).unwrap();
    let adapter = RuffAdapter::new(
        mock_lint_executor(),
        Some(bin),
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    assert_eq!(adapter.name().value(), "ruff");
}

#[test]
fn bandit_adapter_with_custom_bin_path() {
    let bin = FilePath::new("/usr/local/bin/bandit".to_string()).unwrap();
    let adapter = BanditAdapter::new(
        mock_lint_executor(),
        Some(bin),
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    assert_eq!(adapter.name().value(), "bandit");
}

#[test]
fn mypy_adapter_with_custom_bin_path() {
    let bin = FilePath::new("/usr/local/bin/mypy".to_string()).unwrap();
    let adapter = MyPyAdapter::new(
        mock_lint_executor(),
        Some(bin),
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    assert_eq!(adapter.name().value(), "mypy");
}

// ─── CargoAudit Default ───────────────────────────────────

#[test]
fn cargo_audit_default_constructor() {
    let adapter = CargoAuditAdapter::default();
    assert_eq!(adapter.name().value(), "cargo-audit");
}

// ─── apply_fix for non-fixable adapters ───────────────────

#[tokio::test]
async fn tsc_apply_fix_returns_false() {
    let adapter = TSCAdapter::new(mock_lint_executor());
    let path = FilePath::new("/tmp/test.ts".to_string()).unwrap();
    let result = adapter.apply_fix(&path).await.unwrap();
    assert!(!result.value());
}

#[tokio::test]
async fn bandit_apply_fix_returns_false() {
    let adapter = BanditAdapter::new(
        mock_lint_executor(),
        None,
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    let path = FilePath::new("/tmp/test.py".to_string()).unwrap();
    let result = adapter.apply_fix(&path).await.unwrap();
    assert!(!result.value());
}

#[tokio::test]
async fn mypy_apply_fix_returns_false() {
    let adapter = MyPyAdapter::new(
        mock_lint_executor(),
        None,
        Arc::new(filesystem::FilesystemOrchestrator::new()),
    );
    let path = FilePath::new("/tmp/test.py".to_string()).unwrap();
    let result = adapter.apply_fix(&path).await.unwrap();
    assert!(!result.value());
}
