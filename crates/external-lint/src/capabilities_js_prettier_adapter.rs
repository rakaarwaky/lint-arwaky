// PURPOSE: PrettierAdapter — ILinterAdapterProtocol implementation for Prettier integration
//
// Runs `prettier --check <path>` on JS/TS files via resolve_js_cmd (npx).
// Only files with .ts/.tsx/.js/.jsx extensions are scanned.
// apply_fix runs `prettier --write <path>` to auto-format.
//
// Key details:
//   - Early-returns empty results for non-JS/TS files
//   - Uses canonical absolute paths for reliable prettier invocation
//   - Detects warnings by checking for "[warn]" in combined stdout+stderr
//   - Reports a single LintResult per file (not per-difference)

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
use shared_external_lint::IJsToolResolutionProtocol;
use shared_external_lint::contract_external_lint_protocol::ILinterAdapterProtocol;
use shared_filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared_filesystem::contract_filesystem_protocol::IToolResolutionProtocol;
use shared_quality_rules::LinterOperationError;
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct PrettierAdapter {
    lint_executor: Arc<dyn ICommandExecutorProtocol>,
    js_resolution: Arc<dyn IJsToolResolutionProtocol>,
    io: Arc<dyn IFileSystemIOProtocol>,
    tool_resolution: Arc<dyn IToolResolutionProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for PrettierAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("prettier")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = path.value();
        if self.io.is_file(Path::new(path_str))
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = self.tool_resolution.resolve_js_working_dir(path);
        let abs_path = self.io.canonicalize_path_str(path);

        let prettier_name = ToolName::new("prettier");
        let cmd = match self.tool_resolution.resolve_js_cmd(
            &prettier_name,
            vec!["--check".to_string(), abs_path.value],
            &wd,
        ) {
            Some(c) => c,
            None => return Ok(LintResultList::default()),
        };

        let response = self
            .lint_executor
            .exec_cmd_scan(cmd, wd.clone(), 60.0, Some(self.name()), path)
            .map_err(crate::convert_executor_error)?;
        let mut results = Vec::new();
        let combined_output = format!("{}{}", response.stdout, response.stderr);

        for line in combined_output.lines() {
            let trimmed = line.trim();
            if let Some(file_str) = trimmed.strip_prefix("[warn]") {
                let file_str = file_str.trim();
                if file_str.is_empty()
                    || file_str.starts_with("Code style issues")
                    || file_str.starts_with("Forget to run")
                {
                    continue;
                }
                let file_fp = FilePath::new(file_str.to_string()).unwrap_or_else(|_| path.clone());
                let filename_vo = resolve_capabilities_path(file_fp, Some(path.clone()));
                results.push(LintResult {
                    file: filename_vo,
                    line: LineNumber::new(1),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::raw("formatting"),
                    message: LintMessage::new(format!(
                        "Code style issue in {}. Run Prettier to fix.",
                        file_str
                    )),
                    source: Some(self.name()),
                    severity: Severity::MEDIUM, // FR-004: Prettier diff → MEDIUM
                    enclosing_scope: Default::default(),
                    related_locations: LocationList::new(),
                });
            }
        }

        Ok(LintResultList::new(results))
    }

    fn fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        self.js_resolution
            .js_apply_fix(path, &ToolName::new("prettier"), "--write")
            .map_err(crate::convert_executor_error)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl PrettierAdapter {
    pub fn new(
        lint_executor: Arc<dyn ICommandExecutorProtocol>,
        js_resolution: Arc<dyn IJsToolResolutionProtocol>,
        io: Arc<dyn IFileSystemIOProtocol>,
        tool_resolution: Arc<dyn IToolResolutionProtocol>,
    ) -> Self {
        Self {
            lint_executor,
            js_resolution,
            io,
            tool_resolution,
        }
    }
}
