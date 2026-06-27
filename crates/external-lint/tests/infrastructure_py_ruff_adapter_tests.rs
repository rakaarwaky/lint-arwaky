use std::sync::Arc;

use async_trait::async_trait;
use external_lint_lint_arwaky::infrastructure_py_ruff_adapter::RuffAdapter;
use shared_lint_arwaky::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared_lint_arwaky::common::contract_path_normalization_port::IPathNormalizationPort;
use shared_lint_arwaky::common::taxonomy_common_vo::PatternList;
use shared_lint_arwaky::common::taxonomy_duration_vo::Timeout;
use shared_lint_arwaky::common::taxonomy_path_vo::FilePath;
use shared_lint_arwaky::common::taxonomy_response_data_vo::ResponseData;
use shared_lint_arwaky::common::taxonomy_severity_vo::Severity;

struct MockRuffExecutor {
    output: String,
    exit_code: i32,
}

#[async_trait]
impl ICommandExecutorPort for MockRuffExecutor {
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

struct IdentityPathNorm;

impl IPathNormalizationPort for IdentityPathNorm {
    fn normalize_path(&self, path: FilePath) -> FilePath { path }
    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        _context_path: Option<FilePath>,
    ) -> FilePath { path }
}

fn make_adapter(output: &str) -> RuffAdapter {
    RuffAdapter::new(
        Arc::new(MockRuffExecutor {
            output: output.to_string(),
            exit_code: 0,
        }),
        Arc::new(IdentityPathNorm),
        None,
    )
}

fn make_path(p: &str) -> FilePath {
    FilePath::new(p.to_string()).unwrap_or_default()
}

#[tokio::test]
async fn parses_json_array_of_findings() {
    let json = r#"[
        {"filename":"src/main.py","location":{"row":10,"column":5},"code":"F401","message":"`os` imported but unused","severity":"warning"},
        {"filename":"src/main.py","location":{"row":15,"column":3},"code":"E302","message":"expected 2 blank lines","severity":"error"}
    ]"#;
    let adapter = make_adapter(json);
    let path = make_path("src/main.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 2);

    assert_eq!(results.values[0].code.value(), "F401");
    assert_eq!(results.values[0].line.value(), 10);
    assert_eq!(results.values[0].column.value(), 5);

    assert_eq!(results.values[1].code.value(), "E302");
    assert_eq!(results.values[1].line.value(), 15);
    assert_eq!(results.values[1].severity as i32, Severity::HIGH as i32);

    assert_eq!(results.values[0].source.as_ref().unwrap().value(), "ruff");
}

#[tokio::test]
async fn empty_json_array_returns_empty() {
    let adapter = make_adapter("[]");
    let path = make_path("src/main.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn malformed_json_returns_error() {
    let adapter = make_adapter("this is not json");
    let path = make_path("src/main.py");
    let result = adapter.scan(&path).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn maps_severity_correctly() {
    let json = r#"[
        {"filename":"test.py","location":{"row":1,"column":1},"code":"E001","message":"error","severity":"error"},
        {"filename":"test.py","location":{"row":2,"column":1},"code":"W001","message":"warning","severity":"warning"},
        {"filename":"test.py","location":{"row":3,"column":1},"code":"I001","message":"info","severity":"info"}
    ]"#;
    let adapter = make_adapter(json);
    let path = make_path("test.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.values[0].severity as i32, Severity::HIGH as i32);
    assert_eq!(results.values[1].severity as i32, Severity::MEDIUM as i32);
    assert_eq!(results.values[2].severity as i32, Severity::LOW as i32);
}

#[tokio::test]
async fn apply_fix_runs_ruff_check_fix() {
    let adapter = make_adapter("[]");
    let path = make_path("test.py");
    let status = adapter.apply_fix(&path).await.unwrap();
    assert!(status.value());
}
