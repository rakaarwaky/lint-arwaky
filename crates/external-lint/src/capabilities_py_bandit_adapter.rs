// PURPOSE: PyBanditAdapter — ILinterAdapterProtocol implementation for Bandit security scanner integration
//
// Runs `bandit -r <path> --format json --exit-zero` to scan Python files for
// security vulnerabilities. Parses JSON output to extract findings (filename,
// line_range, test_id, issue_text, severity).
//
// Key details:
//   - `--exit-zero` ensures bandit always exits 0 regardless of findings
//   - JSON output avoids fragile regex parsing
//   - Severity is directly mapped: HIGH→HIGH, MEDIUM→MEDIUM, LOW→LOW
//   - apply_fix always returns false (Bandit is a scanner, not a fixer)

use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

use shared::cli_commands::contract_executor_protocol::ICommandExecutorProtocol;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;

use shared::external_lint::taxonomy_external_lint_helper::{
    default_working_dir, exec_cmd_adapter, has_python_files, noop_apply_fix,
};

pub struct BanditAdapter {
    executor: Arc<dyn ICommandExecutorProtocol>,
    bin_path: Option<FilePath>,
}

impl BanditAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorProtocol>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            executor,
            bin_path,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value.clone(),
            None => "bandit".to_string(),
        }
    }

    fn map_severity(&self, severity: &str) -> Severity {
        match severity {
            "HIGH" => Severity::HIGH,
            "MEDIUM" => Severity::MEDIUM,
            "LOW" => Severity::LOW,
            _ => Severity::MEDIUM,
        }
    }
}

#[async_trait]
impl ILinterAdapterProtocol for BanditAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("bandit")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        // Skip if no Python files exist in the target path
        if !has_python_files(path) {
            return Ok(LintResultList::new(vec![]));
        }

        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "-r".to_string(),
            path.value.clone(),
            "--format".to_string(),
            "json".to_string(),
            "--exit-zero".to_string(),
        ];
        let working_dir = default_working_dir(path);

        let response =
            exec_cmd_adapter(self.executor.as_ref(), cmd, working_dir, 120.0, self.name()).await?;

        let stdout = &response.stdout;
        let parsed: Value = match serde_json::from_str(stdout) {
            Ok(v) => v,
            Err(_) => Value::Object(serde_json::Map::new()),
        };
        let findings = match parsed.get("results").and_then(|v| v.as_array()) {
            Some(arr) => arr.clone(),
            None => Vec::new(),
        };
        let mut results = Vec::new();

        for f in findings {
            let filename = f
                .get("filename")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let line_number = f
                .get("line_number")
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let line_range = f
                .get("line_range")
                .and_then(|v| v.as_array())
                .and_then(|a| a.first())
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let test_id = f.get("test_id").and_then(|v| v.as_str()).unwrap_or("B000");
            let issue_text = f
                .get("issue_text")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let issue_severity = f
                .get("issue_severity")
                .and_then(|v| v.as_str())
                .unwrap_or("MEDIUM");

            let resolved = shared::common::utility_path_normalization::resolve_capabilities_path(
                match FilePath::new(filename.to_string()) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                },
                Some(path.clone()),
            );

            results.push(LintResult {
                file: resolved,
                line: LineNumber::new(line_number),
                column: ColumnNumber::new(line_range),
                code: ErrorCode::raw(test_id),
                message: LintMessage::new(issue_text),
                source: Some(self.name()),
                severity: self.map_severity(issue_severity),
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }
        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        noop_apply_fix().await
    }
}
