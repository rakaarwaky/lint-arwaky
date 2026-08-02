// PURPOSE: PyRuffAdapter — ILinterAdapterProtocol implementation for Ruff linter integration
//
// Executes `ruff check --output-format=json` as a subprocess and parses
// the JSON output. Ruff outputs a JSON array of diagnostics with file paths,
// line numbers, severity levels, and rule codes.
//
// Key handling:
//   - Falls back to parent directory if target is a file (Ruff requires a directory)
//   - Searches for pyproject.toml to determine the correct working directory
//   - Maps Ruff severity levels (error/warning/info) to AES severity
//   - Converts relative Ruff paths to absolute project paths

use async_trait::async_trait;
use serde_json::Value;
use shared::cli_commands::{LintResult, LintResultList};

use shared::code_analysis::{ILinterAdapterProtocol, LinterOperationError};

use shared::common::{AdapterError, FilePath, Severity};

use shared::common::{
    AdapterName, ColumnNumber, ComplianceStatus, ErrorCode, ErrorMessage, LineNumber, LintMessage,
    LocationList,
};
use shared::external_lint::IExternalLintExecutorProtocol;

use std::sync::Arc;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct RuffAdapter {
        pub filesystem: Arc<dyn IFilesystemAggregate>,
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
    bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ILinterAdapterProtocol for RuffAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("ruff")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        // Skip if no Python files exist in the target path
        if !self.filesystem.has_python_files_recursive(path) {
            return Ok(LintResultList::new(vec![]));
        }

        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "check".to_string(),
            path.value.clone(),
            "--exclude".to_string(),
            "tests".to_string(),
            "--output-format=json".to_string(),
            "--exit-zero".to_string(),
            "--no-cache".to_string(),
        ];
        let working_dir = self.filesystem.default_working_dir(path);

        let response = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 60.0, self.name())
            .await?;

        let stdout = &response.stdout;
        // Empty output — tool found nothing to report (or no applicable files)
        if stdout.trim().is_empty() {
            return Ok(LintResultList::new(vec![]));
        }
        let findings: Vec<Value> = match serde_json::from_str(stdout) {
            Ok(v) => v,
            Err(e) => {
                return Err(LinterOperationError::Adapter(AdapterError::new(
                    self.name(),
                    ErrorMessage::new(format!(
                        "Failed to parse ruff JSON output: {}. Output was: {:?}",
                        e,
                        stdout.chars().take(200).collect::<String>()
                    )),
                )));
            }
        };
        let mut results = Vec::new();

        for f in findings {
            let filename = f
                .get("filename")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let row = f
                .get("location")
                .and_then(|l| l.get("row"))
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let col = f
                .get("location")
                .and_then(|l| l.get("column"))
                .and_then(|v| v.as_i64())
                .unwrap_or_default();
            let code = f.get("code").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
            let message = f
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let severity_str = f
                .get("severity")
                .and_then(|v| v.as_str())
                .unwrap_or_default();

            let resolved = shared::common::utility_path_normalization::resolve_capabilities_path(
                match FilePath::new(filename.to_string()) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                },
                Some(path.clone()),
            );

            results.push(LintResult {
                file: resolved,
                line: LineNumber::new(row),
                column: ColumnNumber::new(col),
                code: ErrorCode::raw(code),
                message: LintMessage::new(message),
                source: Some(self.name()),
                severity: self.map_severity(severity_str, code),
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }
        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "check".to_string(),
            path.value.clone(),
            "--fix".to_string(),
            "--exit-zero".to_string(),
        ];
        let working_dir = self.filesystem.default_working_dir(path);

        let _ = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 60.0, self.name())
            .await?;
        Ok(ComplianceStatus::new(true))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl RuffAdapter {
    pub fn new(
        lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
        bin_path: Option<FilePath>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            lint_executor,
            bin_path,
            filesystem,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value.clone(),
            None => "ruff".to_string(),
        }
    }

    fn map_severity(&self, _severity: &str, code: &str) -> Severity {
        // FR-004: Ruff severity mapping is code-based, not tool-severity-based.
        // Code format: e.g., "E501", "F401", "S105", "B006"
        if code == "E999" || code.starts_with('S') {
            Severity::CRITICAL // syntax error (E999) or security rules (S1xx)
        } else if code == "F401" {
            Severity::MEDIUM // unused import
        } else if (code.starts_with('F')
            && code.len() >= 3
            && code[1..]
                .parse::<u32>()
                .is_ok_and(|n| (800..900).contains(&n)))
            || (code.starts_with('B')
                && code.len() >= 3
                && code[1..]
                    .parse::<u32>()
                    .is_ok_and(|n| (1..100).contains(&n)))
        {
            Severity::HIGH // F8xx: undefined name, B0xx: bugbear
        } else if (code.starts_with('E')
            && code.len() >= 3
            && code[1..]
                .parse::<u32>()
                .is_ok_and(|n| (100..200).contains(&n) || (500..600).contains(&n)))
            || (code.starts_with('W')
                && code.len() >= 3
                && code[1..]
                    .parse::<u32>()
                    .is_ok_and(|n| (200..300).contains(&n)))
        {
            Severity::LOW // E1xx: indentation, E5xx: line length, W2xx: whitespace
        } else {
            Severity::MEDIUM // default
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RuffAdapter;
    use shared::code_analysis::LinterOperationError;
    use shared::common::{AdapterName, ComplianceStatus, FilePath, ResponseData, Severity};
    use shared::external_lint::IExternalLintExecutorProtocol;
    use std::sync::Arc;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

    fn make_adapter() -> RuffAdapter {
        let executor: Arc<dyn IExternalLintExecutorProtocol> = Arc::new(EmptyLintExecutor);
        RuffAdapter::new(executor, None)
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

    // ─── FRD-004: Ruff severity mapping per code ───

    #[test]
    fn e999_syntax_error_maps_to_critical() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("error", "E999"), Severity::CRITICAL);
    }

    #[test]
    fn security_rules_map_to_critical() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "S105"), Severity::CRITICAL);
        assert_eq!(adapter.map_severity("warning", "S602"), Severity::CRITICAL);
        assert_eq!(adapter.map_severity("error", "S101"), Severity::CRITICAL);
    }

    #[test]
    fn f8xx_undefined_name_maps_to_high() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "F821"), Severity::HIGH);
        assert_eq!(adapter.map_severity("warning", "F811"), Severity::HIGH);
    }

    #[test]
    fn b0xx_bugbear_maps_to_high() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "B006"), Severity::HIGH);
        assert_eq!(adapter.map_severity("warning", "B007"), Severity::HIGH);
    }

    #[test]
    fn f401_unused_import_maps_to_medium() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "F401"), Severity::MEDIUM);
    }

    #[test]
    fn e1xx_indentation_maps_to_low() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "E111"), Severity::LOW);
        assert_eq!(adapter.map_severity("warning", "E117"), Severity::LOW);
    }

    #[test]
    fn e5xx_line_length_maps_to_low() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "E501"), Severity::LOW);
    }

    #[test]
    fn w2xx_whitespace_maps_to_low() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "W291"), Severity::LOW);
        assert_eq!(adapter.map_severity("warning", "W292"), Severity::LOW);
    }

    #[test]
    fn unknown_code_defaults_to_medium() {
        let adapter = make_adapter();
        assert_eq!(adapter.map_severity("warning", "C999"), Severity::MEDIUM);
        assert_eq!(adapter.map_severity("error", "XXXX"), Severity::MEDIUM);
    }
}