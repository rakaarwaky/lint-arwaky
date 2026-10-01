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
use shared_cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_lint_vo::LocationList;
use shared_common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_tool_name_vo::ToolName;
use shared_common::utility_path_normalization::resolve_capabilities_path;
use shared_external_lint::ICommandExecutorProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use shared_quality_rules::LinterOperationError;
use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct TSCAdapter {
    lint_executor: Arc<dyn ICommandExecutorProtocol>,
    io: Arc<dyn IFileSystemIOProtocol>,
    tool_resolution: Arc<dyn IToolResolutionProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for TSCAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("tsc")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = path.value();
        if self.io.is_file(Path::new(path_str))
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = self.tool_resolution.resolve_js_working_dir(path);
        let abs_path = self.io.canonicalize_path_str(path);

        let mut args = vec![
            "--noEmit".to_string(),
            "--pretty".to_string(),
            "false".to_string(),
        ];
        if abs_path.value != "." && abs_path.value != "./" {
            args.push(abs_path.value);
        }

        let tsc_name = ToolName::new("tsc");
        let cmd = match self.tool_resolution.resolve_js_cmd(&tsc_name, args, &wd) {
            Some(c) => c,
            None => return Ok(LintResultList::default()),
        };

        let response = self
            .lint_executor
            .exec_cmd_scan(cmd, wd.clone(), 60.0, Some(self.name()), path)
            .map_err(crate::convert_executor_error)?;

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
                let filename = caps
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                let line_num = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .unwrap_or(1);
                let col_num = caps
                    .get(3)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .unwrap_or_default();
                let code = caps
                    .get(4)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                let msg = caps
                    .get(5)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();

                let filename_vo = resolve_capabilities_path(
                    FilePath::new(filename).unwrap_or_else(|_| path.clone()),
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
                    related_locations: LocationList::new(),
                });
            }
        }

        Ok(LintResultList::new(results))
    }

    fn fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

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
    pub fn new(
        lint_executor: Arc<dyn ICommandExecutorProtocol>,
        io: Arc<dyn IFileSystemIOProtocol>,
        tool_resolution: Arc<dyn IToolResolutionProtocol>,
    ) -> Self {
        Self {
            lint_executor,
            io,
            tool_resolution,
        }
    }
}
