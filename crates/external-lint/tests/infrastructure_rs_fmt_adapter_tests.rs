use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use std::sync::Arc;

use async_trait::async_trait;
use external_lint_lint_arwaky::infrastructure_rs_fmt_adapter::RustFmtAdapter;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_common_vo::{BooleanVO, PatternList};
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_utility_port::IExternalLintUtilityPort;

struct MockFmtExecutor {
    output: String,
    exit_code: i32,
}

#[async_trait]
impl ICommandExecutorPort for MockFmtExecutor {
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
            stderr: String::new(),
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

fn make_adapter(output: &str, exit_code: i32) -> RustFmtAdapter {
    RustFmtAdapter::new(
        Arc::new(MockFmtExecutor {
            output: output.to_string(),
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

fn with_cargo_toml() -> std::path::PathBuf {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("rustfmt_test_{}_{}", std::process::id(), id));
    let _ = std::fs::create_dir_all(&dir);
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"test\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    dir
}

#[tokio::test]
async fn exit_code_zero_returns_empty() {
    let adapter = make_adapter("", 0);
    let dir = with_cargo_toml();
    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn parses_diff_output_for_added_lines() {
    let diff = "Diff in src/main.rs:\n--- old\n+++ new\n@@ -1,3 +1,3 @@\n fn main() {\n-    println!(\"hello\");\n+    println!(\"hello\");\n }";
    let adapter = make_adapter(diff, 1);
    let dir = with_cargo_toml();
    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert!(results.len() >= 1);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn returns_empty_when_no_cargo_toml() {
    let adapter = make_adapter("", 0);
    let dir = std::env::temp_dir().join(format!("rustfmt_notoml_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn returns_empty_when_diff_has_no_added_lines() {
    // When exit code != 0 but no '+' lines are found, adapter returns empty (no false positive)
    let diff = "Diff in src/main.rs:\\n-    removed line\\n context line";
    let adapter = make_adapter(diff, 1);
    let dir = with_cargo_toml();
    let path = make_path(&dir.to_string_lossy());
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn apply_fix_runs_cargo_fmt() {
    let adapter = make_adapter("", 0);
    let path = make_path("src/main.rs");
    let status = adapter.apply_fix(&path).await.unwrap();
    assert!(status.value());
}
