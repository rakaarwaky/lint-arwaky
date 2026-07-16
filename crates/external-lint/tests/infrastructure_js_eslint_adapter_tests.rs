use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use std::sync::Arc;

use async_trait::async_trait;
use external_lint_lint_arwaky::infrastructure_js_eslint_adapter::ESLintAdapter;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_common_vo::{BooleanVO, ErrorMessage, PatternList};
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_utility_port::IExternalLintUtilityPort;

struct MockESLintExecutor {
    output: String,
}

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
            returncode: 0,
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
        _working_dir: &FilePath,
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
        executor: &dyn ICommandExecutorPort,
        args: PatternList,
        working_dir: FilePath,
        _timeout_secs: Timeout,
        _adapter_name: Option<shared::common::taxonomy_adapter_name_vo::AdapterName>,
        _path: &FilePath,
    ) -> Result<ResponseData, shared::code_analysis::taxonomy_operation_error::LinterOperationError>
    {
        executor
            .execute_command(args, working_dir, None)
            .await
            .map_err(|e| {
                use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
                LinterOperationError::Scan(shared::common::taxonomy_adapter_error::ScanError {
                    path: FilePath::new("unknown".to_string()).unwrap_or_default(),
                    message: ErrorMessage::new(e.to_string()),
                    error_code: None,
                    adapter_name: None,
                    cause: None,
                })
            })
    }
    async fn exec_cmd_adapter(
        &self,
        executor: &dyn ICommandExecutorPort,
        args: PatternList,
        working_dir: FilePath,
        _timeout_secs: Timeout,
        _adapter_name: shared::common::taxonomy_adapter_name_vo::AdapterName,
    ) -> Result<ResponseData, shared::code_analysis::taxonomy_operation_error::LinterOperationError>
    {
        executor
            .execute_command(args, working_dir, None)
            .await
            .map_err(|e| {
                use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
                LinterOperationError::Scan(shared::common::taxonomy_adapter_error::ScanError {
                    path: FilePath::new("unknown".to_string()).unwrap_or_default(),
                    message: ErrorMessage::new(e.to_string()),
                    error_code: None,
                    adapter_name: None,
                    cause: None,
                })
            })
    }
    async fn js_apply_fix(
        &self,
        executor: &dyn ICommandExecutorPort,
        path: &FilePath,
        tool: &str,
        fix_arg: &str,
    ) -> Result<
        ComplianceStatus,
        shared::code_analysis::taxonomy_operation_error::LinterOperationError,
    > {
        let _ = (executor, path, tool, fix_arg);
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
    fn resolve_infrastructure_path(&self, path: FilePath, _: Option<FilePath>) -> FilePath {
        path
    }
}

fn make_adapter(output: &str) -> ESLintAdapter {
    ESLintAdapter::new(
        Arc::new(MockESLintExecutor {
            output: output.to_string(),
        }),
        Arc::new(IdentityPathNorm),
        Arc::new(MockExternalLintUtilityPort),
    )
}

fn make_path(p: &str) -> FilePath {
    FilePath::new(p.to_string()).unwrap_or_default()
}

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
    assert_eq!(
        results.values[0].severity.clone() as i32,
        Severity::HIGH as i32
    );
    assert_eq!(results.values[0].code.code(), "no-unused-vars");
    assert_eq!(results.values[0].line.value(), 10);

    // severity 1 -> MEDIUM
    assert_eq!(
        results.values[1].severity.clone() as i32,
        Severity::MEDIUM as i32
    );
    assert_eq!(results.values[1].code.code(), "semi");
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
    assert_eq!(results.values[0].code.code(), "ESLINT");
}
