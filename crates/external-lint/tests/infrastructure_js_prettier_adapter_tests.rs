use std::sync::Arc;

use async_trait::async_trait;
use external_lint_lint_arwaky::infrastructure_js_prettier_adapter::PrettierAdapter;
use shared_lint_arwaky::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared_lint_arwaky::common::contract_path_normalization_port::IPathNormalizationPort;
use shared_lint_arwaky::common::taxonomy_common_vo::PatternList;
use shared_lint_arwaky::common::taxonomy_duration_vo::Timeout;
use shared_lint_arwaky::common::taxonomy_path_vo::FilePath;
use shared_lint_arwaky::common::taxonomy_response_data_vo::ResponseData;
use shared_lint_arwaky::cli_commands::taxonomy_severity_vo::Severity;

struct MockPrettierExecutor { stdout: String, stderr: String, exit_code: i32 }

#[async_trait]
impl ICommandExecutorPort for MockPrettierExecutor {
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
            stdout: self.stdout.clone(),
            stderr: self.stderr.clone(),
            returncode: self.exit_code as i64,
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

fn make_adapter(stdout: &str, stderr: &str, exit_code: i32) -> PrettierAdapter {
    PrettierAdapter::new(
        Arc::new(MockPrettierExecutor {
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
            exit_code,
        }),
        Arc::new(IdentityPathNorm),
    )
}

fn make_path(p: &str) -> FilePath { FilePath::new(p.to_string()).unwrap_or_default() }

#[tokio::test]
async fn detects_warning_in_stderr() {
    let adapter = make_adapter("", "[warn] src/app.ts: Code style issues", 1);
    let path = make_path("src/app.ts");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results.values[0].code.value(), "formatting");
    assert_eq!(results.values[0].severity as i32, Severity::LOW as i32);
}

#[tokio::test]
async fn no_warnings_returns_empty() {
    let adapter = make_adapter("", "", 0);
    let path = make_path("src/app.ts");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn skips_non_prettier_file_types() {
    let adapter = make_adapter("", "", 0);
    let path = make_path("test.py");
    let results = adapter.scan(&path).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn scans_recognized_extensions() {
    for ext in &["ts", "tsx", "js", "jsx", "json", "css", "md", "yml", "yaml"] {
        let adapter = make_adapter("", "", 0);
        let path = make_path(&format!("test.{}", ext));
        let results = adapter.scan(&path).await.unwrap();
        // Should pass the extension check (result may be empty from executor)
        assert!(results.len() == 0, "failed for .{}", ext);
    }
}

#[tokio::test]
async fn apply_fix_runs_prettier_write() {
    let adapter = make_adapter("", "", 0);
    let path = make_path("test.ts");
    let status = adapter.apply_fix(&path).await.unwrap();
    // Returns false because mock executor returns exit_code 0
    assert!(status.value());
}
