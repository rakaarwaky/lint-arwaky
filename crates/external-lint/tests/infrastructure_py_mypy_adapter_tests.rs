use std::sync::Arc;

use async_trait::async_trait;
use external_lint_lint_arwaky::infrastructure_py_mypy_adapter::MyPyAdapter;
use shared_lint_arwaky::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared_lint_arwaky::common::contract_path_normalization_port::IPathNormalizationPort;
use shared_lint_arwaky::common::taxonomy_common_vo::PatternList;
use shared_lint_arwaky::common::taxonomy_duration_vo::Timeout;
use shared_lint_arwaky::common::taxonomy_path_vo::FilePath;
use shared_lint_arwaky::common::taxonomy_response_data_vo::ResponseData;
use shared_lint_arwaky::common::taxonomy_severity_vo::Severity;

struct MockMyPyExecutor { output: String }

#[async_trait]
impl ICommandExecutorPort for MockMyPyExecutor {
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
            returncode: 1,
            metadata: meta,
        })
    }
    async fn health_check(&self) -> anyhow::Result<ResponseData> { Ok(ResponseData::new()) }
}

struct IdentityPathNorm;
impl IPathNormalizationPort for IdentityPathNorm {
    fn normalize_path(&self, path: FilePath) -> FilePath { path }
    fn resolve_infrastructure_path(&self, path: FilePath, _: Option<FilePath>) -> FilePath { path }
}

fn make_adapter(output: &str) -> MyPyAdapter {
    MyPyAdapter::new(
        Arc::new(MockMyPyExecutor { output: output.to_string() }),
        Arc::new(IdentityPathNorm),
        None,
    )
}

fn make_path(p: &str) -> FilePath { FilePath::new(p.to_string()).unwrap_or_default() }

#[tokio::test]
async fn parses_mypy_output_with_column() {
    let output = "src/main.py:10:5: error: Incompatible return type [return-value]\n";
    let adapter = make_adapter(output);
    let path = make_path("src/main.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].line.value(), 10);
    assert_eq!(results.values[0].column.value(), 5);
    assert_eq!(results.values[0].code.value(), "return-value");
    assert_eq!(results.values[0].severity as i32, Severity::HIGH as i32);
}

#[tokio::test]
async fn parses_mypy_output_without_column() {
    let output = "src/main.py:10: error: Incompatible return type [return-value]\n";
    let adapter = make_adapter(output);
    let path = make_path("src/main.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].line.value(), 10);
    assert_eq!(results.values[0].column.value(), 0);
    assert_eq!(results.values[0].code.value(), "return-value");
}

#[tokio::test]
async fn parses_multiple_mypy_findings() {
    let output = "\
src/main.py:10: error: Incompatible return type [return-value]
src/utils.py:25:7: warning: unused function [unused-function]
src/main.py:42: note: revealed type is 'int' [note]
";
    let adapter = make_adapter(output);
    let path = make_path(".");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 3);
    // error -> HIGH
    assert_eq!(results.values[0].severity as i32, Severity::HIGH as i32);
    // warning -> MEDIUM
    assert_eq!(results.values[1].severity as i32, Severity::MEDIUM as i32);
    // note -> LOW
    assert_eq!(results.values[2].severity as i32, Severity::LOW as i32);
}

#[tokio::test]
async fn syntax_errors_get_critical_severity() {
    let output = "src/main.py:5: error: syntax error in function definition [syntax]\n";
    let adapter = make_adapter(output);
    let path = make_path("src/main.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.values[0].severity as i32, Severity::CRITICAL as i32);
}

#[tokio::test]
async fn empty_output_returns_empty() {
    let adapter = make_adapter("");
    let path = make_path("src/main.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn apply_fix_returns_noop() {
    let adapter = make_adapter("");
    let path = make_path("test.py");
    let status = adapter.apply_fix(&path).await.unwrap();
    assert!(!status.value());
}
