use external_lint_lint_arwaky::infrastructure_rs_audit_adapter::CargoAuditAdapter;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_common_vo::{BooleanVO, PatternList};
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_utility_port::IExternalLintUtilityPort;
use std::sync::Arc;

use async_trait::async_trait;

struct MockExternalLintUtilityPort;

#[async_trait]
impl IExternalLintUtilityPort for MockExternalLintUtilityPort {
    fn canonicalize_path(&self, path_str: &str) -> FilePath {
        FilePath::new(path_str.to_string()).unwrap_or_default()
    }
    fn default_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    fn has_python_files(&self, _path: &FilePath) -> BooleanVO {
        BooleanVO::new(true)
    }
    fn has_py_in_dir(&self, _dir: &DirectoryPath) -> BooleanVO {
        BooleanVO::new(true)
    }
    fn is_in_path(&self, _executable: &str) -> BooleanVO {
        BooleanVO::new(true)
    }
    fn resolve_js_cmd(
        &self,
        executable: &str,
        args: PatternList,
        working_dir: &FilePath,
    ) -> PatternList {
        let mut cmd = vec![executable.to_string()];
        cmd.extend(args.values);
        PatternList::new(cmd)
    }
    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
        path.clone()
    }
    async fn exec_cmd_scan(
        &self,
        _executor: &dyn ICommandExecutorPort,
        args: PatternList,
        working_dir: FilePath,
        _timeout_secs: Timeout,
        _adapter_name: Option<shared::common::taxonomy_adapter_name_vo::AdapterName>,
        _path: &FilePath,
    ) -> Result<ResponseData, shared::code_analysis::taxonomy_operation_error::LinterOperationError>
    {
        let mut meta = std::collections::HashMap::new();
        meta.insert("protocol".into(), serde_json::Value::String("Stdio".into()));
        Ok(ResponseData {
            value: None,
            stdout: args.values.join(" "),
            stderr: String::new(),
            returncode: 0,
            metadata: meta,
        })
    }
    async fn exec_cmd_adapter(
        &self,
        _executor: &dyn ICommandExecutorPort,
        args: PatternList,
        working_dir: FilePath,
        _timeout_secs: Timeout,
        _adapter_name: shared::common::taxonomy_adapter_name_vo::AdapterName,
    ) -> Result<ResponseData, shared::code_analysis::taxonomy_operation_error::LinterOperationError>
    {
        let mut meta = std::collections::HashMap::new();
        meta.insert("protocol".into(), serde_json::Value::String("Stdio".into()));
        Ok(ResponseData {
            value: None,
            stdout: args.values.join(" "),
            stderr: String::new(),
            returncode: 0,
            metadata: meta,
        })
    }
    async fn js_apply_fix(
        &self,
        _executor: &dyn ICommandExecutorPort,
        _path: &FilePath,
        _tool: &str,
        _fix_arg: &str,
    ) -> Result<
        ComplianceStatus,
        shared::code_analysis::taxonomy_operation_error::LinterOperationError,
    > {
        Ok(ComplianceStatus::new(true))
    }
    async fn noop_apply_fix(
        &self,
    ) -> Result<
        ComplianceStatus,
        shared::code_analysis::taxonomy_operation_error::LinterOperationError,
    > {
        Ok(ComplianceStatus::new(true))
    }
}

struct IdentityPathNorm;

impl IPathNormalizationPort for IdentityPathNorm {
    fn normalize_path(&self, path: FilePath) -> FilePath {
        path
    }
    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        _context_path: Option<FilePath>,
    ) -> FilePath {
        path
    }
}

fn make_adapter() -> CargoAuditAdapter {
    CargoAuditAdapter::new(
        Arc::new(IdentityPathNorm),
        Arc::new(MockExternalLintUtilityPort),
    )
}

fn make_path(p: &str) -> FilePath {
    FilePath::new(p.to_string()).unwrap_or_default()
}

#[tokio::test]
async fn name_returns_cargo_audit() {
    let adapter = make_adapter();
    assert_eq!(adapter.name().value(), "cargo-audit");
}

#[tokio::test]
async fn scan_returns_empty_when_no_cargo_lock() {
    let adapter = make_adapter();
    let dir = std::env::temp_dir().join(format!("audit_nolock_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    // No Cargo.lock in this directory
    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert!(
        results.is_empty(),
        "expected no results for dir without Cargo.lock"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn scan_returns_empty_for_nonexistent_path() {
    let adapter = make_adapter();
    let path = make_path("/nonexistent/path/xyz_audit_test");
    let results = adapter.scan(&path).await.unwrap();
    assert!(results.is_empty());
}

#[tokio::test]
async fn scan_with_empty_cargo_lock_does_not_panic() {
    let adapter = make_adapter();
    let dir = std::env::temp_dir().join(format!("audit_empty_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    // Create empty Cargo.lock — parser will fail gracefully
    std::fs::write(dir.join("Cargo.lock"), "").unwrap();
    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert!(
        results.is_empty(),
        "expected empty results for empty Cargo.lock"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn apply_fix_returns_true() {
    let adapter = make_adapter();
    let path = make_path("Cargo.lock");
    let status = adapter.apply_fix(&path).await.unwrap();
    assert!(
        status.value(),
        "cargo-audit apply_fix should return true (noop)"
    );
}
