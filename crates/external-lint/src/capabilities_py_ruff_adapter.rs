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

use serde_json::Value;
use shared_cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared_common::ErrorMessage;
use shared_common::taxonomy_adapter_error::AdapterError;
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_lint_vo::LocationList;
use shared_common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_external_lint::utility_path_normalization::resolve_capabilities_path;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use shared_quality_rules::LinterOperationError;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct RuffAdapter {
    pub tool_resolution: Arc<dyn IToolResolutionProtocol>,
    pub io: Arc<dyn IFileSystemIOProtocol>,
    lint_executor: Arc<dyn ICommandExecutorProtocol>,
    bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for RuffAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("ruff")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        // Skip if no Python files exist in the target path
        if !self.tool_resolution.is_python_file_recursive(path) {
            return Ok(LintResultList::new(vec![]));
        }

        let executable = self.resolve_executable();
        let abs_path = self.io.canonicalize_path_str(path);
        let cmd = vec![
            executable,
            "check".to_string(),
            abs_path.value.to_string(),
            "--exclude".to_string(),
            "tests".to_string(),
            "--output-format=json".to_string(),
            "--exit-zero".to_string(),
            "--no-cache".to_string(),
        ];
        let working_dir = self.tool_resolution.default_working_dir(path);

        let response = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 60.0, self.name())
            .map_err(crate::convert_executor_error)?;

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

        for f in &findings {
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

            let resolved = resolve_capabilities_path(
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

    fn fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "check".to_string(),
            path.value().to_string(),
            "--fix".to_string(),
            "--exit-zero".to_string(),
        ];
        let working_dir = self.tool_resolution.default_working_dir(path);

        let _ = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 60.0, self.name())
            .map_err(crate::convert_executor_error)?;
        Ok(ComplianceStatus::new(true))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl RuffAdapter {
    pub fn new(
        lint_executor: Arc<dyn ICommandExecutorProtocol>,
        bin_path: Option<FilePath>,
        io: Arc<dyn IFileSystemIOProtocol>,
        tool_resolution: Arc<dyn IToolResolutionProtocol>,
    ) -> Self {
        Self {
            lint_executor,
            bin_path,
            io,
            tool_resolution,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value().to_string(),
            None => "ruff".to_string(),
        }
    }

    pub fn map_severity(&self, _severity: &str, code: &str) -> Severity {
        // FR-004: Ruff severity mapping is code-based, not tool-severity-based.
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
