use std::sync::OnceLock;

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
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use std::path::Path;
use std::sync::Arc;

use shared::external_lint::taxonomy_external_lint_helper::{
    canonicalize_path, exec_cmd_scan, noop_apply_fix, resolve_js_cmd,
    resolve_js_working_dir as resolve_working_dir,
};

// (No protocol implementation found in this file)

// PURPOSE: TSCAdapter — ILinterAdapterProtocol implementation for TypeScript compiler integration
//
// Runs `tsc --noEmit --pretty false <path>` to type-check TypeScript files.
// Parses compiler output with two regex patterns (parenthesized format and
// colon-delimited format). apply_fix always returns false (tsc is a compiler).
//
// Key details:
//   - `--noEmit` prevents output files, only runs type checking
//   - `--pretty false` ensures machine-parseable output
//   - Two regex patterns handle different tsc output formats across versions
//   - Skips files that don't end in .ts or .tsx
//   - All tsc errors are reported as HIGH severity

use regex::Regex;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct TSCAdapter {
    executor: Arc<dyn ICommandExecutorProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl ILinterAdapterProtocol for TSCAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("tsc")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if shared::external_lint::utility_external_lint_io::is_file(Path::new(path_str))
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = canonicalize_path(path_str);

        let mut args = vec![
            "--noEmit".to_string(),
            "--pretty".to_string(),
            "false".to_string(),
        ];
        if abs_path != "." && abs_path != "./" {
            args.push(abs_path);
        }

        let cmd = resolve_js_cmd("tsc", args, &wd.value);

        let response = exec_cmd_scan(
            self.executor.as_ref(),
            cmd,
            wd.clone(),
            60.0,
            Some(self.name()),
            path,
        )
        .await?;

        let output = format!("{}{}", response.stdout, response.stderr);
        let mut results = Vec::new();

        let pattern1 = match tsc_pattern1() {
            Some(r) => r,
            None => return Ok(LintResultList::new(vec![])),
        };
        let pattern2 = match tsc_pattern2() {
            Some(r) => r,
            None => return Ok(LintResultList::new(vec![])),
        };

        for line in output.lines() {
            let line = line.trim();
            if let Some(caps) = pattern1.captures(line).or_else(|| pattern2.captures(line)) {
                let filename = match caps.get(1) {
                    Some(m) => m.as_str().to_string(),
                    None => String::new(),
                };
                let line_num = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .unwrap_or(1);
                let col_num = caps
                    .get(3)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .unwrap_or_default();
                let code = match caps.get(4) {
                    Some(m) => m.as_str().to_string(),
                    None => String::new(),
                };
                let msg = match caps.get(5) {
                    Some(m) => m.as_str().to_string(),
                    None => String::new(),
                };

                let filename_vo =
                    shared::common::utility_path_normalization::resolve_capabilities_path(
                        FilePath::new(filename).unwrap_or(path.clone()),
                        Some(path.clone()),
                    );

                results.push(LintResult {
                    file: filename_vo,
                    line: LineNumber::new(line_num as i64),
                    column: ColumnNumber::new(col_num as i64),
                    code: ErrorCode::raw(&code),
                    message: LintMessage::new(msg),
                    source: Some(self.name()),
                    severity: Severity::HIGH,
                    enclosing_scope: Default::default(),
                    related_locations: Default::default(),
                });
            }
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        noop_apply_fix().await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

// (No protocol implementation found in this file)

// (No protocol implementation found in this file)

fn tsc_pattern1() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^(]+)\((\d+),(\d+)\):\s+error\s+(TS\d+):\s+(.*)$").ok())
        .as_ref()
}

fn tsc_pattern2() -> Option<&'static Regex> {
    static RE: OnceLock<Option<Regex>> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^([^:]+):(\d+):(\d+)\s+-\s+error\s+(TS\d+):\s+(.*)$").ok())
        .as_ref()
}

impl TSCAdapter {
    pub fn new(executor: Arc<dyn ICommandExecutorProtocol>) -> Self {
        Self { executor }
    }
}
