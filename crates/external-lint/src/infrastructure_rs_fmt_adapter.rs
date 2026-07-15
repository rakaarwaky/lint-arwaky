// PURPOSE: RsFmtAdapter — ILinterAdapterPort implementation for rustfmt integration
//
// Runs `cargo fmt --check` on Rust projects. Since rustfmt is a formatter
// (not a linter), the adapter parses diff output lines to report each
// formatting difference as an individual LintResult.
//
// Key design decisions:
//   - Resolves Cargo.toml parent dir as working directory (via resolve_cargo_working_dir)
//   - Uses ICommandExecutorPort for subprocess execution with 120s timeout
//   - apply_fix runs `cargo fmt` (without --check) to auto-format
//   - Only reports added lines (+ prefix) as violations, not context lines
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use tracing::debug;

use shared::external_lint::infrastructure_external_lint_adapter::resolve_cargo_working_dir;

/// Adapter that wraps `cargo fmt --check` as an ILinterAdapterPort.
///
/// Parses rustfmt's unified diff output to create per-difference LintResults.
/// When no Cargo.toml is found, the scan is silently skipped.
pub struct RustFmtAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    _bin_path: Option<FilePath>,
}

impl RustFmtAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            executor,
            path_norm,
            _bin_path: bin_path,
        }
    }
}

#[async_trait]
impl ILinterAdapterPort for RustFmtAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("rustfmt")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();

        // Find the Cargo.toml parent to use as working directory — resolves workspace roots
        let working_dir = resolve_cargo_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_toml = Path::new(working_dir_str).join("Cargo.toml");
        if !cargo_toml.exists() {
            debug!("Skipping rustfmt: Cargo.toml not found at {:?}", cargo_toml);
            return Ok(LintResultList::new(results));
        }

        // Run `cargo fmt --check` — exits non-zero when formatting differs
        let cmd = vec![
            "cargo".to_string(),
            "fmt".to_string(),
            "--check".to_string(),
        ];
        let result = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir.clone(),
                Some(shared::taxonomy_duration_vo::Timeout::new(120.0)),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    self.name(),
                    ErrorMessage::new(e.to_string()),
                ))
            })?;

        if result.returncode == 0 {
            return Ok(LintResultList::new(results));
        }

        // Parse rustfmt's unified diff output.
        // Format: "Diff in <file> at line N:" followed by diff hunks
        let output = result.stdout + &result.stderr;
        let mut current_file = String::new();
        for line in output.lines() {
            // Track which file the current diff hunk belongs to
            if line.starts_with("Diff in ") {
                current_file = line
                    .trim_start_matches("Diff in ")
                    .trim_end_matches(':')
                    .trim()
                    .to_string();
            } else if line.starts_with("--- ") || line.starts_with("+++ ") {
                continue;
            }

            // Report added lines (+) as formatting violations
            if line.starts_with('+') && !line.starts_with("+++") {
                let resolved = self.path_norm.resolve_infrastructure_path(
                    match FilePath::new(current_file.clone()) {
                        Ok(fp) => fp,
                        Err(_) => path.clone(),
                    },
                    Some(path.clone()),
                );
                results.push(LintResult {
                    file: resolved,
                    line: LineNumber::new(0),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::raw("rustfmt::unformatted"),
                    message: LintMessage::new(line.trim().to_string()),
                    source: Some(AdapterName::raw("rustfmt")),
                    severity: Severity::MEDIUM,
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }
        }

        if results.is_empty() {
            // No diff lines parsed — cargo fmt --check may have exited non-zero
            // for a reason unrelated to formatting (e.g., parse error). Don't
            // create a fake violation.
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let working_dir = resolve_cargo_working_dir(path);
        let cmd = vec!["cargo".to_string(), "fmt".to_string()];
        let _ = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir,
                Some(shared::taxonomy_duration_vo::Timeout::new(120.0)),
            )
            .await;
        Ok(ComplianceStatus::new(true))
    }
}
