// Smoke tests — quick boot + respond within 5s.
//
// These tests verify that the core components can be instantiated quickly
// and that basic operations complete without hanging or panicking.

#[allow(dead_code, unused_imports)]
#[path = "../../shared/tests/common/mock_filesystem.rs"]
mod mock_filesystem;

use std::sync::Arc;
use std::time::Instant;

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_protocol::ICommandExecutorProtocol;
use shared::external_lint::contract_external_lint_protocol::IExternalLintSelectorProtocol;
use shared::external_lint::contract_external_lint_protocol::IJsToolResolutionProtocol;
use shared::external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;

use mock_filesystem::MockFilesystem;

struct MockCmdExecutor;
impl ICommandExecutorProtocol for MockCmdExecutor {
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
}

struct MockJsResolution;
impl IJsToolResolutionProtocol for MockJsResolution {
    fn resolve_js_cmd(
        &self,
        _: &shared::filesystem::taxonomy_filesystem_vo::ToolName,
        _: Vec<String>,
        _: &FilePath,
    ) -> Option<Vec<String>> {
        None
    }
    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
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

const SMOKE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

// ─── Smoke: Selector creation and adapter selection ───────

#[test]
fn smoke_selector_creation_and_selection() {
    let start = Instant::now();
    let selector = external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, true, true);
    assert_eq!(selected.len(), 9);
    assert!(start.elapsed() < SMOKE_TIMEOUT);
}

#[test]
fn smoke_selector_rust_only() {
    let start = Instant::now();
    let selector = external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(true, false, false);
    assert_eq!(selected.len(), 3);
    assert!(start.elapsed() < SMOKE_TIMEOUT);
}

#[test]
fn smoke_selector_python_only() {
    let start = Instant::now();
    let selector = external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, true, false);
    assert_eq!(selected.len(), 3);
    assert!(start.elapsed() < SMOKE_TIMEOUT);
}

#[test]
fn smoke_selector_js_only() {
    let start = Instant::now();
    let selector = external_lint_lint_arwaky::capabilities_external_lint_selector::CapabilitiesExternalLintSelector::with_defaults();
    let selected = selector.select_adapters(false, false, true);
    assert_eq!(selected.len(), 3);
    assert!(start.elapsed() < SMOKE_TIMEOUT);
}

// ─── Smoke: StdioClient creation ──────────────────────────

#[test]
fn smoke_stdio_client_creation() {
    let start = Instant::now();
    let client = external_lint_lint_arwaky::StdioClient::new(Timeout::new(5.0));
    let health = client.health_check();
    assert!(health.is_ok());
    assert!(start.elapsed() < SMOKE_TIMEOUT);
}

// ─── Smoke: Adapter creation with mock executor ───────────

#[test]
fn smoke_all_adapters_created_quickly() {
    let start = Instant::now();

    let lint_exec: Arc<dyn ICommandExecutorProtocol> = Arc::new(MockCmdExecutor);
    let cmd_exec: Arc<
        dyn shared::external_lint::contract_external_lint_protocol::ICommandExecutorProtocol,
    > = Arc::new(MockCmdExecutor);
    let _fs: Arc<dyn shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate> =
        Arc::new(MockFilesystem::new());
    let tr: Arc<dyn shared::filesystem::IToolResolutionProtocol> = Arc::new(MockFilesystem::new());
    let io: Arc<dyn shared::filesystem::IFileSystemIOProtocol> = Arc::new(MockFilesystem::new());

    let path = FilePath::new("/tmp".to_string()).unwrap();

    // Python adapters
    let _ruff = external_lint_lint_arwaky::RuffAdapter::new(
        lint_exec.clone(),
        None,
        io.clone(),
        tr.clone(),
    );
    let _bandit = external_lint_lint_arwaky::BanditAdapter::new(
        lint_exec.clone(),
        None,
        io.clone(),
        tr.clone(),
    );
    let _mypy = external_lint_lint_arwaky::MyPyAdapter::new(
        lint_exec.clone(),
        None,
        io.clone(),
        tr.clone(),
    );

    // JS adapters
    let js_res: Arc<dyn IJsToolResolutionProtocol> = Arc::new(MockJsResolution);
    let _eslint = external_lint_lint_arwaky::ESLintAdapter::new(
        lint_exec.clone(),
        js_res.clone(),
        io.clone(),
        tr.clone(),
    );
    let _prettier = external_lint_lint_arwaky::PrettierAdapter::new(
        lint_exec.clone(),
        js_res.clone(),
        io.clone(),
        tr.clone(),
    );
    let _tsc =
        external_lint_lint_arwaky::TSCAdapter::new(lint_exec.clone(), io.clone(), tr.clone());

    // Rust adapters
    let _clippy =
        external_lint_lint_arwaky::RustLinterAdapter::new(cmd_exec.clone(), None, tr.clone());
    let _fmt = external_lint_lint_arwaky::RustFmtAdapter::new(cmd_exec.clone(), None, tr.clone());
    let _audit = external_lint_lint_arwaky::CargoAuditAdapter::new(cmd_exec.clone(), tr.clone());

    // Scan each (should return immediately with mock filesystem)
    let _ = _ruff.scan(&path);
    let _ = _bandit.scan(&path);
    let _ = _mypy.scan(&path);
    let _ = _eslint.scan(&path);
    let _ = _prettier.scan(&path);
    let _ = _tsc.scan(&path);
    let _ = _clippy.scan(&path);
    let _ = _fmt.scan(&path);
    let _ = _audit.scan(&path);

    assert!(start.elapsed() < SMOKE_TIMEOUT);
}

// ─── Smoke: ExternalLintExecutor creation ─────────────────

#[test]
fn smoke_external_lint_executor_creation() {
    let start = Instant::now();
    let executor = external_lint_lint_arwaky::ExternalLintExecutor::new(
        Arc::new(MockCmdExecutor),
        Arc::new(MockFilesystem::new()),
        Arc::new(MockFilesystem::new()),
    );
    let path = FilePath::new("/tmp".to_string()).unwrap();
    let _ = executor.exec_cmd_adapter(
        vec!["echo".into()],
        path.clone(),
        1.0,
        AdapterName::raw("test"),
    );
    let _ = executor.exec_cmd_scan(vec!["echo".into()], path.clone(), 1.0, None, &path);
    assert!(start.elapsed() < SMOKE_TIMEOUT);
}
