use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_protocol::ILinterAdapterProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::utility_file;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use std::path::Path;
use std::sync::Arc;

use shared::external_lint::contract_external_lint_executor_protocol::IExternalLintExecutorProtocol;
use shared::external_lint::utility_external_lint::{
    canonicalize_path, resolve_js_cmd, resolve_js_working_dir as resolve_working_dir,
};

// (No protocol implementation found in this file)

// PURPOSE: PrettierAdapter — ILinterAdapterProtocol implementation for Prettier integration
//
// Runs `prettier --check <path>` on JS/TS files via
// resolve_js_cmd (npx). Only files with .ts/.tsx/.js/.jsx extensions are scanned.
// apply_fix runs `prettier --write <path>` to auto-format.
//
// Key details:
//   - Early-returns empty results for non-JS/TS files
//   - Uses canonical absolute paths for reliable prettier invocation
//   - Detects warnings by checking for "[warn]" in combined stdout+stderr
//   - Reports a single LintResult per file (not per-difference)

// ─── Block 1: Struct Definition ───────────────────────────

pub struct PrettierAdapter {
    lint_executor: Arc<dyn IExternalLintExecutorProtocol>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl ILinterAdapterProtocol for PrettierAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("prettier")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if utility_file::is_file_generic(Path::new(path_str))
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = canonicalize_path(path_str);

        let cmd = resolve_js_cmd("prettier", vec!["--check".to_string(), abs_path], &wd.value);

        let response = self
            .lint_executor
            .exec_cmd_scan(cmd, wd.clone(), 60.0, Some(self.name()), path)
            .await?;

        let mut results = Vec::new();
        let combined_output = format!("{}{}", response.stdout, response.stderr);

        if combined_output.contains("[warn]") {
            let filename_vo = shared::common::utility_path_normalization::resolve_capabilities_path(
                path.clone(),
                Some(path.clone()),
            );
            results.push(LintResult {
                file: filename_vo,
                line: LineNumber::new(1),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw("formatting"),
                message: LintMessage::new("Code style issues found. Run Prettier to fix."),
                source: Some(self.name()),
                severity: Severity::LOW,
                enclosing_scope: Default::default(),
                related_locations: Default::default(),
            });
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        self.lint_executor
            .js_apply_fix(path, "prettier", "--write")
            .await
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

// (No protocol implementation found in this file)

// (No protocol implementation found in this file)

impl PrettierAdapter {
    pub fn new(lint_executor: Arc<dyn IExternalLintExecutorProtocol>) -> Self {
        Self { lint_executor }
    }
}
