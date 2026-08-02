// PURPOSE: PyMypyAdapter — ILinterAdapterProtocol implementation for MyPy type checker integration
//
// Runs `mypy <path>` on Python files and parses its structured output with
// two regex patterns (with/without column numbers). Severity is mapped
// heuristically: notes → LOW, warnings → MEDIUM, errors → HIGH,
// syntax/parse errors → CRITICAL.
//
// Key details:
//   - `--no-error-summary` avoids summary lines, keeping output parseable
//   - `--pretty false` ensures machine-parseable single-line output
//   - Falls back to column-less regex if column-full regex doesn't match
//   - apply_fix always returns false (mypy is a type checker, not a formatter)

use regex::Regex;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_lint_vo::LocationList;
use shared::common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_path_normalization::resolve_capabilities_path;
use shared::external_lint::IExternalLintExecutorProtocol;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::quality_rules::LinterOperationError;
use shared::quality_rules::contract_adapter_protocol::ILinterAdapterProtocol;
use std::sync::Arc;
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MyPyAdapter {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
    bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for MyPyAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("mypy")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        // Skip if no Python files exist in the target path
        if !self.filesystem.is_python_file_recursive(path) {
            return Ok(LintResultList::new(vec![]));
        }

        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            path.value().to_string(),
            "--exclude".to_string(),
            "tests".to_string(),
            "--no-error-summary".to_string(),
            "--pretty".to_string(),
            "false".to_string(),
        ];
        let working_dir = self.filesystem.default_working_dir(path);

        let response = self
            .lint_executor
            .exec_cmd_adapter(cmd, working_dir, 120.0, self.name())
            .map_err(crate::convert_executor_error)?;

        let stdout = &response.stdout;
        let re = match mypy_re_with_col() {
            Some(r) => r,
            None => match mypy_re_without_col() {
                Some(r) => r,
                None => return Ok(LintResultList::new(vec![])),
            },
        };
        let re_simple = match mypy_re_without_col() {
            Some(r) => r,
            None => return Ok(LintResultList::new(vec![])),
        };
        let mut results = Vec::new();

        for line in stdout.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(caps) = re.captures(line) {
                let filename = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let line_number: i64 = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let column: i64 = caps
                    .get(3)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let msg_type = caps.get(4).map(|m| m.as_str()).unwrap_or("error");
                let message = caps.get(5).map(|m| m.as_str()).unwrap_or("");
                let code = caps.get(6).map(|m| m.as_str()).unwrap_or("");

                let resolved = resolve_capabilities_path(
                    match FilePath::new(filename.to_string()) {
                        Ok(fp) => fp,
                        Err(_) => path.clone(),
                    },
                    Some(path.clone()),
                );

                results.push(LintResult {
                    file: resolved,
                    line: LineNumber::new(line_number),
                    column: ColumnNumber::new(column),
                    code: ErrorCode::raw(code),
                    message: LintMessage::new(message),
                    source: Some(self.name()),
                    severity: Self::map_severity(msg_type, message),
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            } else if let Some(caps) = re_simple.captures(line) {
                let filename = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let line_number: i64 = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let msg_type = caps.get(3).map(|m| m.as_str()).unwrap_or("error");
                let message = caps.get(4).map(|m| m.as_str()).unwrap_or("");
                let code = caps.get(5).map(|m| m.as_str()).unwrap_or("");

                let resolved = resolve_capabilities_path(
                    match FilePath::new(filename.to_string()) {
                        Ok(fp) => fp,
                        Err(_) => path.clone(),
                    },
                    Some(path.clone()),
                );

                results.push(LintResult {
                    file: resolved,
                    line: LineNumber::new(line_number),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::raw(code),
                    message: LintMessage::new(message),
                    source: Some(self.name()),
                    severity: Self::map_severity(msg_type, message),
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }
        }
        Ok(LintResultList::new(results))
    }

    fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

fn mypy_re_with_col() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^:]+):(\d+):(\d+):\s+(\w+):\s+(.+?)\s+\[([\w-]+)\]$").ok())
        .as_ref()
}

fn mypy_re_without_col() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^:]+):(\d+):\s+(\w+):\s+(.+?)\s+\[([\w-]+)\]$").ok())
        .as_ref()
}

impl MyPyAdapter {
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
            Some(p) => p.value().to_string(),
            None => "mypy".to_string(),
        }
    }

    fn map_severity(msg_type: &str, msg: &str) -> Severity {
        let m = msg.to_lowercase();
        if msg_type == "note" {
            return Severity::LOW;
        }
        if m.contains("syntax") || m.contains("parse") {
            return Severity::CRITICAL;
        }
        if msg_type == "warning" {
            return Severity::MEDIUM;
        }
        Severity::HIGH
    }
}
