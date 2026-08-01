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
use shared::cli_commands::{LintResult, LintResultList};

use shared::code_analysis::{ILinterAdapterProtocol, LinterOperationError};

use shared::common::{
    AdapterName, ColumnNumber, ComplianceStatus, ErrorCode, FilePath, LineNumber, LintMessage,
    LocationList, Severity,
};

use std::sync::Arc;

use shared::external_lint::IExternalLintExecutorProtocol;
use shared::external_lint::utility_external_lint::{
    default_working_dir, has_python_files, noop_apply_fix,
};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct BanditAdapter {
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
    bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

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
            "--exclude".to_string(),
            "tests".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--exit-zero".to_string(),
        ];
        let working_dir = default_working_dir(path);

        let response = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 120.0, self.name())
            .await?;

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
            let issue_confidence = f
                .get("issue_confidence")
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
                severity: self.map_severity(issue_severity, issue_confidence),
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl BanditAdapter {
    pub fn new(
        lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            lint_executor,
            bin_path,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value.clone(),
            None => "bandit".to_string(),
        }
    }

    fn map_severity(&self, severity: &str, confidence: &str) -> Severity {
        // FR-004: Bandit severity — HIGH confidence + HIGH severity → CRITICAL.
        match (severity, confidence) {
            ("HIGH", "HIGH") => Severity::CRITICAL,
            ("HIGH", _) => Severity::HIGH,
            ("MEDIUM", _) => Severity::MEDIUM,
            ("LOW", _) => Severity::LOW,
            _ => Severity::MEDIUM,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BanditAdapter;
    use shared::code_analysis::LinterOperationError;
    use shared::common::{AdapterName, ComplianceStatus, FilePath, ResponseData, Severity};
    use shared::external_lint::IExternalLintExecutorProtocol;
    use std::sync::Arc;

    fn make_adapter() -> BanditAdapter {
        let executor: Arc<dyn IExternalLintExecutorProtocol> = Arc::new(EmptyLintExecutor);
        BanditAdapter::new(executor, None)
    }

    struct EmptyLintExecutor;
    #[async_trait::async_trait]
    impl IExternalLintExecutorProtocol for EmptyLintExecutor {
        async fn exec_cmd_scan(
            &self,
            _: Vec<String>,
            _: FilePath,
            _: f64,
            _: Option<AdapterName>,
            _: &FilePath,
        ) -> Result<ResponseData, LinterOperationError> {
            Ok(ResponseData::default())
        }
        async fn exec_cmd_adapter(
            &self,
            _: Vec<String>,
            _: FilePath,
            _: f64,
            _: AdapterName,
        ) -> Result<ResponseData, LinterOperationError> {
            Ok(ResponseData::default())
        }
        async fn js_apply_fix(
            &self,
            _: &FilePath,
            _: &str,
            _: &str,
        ) -> Result<ComplianceStatus, LinterOperationError> {
            Ok(ComplianceStatus::new(false))
        }
    }

    // ─── FRD-004: Bandit severity mapping ───

    #[test]
    fn high_confidence_high_severity_maps_to_critical() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("HIGH", "HIGH"), Severity::CRITICAL);
    }

    #[test]
    fn high_severity_low_confidence_maps_to_high() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("HIGH", "LOW"), Severity::HIGH);
        assert_eq!(adapter.map_severity("HIGH", "MEDIUM"), Severity::HIGH);
    }

    #[test]
    fn medium_severity_any_confidence_maps_to_medium() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("MEDIUM", "HIGH"), Severity::MEDIUM);
        assert_eq!(adapter.map_severity("MEDIUM", "LOW"), Severity::MEDIUM);
    }

    #[test]
    fn low_severity_any_confidence_maps_to_low() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("LOW", "HIGH"), Severity::LOW);
        assert_eq!(adapter.map_severity("LOW", "LOW"), Severity::LOW);
    }

    #[test]
    fn unknown_severity_defaults_to_medium() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("UNKNOWN", "HIGH"), Severity::MEDIUM);
    }
}