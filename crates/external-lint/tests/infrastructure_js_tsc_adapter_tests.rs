use std::sync::Arc;

use async_trait::async_trait;
use external_lint_lint_arwaky::infrastructure_js_tsc_adapter::TSCAdapter;
use shared_lint_arwaky::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared_lint_arwaky::common::contract_path_normalization_port::IPathNormalizationPort;
use shared_lint_arwaky::common::taxonomy_common_vo::PatternList;
use shared_lint_arwaky::common::taxonomy_duration_vo::Timeout;
use shared_lint_arwaky::common::taxonomy_path_vo::FilePath;
use shared_lint_arwaky::common::taxonomy_response_data_vo::ResponseData;
use shared_lint_arwaky::cli_commands::taxonomy_severity_vo::Severity;

struct MockTSCExecutor { output: String }

#[async_trait]
impl ICommandExecutorPort for MockTSCExecutor {
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
            stdout: String::new(),
            stderr: self.output.clone(),
            returncode: 2,
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

fn make_adapter(output: &str) -> TSCAdapter {
    TSCAdapter::new(
        Arc::new(MockTSCExecutor { output: output.to_string() }),
        Arc::new(IdentityPathNorm),
    )
}

fn make_path(p: &str) -> FilePath { FilePath::new(p.to_string()).unwrap_or_default() }

#[tokio::test]
async fn parses_parenthesized_tsc_format() {
    let output = "src/app.ts(10,5): error TS2322: Type 'number' is not assignable to type 'string'.\n";
    let adapter = make_adapter(output);
    let path = make_path("src/app.ts");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].code.value(), "TS2322");
    assert_eq!(results.values[0].line.value(), 10);
    assert_eq!(results.values[0].column.value(), 5);
    assert_eq!(results.values[0].severity as i32, Severity::HIGH as i32);
}

#[tokio::test]
async fn parses_colon_delimited_tsc_format() {
    let output = "src/app.ts:10:5 - error TS2554: Expected 1 arguments, but got 2.\n";
    let adapter = make_adapter(output);
    let path = make_path("src/app.ts");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].code.value(), "TS2554");
    assert_eq!(results.values[0].line.value(), 10);
    assert_eq!(results.values[0].column.value(), 5);
}

#[tokio::test]
async fn parses_multiple_tsc_errors() {
    let output = "\
src/app.ts(10,5): error TS2322: Type mismatch.
src/utils.ts(25,3): error TS7006: Parameter implicitly has 'any' type.
";
    let adapter = make_adapter(output);
    let path = make_path("src/app.ts");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results.values[0].code.value(), "TS2322");
    assert_eq!(results.values[1].code.value(), "TS7006");
}

#[tokio::test]
async fn empty_output_returns_empty() {
    let adapter = make_adapter("");
    let path = make_path("src/app.ts");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn skips_non_ts_tsx_files() {
    let adapter = make_adapter("error");
    let path = make_path("test.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn apply_fix_returns_noop() {
    let adapter = make_adapter("");
    let path = make_path("test.ts");
    let status = adapter.apply_fix(&path).await.unwrap();
    assert!(!status.value());
}
