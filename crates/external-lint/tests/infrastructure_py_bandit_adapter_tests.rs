use std::sync::Arc;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;

use async_trait::async_trait;
use external_lint_lint_arwaky::infrastructure_py_bandit_adapter::BanditAdapter;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::cli_commands::taxonomy_severity_vo::Severity;

struct MockBanditExecutor { output: String }

#[async_trait]
impl ICommandExecutorPort for MockBanditExecutor {
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
            returncode: 0,
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

fn make_adapter(output: &str) -> BanditAdapter {
    BanditAdapter::new(
        Arc::new(MockBanditExecutor { output: output.to_string() }),
        Arc::new(IdentityPathNorm),
        None,
    )
}

fn make_path(p: &str) -> FilePath { FilePath::new(p.to_string()).unwrap_or_default() }

#[tokio::test]
async fn parses_bandit_json_results() {
    let json = r#"{
        "results": [
            {"filename":"src/main.py","line_number":42,"line_range":[42],"test_id":"B101","issue_text":"Use of assert detected","issue_severity":"HIGH"},
            {"filename":"src/main.py","line_number":55,"line_range":[55],"test_id":"B301","issue_text":"pickle usage","issue_severity":"MEDIUM"}
        ]
    }"#;
    let adapter = make_adapter(json);
    let path = make_path("src/main.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results.values[0].code.code(), "B101");
    assert_eq!(results.values[0].line.value(), 42);
    assert_eq!(results.values[0].severity.clone() as i32, Severity::HIGH as i32);
    assert_eq!(results.values[1].code.code(), "B301");
    assert_eq!(results.values[1].severity.clone() as i32, Severity::MEDIUM as i32);
}

#[tokio::test]
async fn empty_results_returns_empty() {
    let adapter = make_adapter(r#"{"results":[]}"#);
    let path = make_path("src/main.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn malformed_json_returns_empty() {
    let adapter = make_adapter("not json");
    let path = make_path("src/main.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn maps_bandit_severity_correctly() {
    let json = r#"{
        "results": [
            {"filename":"t.py","line_number":1,"line_range":[1],"test_id":"B001","issue_text":"hi","issue_severity":"HIGH"},
            {"filename":"t.py","line_number":2,"line_range":[2],"test_id":"B002","issue_text":"hi","issue_severity":"MEDIUM"},
            {"filename":"t.py","line_number":3,"line_range":[3],"test_id":"B003","issue_text":"hi","issue_severity":"LOW"},
            {"filename":"t.py","line_number":4,"line_range":[4],"test_id":"B004","issue_text":"hi","issue_severity":"UNKNOWN"}
        ]
    }"#;
    let adapter = make_adapter(json);
    let path = make_path("t.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.values[0].severity.clone() as i32, Severity::HIGH as i32);
    assert_eq!(results.values[1].severity.clone() as i32, Severity::MEDIUM as i32);
    assert_eq!(results.values[2].severity.clone() as i32, Severity::LOW as i32);
    assert_eq!(results.values[3].severity.clone() as i32, Severity::MEDIUM as i32); // default
}

#[tokio::test]
async fn apply_fix_returns_noop() {
    let adapter = make_adapter("{}");
    let path = make_path("test.py");
    let status = adapter.apply_fix(&path).await.unwrap();
    assert!(!status.value()); // bandit can't fix
}
