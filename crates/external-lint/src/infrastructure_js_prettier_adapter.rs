// PURPOSE: PrettierAdapter — ILinterAdapterPort implementation for Prettier integration
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

use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use std::path::Path;
use std::sync::Arc;

use shared::external_lint::contract_external_lint_utility_port::IExternalLintUtilityPort;

pub struct PrettierAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    utility: Arc<dyn IExternalLintUtilityPort>,
}

impl PrettierAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        utility: Arc<dyn IExternalLintUtilityPort>,
    ) -> Self {
        Self {
            executor,
            path_norm,
            utility,
        }
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for PrettierAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("prettier")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if Path::new(path_str).is_file()
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = self.utility.resolve_js_working_dir(path);
        let abs_path = self.utility.canonicalize_path(path_str);

        let cmd = self.utility.resolve_js_cmd(
            "prettier",
            shared::common::taxonomy_common_vo::PatternList::new(vec![
                "--check".to_string(),
                abs_path.value,
            ]),
            &wd,
        );

        let response = self
            .utility
            .exec_cmd_scan(
                self.executor.as_ref(),
                cmd,
                wd.clone(),
                Timeout::new(60.0),
                Some(self.name()),
                path,
            )
            .await?;

        let mut results = Vec::new();
        let combined_output = format!("{}{}", response.stdout, response.stderr);

        if combined_output.contains("[warn]") {
            let filename_vo = self
                .path_norm
                .resolve_infrastructure_path(path.clone(), Some(path.clone()));
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
        self.utility
            .js_apply_fix(self.executor.as_ref(), path, "prettier", "--write")
            .await
    }
}
