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

use async_trait::async_trait;
use regex::Regex;
use shared::cli_commands::{LintResult, LintResultList};

use shared::code_analysis::{ILinterAdapterProtocol, LinterOperationError};

use shared::common::{FilePath, Severity};

use shared::external_lint::IExternalLintExecutorProtocol;

use shared::common::{
    AdapterName, ColumnNumber, ComplianceStatus, ErrorCode, LineNumber, LintMessage, LocationList,
};

use std::sync::Arc;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MyPyAdapter {
        pub filesystem: Arc<dyn IFilesystemAggregate>,
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
    bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait]
impl ILinterAdapterProtocol for MyPyAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("mypy")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        // Skip if no Python files exist in the target path
        if !self.filesystem.has_python_files_recursive(path) {
            return Ok(LintResultList::new(vec![]));
        }

        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            path.value.clone(),
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
            .await?;

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
                let filename = match caps.get(1) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let line_number: i64 = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let column: i64 = caps
                    .get(3)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let msg_type = match caps.get(4) {
                    Some(m) => m.as_str(),
                    None => "error",
                };
                let message = match caps.get(5) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let code = match caps.get(6) {
                    Some(m) => m.as_str(),
                    None => "",
                };

                let resolved =
                    shared::common::utility_path_normalization::resolve_capabilities_path(
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
                let filename = match caps.get(1) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let line_number: i64 = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or_default();
                let msg_type = match caps.get(3) {
                    Some(m) => m.as_str(),
                    None => "error",
                };
                let message = match caps.get(4) {
                    Some(m) => m.as_str(),
                    None => "",
                };
                let code = match caps.get(5) {
                    Some(m) => m.as_str(),
                    None => "",
                };

                let resolved =
                    shared::common::utility_path_normalization::resolve_capabilities_path(
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

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        self.filesystem.noop_apply_fix().await
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
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value.clone(),
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
