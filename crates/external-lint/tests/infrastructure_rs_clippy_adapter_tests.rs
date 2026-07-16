use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use std::sync::Arc;

use async_trait::async_trait;
use external_lint_lint_arwaky::infrastructure_rs_clippy_adapter::RustLinterAdapter;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_common_vo::{BooleanVO, PatternList};
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_utility_port::IExternalLintUtilityPort;

// ---------------------------------------------------------------------------
// Mock executor that returns a canned clippy JSON output
// ---------------------------------------------------------------------------
struct MockClippyExecutor {
    output: String,
    stderr: String,
    exit_code: i32,
}

#[async_trait]
impl ICommandExecutorPort for MockClippyExecutor {
    async fn execute_command(
        &self,
        _command: PatternList,
        _working_dir: FilePath,
        _timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        let mut meta = std::collections::HashMap::new();
        meta.insert("protocol".into(), serde_json::Value::String("Stdio".into()));
        Ok(ResponseData {
            value: None,
            stdout: self.output.clone(),
            stderr: self.stderr.clone(),
            returncode: self.exit_code as i64,
            metadata: meta,
        })
    }

    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}

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
        cmd.extend(args.value);
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
            stdout: args.value.join(" "),
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
            stdout: args.value.join(" "),
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

fn make_adapter(output: &str, stderr: &str, exit_code: i32) -> RustLinterAdapter {
    RustLinterAdapter::new(
        Arc::new(MockClippyExecutor {
            output: output.to_string(),
            stderr: stderr.to_string(),
            exit_code,
        }),
        Arc::new(IdentityPathNorm),
        Arc::new(MockExternalLintUtilityPort),
        None,
    )
}

fn make_path(p: &str) -> FilePath {
    FilePath::new(p.to_string()).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn parses_diagnostic_json_line() {
    let json = r#"{"reason":"compiler-message","message":{"level":"error","code":{"code":"clippy::needless_return"},"message":"this `return` is unnecessary","spans":[{"is_primary":true,"file_name":"src/main.rs","line_start":42,"column_start":5}]}}"#;
    let adapter = make_adapter(json, "", 1);
    // Need a Cargo.toml in a parent dir for the adapter to proceed
    let dir = std::env::temp_dir().join(format!("clippy_test_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let cargo_toml = dir.join("Cargo.toml");
    std::fs::write(&cargo_toml, "[package]\n").unwrap();

    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 1);
    let r = &results.values[0];
    assert_eq!(r.code.code(), "clippy::needless_return");
    assert_eq!(r.line.value(), 42);
    assert_eq!(r.column.value(), 5);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn skips_non_compiler_message_lines() {
    // JSON output that doesn't have reason=compiler-message should be skipped
    let json = r#"{"reason":"build-finished","success":true}"#;
    let adapter = make_adapter(json, "", 0);
    let dir = std::env::temp_dir().join(format!("clippy_skip_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let cargo_toml = dir.join("Cargo.toml");
    std::fs::write(&cargo_toml, "[package]\n").unwrap();

    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn falls_back_to_stderr_when_stdout_empty() {
    let stdout = "";
    let stderr = r#"{"reason":"compiler-message","message":{"level":"warning","code":{"code":"clippy::style"},"message":"style issue","spans":[{"is_primary":true,"file_name":"src/lib.rs","line_start":15,"column_start":1}]}}"#;
    let adapter = make_adapter(stdout, stderr, 1);

    let dir = std::env::temp_dir().join(format!("clippy_stderr_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let cargo_toml = dir.join("Cargo.toml");
    std::fs::write(&cargo_toml, "[package]\n").unwrap();

    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].code.code(), "clippy::style");

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn skips_non_primary_spans() {
    let json = r#"{"reason":"compiler-message","message":{"level":"warning","code":{"code":"clippy::type_complexity"},"message":"type too complex","spans":[{"is_primary":false,"file_name":"src/main.rs","line_start":1,"column_start":1},{"is_primary":true,"file_name":"src/main.rs","line_start":10,"column_start":5}]}}"#;
    let adapter = make_adapter(json, "", 1);

    let dir = std::env::temp_dir().join(format!("clippy_span_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let cargo_toml = dir.join("Cargo.toml");
    std::fs::write(&cargo_toml, "[package]\n").unwrap();

    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].line.value(), 10);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn maps_error_severity_to_high() {
    let json = r#"{"reason":"compiler-message","message":{"level":"error","code":{"code":"clippy::errored"},"message":"error","spans":[{"is_primary":true,"file_name":"src/main.rs","line_start":1,"column_start":1}]}}"#;
    let adapter = make_adapter(json, "", 1);

    let dir = std::env::temp_dir().join(format!("clippy_sev_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let cargo_toml = dir.join("Cargo.toml");
    std::fs::write(&cargo_toml, "[package]\n").unwrap();

    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.values[0].severity.clone() as i32, 3); // HIGH

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn maps_warning_severity_to_medium() {
    let json = r#"{"reason":"compiler-message","message":{"level":"warning","code":{"code":"clippy::warned"},"message":"warning","spans":[{"is_primary":true,"file_name":"src/main.rs","line_start":1,"column_start":1}]}}"#;
    let adapter = make_adapter(json, "", 1);

    let dir = std::env::temp_dir().join(format!("clippy_warn_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let cargo_toml = dir.join("Cargo.toml");
    std::fs::write(&cargo_toml, "[package]\n").unwrap();

    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.values[0].severity.clone() as i32, 2); // MEDIUM

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn returns_empty_when_no_cargo_toml() {
    let adapter = make_adapter("", "", 0);
    let dir = std::env::temp_dir().join(format!("clippy_notoml_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    // No Cargo.toml in this dir
    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn ignores_malformed_json_lines() {
    // A JSON parse error should be silently skipped
    let json = "this is not json\n{\"reason\":\"compiler-message\",\"message\":{\"level\":\"warning\",\"code\":{\"code\":\"clippy::ok\"},\"message\":\"ok\",\"spans\":[{\"is_primary\":true,\"file_name\":\"src/main.rs\",\"line_start\":1,\"column_start\":1}]}}";
    let adapter = make_adapter(json, "", 1);

    let dir = std::env::temp_dir().join(format!("clippy_mal_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let cargo_toml = dir.join("Cargo.toml");
    std::fs::write(&cargo_toml, "[package]\n").unwrap();

    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    // Should parse the valid JSON line and skip the non-JSON line
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].code.code(), "clippy::ok");

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn apply_fix_runs_clippy_fix() {
    let adapter = make_adapter("", "", 0);
    let path = make_path("src/main.rs");
    let status = adapter.apply_fix(&path).await.unwrap();
    assert!(status.value());
}
