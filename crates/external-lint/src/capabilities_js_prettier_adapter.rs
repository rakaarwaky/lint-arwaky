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
use shared::external_lint::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::taxonomy_filesystem_vo::ToolName;
use shared::quality_rules::LinterOperationError;
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct PrettierAdapter {
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for PrettierAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("prettier")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = path.value();
        if self.filesystem.is_file(Path::new(path_str))
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = self.filesystem.resolve_js_working_dir(path);
        let abs_path = self.filesystem.canonicalize_path_str(path);

        let prettier_name = match ToolName::new("prettier") {
            Ok(n) => n,
            Err(_) => return Ok(LintResultList::default()),
        };
        let cmd = match self.filesystem.resolve_js_cmd(
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

    fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        self.lint_executor
            .js_apply_fix(path, "prettier", "--write")
            .map_err(crate::convert_executor_error)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl PrettierAdapter {
    pub fn new(
        lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            lint_executor,
            filesystem,
        }
    }
}
