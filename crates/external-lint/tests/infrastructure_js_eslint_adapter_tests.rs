use std::sync::Arc;

use async_trait::async_trait;
use external_lint_lint_arwaky::infrastructure_js_eslint_adapter::ESLintAdapter;
use shared_lint_arwaky::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared_lint_arwaky::common::contract_path_normalization_port::IPathNormalizationPort;
use shared_lint_arwaky::common::taxonomy_common_vo::PatternList;
use shared_lint_arwaky::common::taxonomy_duration_vo::Timeout;
use shared_lint_arwaky::common::taxonomy_path_vo::FilePath;
use shared_lint_arwaky::common::taxonomy_response_data_vo::ResponseData;
use shared_lint_arwaky::common::taxonomy_severity_vo::Severity;

struct MockESLintExecutor { output: String }

#[async_trait]
impl ICommandExecutorPort for MockESLintExecutor {
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

fn make_adapter(output: &str) -> ESLintAdapter {
    ESLintAdapter::new(
        Arc::new(MockESLintExecutor { output: output.to_string() }),
        Arc::new(IdentityPathNorm),
    )
}

fn make_path(p: &str) -> FilePath { FilePath::new(p.to_string()).unwrap_or_default() }

#[tokio::test]
async fn parses_eslint_json_output() {
    let json = r#"[
        {
            "filePath":"/project/src/app.ts",
            "messages":[
                {"line":10,"column":5,"ruleId":"no-unused-vars","message":"'x' is assigned but never used","severity":2},
                {"line":15,"column":1,"ruleId":"semi","message":"Missing semicolon","severity":1}
            ]
        }
    ]"#;
    let adapter = make_adapter(json);
    let path = make_path("/project/src/app.ts");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 2);

    // severity 2 -> HIGH
    assert_eq!(results.values[0].severity as i32, Severity::HIGH as i32);
    assert_eq!(results.values[0].code.value(), "no-unused-vars");
    assert_eq!(results.values[0].line.value(), 10);

    // severity 1 -> MEDIUM
    assert_eq!(results.values[1].severity as i32, Severity::MEDIUM as i32);
    assert_eq!(results.values[1].code.value(), "semi");
}

#[tokio::test]
async fn empty_messages_returns_empty() {
    let json = r#"[{"filePath":"/project/src/app.ts","messages":[]}]"#;
    let adapter = make_adapter(json);
    let path = make_path("/project/src/app.ts");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn empty_json_array_returns_empty() {
    let adapter = make_adapter("[]");
    let path = make_path("/project/src/app.ts");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn skips_non_ts_js_files() {
    let adapter = make_adapter("[]");
    let path = make_path("test.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn malformed_json_returns_error() {
    let adapter = make_adapter("not json");
    let path = make_path("test.ts");
    let result = adapter.scan(&path).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn fallback_rule_id_for_missing_rule() {
    let json = r#"[{"filePath":"/project/src/app.ts","messages":[{"line":1,"column":1,"ruleId":null,"message":"test","severity":2}]}]"#;
    let adapter = make_adapter(json);
    let path = make_path("test.ts");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.values[0].code.value(), "ESLINT");
}
