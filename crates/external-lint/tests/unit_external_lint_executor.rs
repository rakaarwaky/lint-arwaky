// PURPOSE: Unit tests for ExternalLintExecutor — command execution with error mapping.

use external_lint_lint_arwaky::ExternalLintExecutor;
use shared::common::{
    AdapterName, FilePath, ICommandExecutorProtocol, PatternList, ResponseData, Timeout,
};

use shared::external_lint::IExternalLintExecutorProtocol;
use std::sync::Arc;

// ─── Mock Executor ────────────────────────────────────────

struct MockExecutor {
    response: ResponseData,
}

#[async_trait::async_trait]
impl ICommandExecutorProtocol for MockExecutor {
    async fn execute_command(
        &self,
        _command: PatternList,
        _working_dir: FilePath,
        _timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        Ok(self.response.clone())
    }

    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}

struct FailingExecutor;

#[async_trait::async_trait]
impl ICommandExecutorProtocol for FailingExecutor {
    async fn execute_command(
        &self,
        _command: PatternList,
        _working_dir: FilePath,
        _timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        anyhow::bail!("Command not found: os error 2")
    }

    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}

fn make_executor(response: ResponseData) -> ExternalLintExecutor {
    ExternalLintExecutor::new(Arc::new(MockExecutor { response }), Arc::new(filesystem::FilesystemOrchestrator::new()), Arc::new(filesystem::FilesystemOrchestrator::new()))
}

// ─── exec_cmd_scan ────────────────────────────────────────

#[tokio::test]
async fn exec_cmd_scan_returns_response_on_success() {
    let mut resp = ResponseData::new();
    resp.stdout = "lint output".to_string();
    resp.returncode = 0;

    let executor = make_executor(resp);
    let path = FilePath::new("/tmp/test.rs".to_string()).unwrap();

    let result = executor
        .exec_cmd_scan(
            vec!["cargo".to_string(), "clippy".to_string()],
            FilePath::new("/tmp".to_string()).unwrap(),
            60.0,
            Some(AdapterName::raw("clippy")),
            &path,
        )
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().stdout, "lint output");
}

#[tokio::test]
async fn exec_cmd_scan_maps_error_to_scan_error() {
    let executor = ExternalLintExecutor::new(Arc::new(FailingExecutor), Arc::new(filesystem::FilesystemOrchestrator::new()), Arc::new(filesystem::FilesystemOrchestrator::new()));
    let path = FilePath::new("/tmp/test.rs".to_string()).unwrap();

    let result = executor
        .exec_cmd_scan(
            vec!["missing-tool".to_string()],
            FilePath::new("/tmp".to_string()).unwrap(),
            60.0,
            Some(AdapterName::raw("test")),
            &path,
        )
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Scan error"));
}

// ─── exec_cmd_adapter ─────────────────────────────────────

#[tokio::test]
async fn exec_cmd_adapter_returns_response_on_success() {
    let mut resp = ResponseData::new();
    resp.stdout = "[]".to_string();
    resp.returncode = 0;

    let executor = make_executor(resp);

    let result = executor
        .exec_cmd_adapter(
            vec!["ruff".to_string(), "check".to_string()],
            FilePath::new("/tmp".to_string()).unwrap(),
            60.0,
            AdapterName::raw("ruff"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn exec_cmd_adapter_maps_error_to_adapter_error() {
    let executor = ExternalLintExecutor::new(Arc::new(FailingExecutor), Arc::new(filesystem::FilesystemOrchestrator::new()), Arc::new(filesystem::FilesystemOrchestrator::new()));

    let result = executor
        .exec_cmd_adapter(
            vec!["missing".to_string()],
            FilePath::new("/tmp".to_string()).unwrap(),
            60.0,
            AdapterName::raw("ruff"),
        )
        .await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Adapter error"));
}

// ─── Constructor ──────────────────────────────────────────

#[test]
fn constructor_accepts_arc_executor() {
    let executor: Arc<dyn ICommandExecutorProtocol> = Arc::new(MockExecutor {
        response: ResponseData::new(),
    });
    let lint_executor = ExternalLintExecutor::new(executor), Arc::new(filesystem::FilesystemOrchestrator::new()), Arc::new(filesystem::FilesystemOrchestrator::new());
    let _ = lint_executor;
}
