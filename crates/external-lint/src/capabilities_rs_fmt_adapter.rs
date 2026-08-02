// PURPOSE: RsFmtAdapter — ILinterAdapterProtocol implementation for rustfmt integration
//
// Runs `cargo fmt --check` on Rust projects. Since rustfmt is a formatter
// (not a linter), the adapter parses diff output lines to report each
// formatting difference as an individual LintResult.
//
// Key design decisions:
//   - Resolves Cargo.toml parent dir as working directory (via resolve_cargo_working_dir)
//   - Uses ICommandExecutorProtocol for subprocess execution with 120s timeout
//   - apply_fix runs `cargo fmt` (without --check) to auto-format
//   - Only reports added lines (+ prefix) as violations, not context lines

use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::ErrorMessage;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{ColumnNumber, LineNumber, PatternList};
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_lint_vo::LocationList;
use shared::common::taxonomy_message_vo::{ComplianceStatus, LintMessage};
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_path_normalization::resolve_capabilities_path;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::quality_rules::LinterOperationError;
use shared::quality_rules::contract_adapter_protocol::ILinterAdapterProtocol;
use std::path::Path;
use std::sync::Arc;
use tracing::debug;

use shared::common::contract_executor_protocol::ICommandExecutorProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

/// Adapter that wraps `cargo fmt --check` as an ILinterAdapterProtocol.
pub struct RustFmtAdapter {
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    executor: Arc<dyn ICommandExecutorProtocol>,
    _bin_path: Option<FilePath>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILinterAdapterProtocol for RustFmtAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("rustfmt")
    }

    fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();

        // Find the Cargo.toml parent to use as working directory
        let working_dir = self.filesystem.resolve_cargo_working_dir(path);
        let working_dir_str = working_dir.value();

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
                Some(Timeout::new(120.0)),
            )
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
                let resolved = resolve_capabilities_path(
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

        Ok(LintResultList::new(results))
    }

    fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let working_dir = self.filesystem.resolve_cargo_working_dir(path);
        let cmd = vec!["cargo".to_string(), "fmt".to_string()];
        let _ = self.executor.execute_command(
            PatternList::new(cmd),
            working_dir,
            Some(Timeout::new(120.0)),
        );
        Ok(ComplianceStatus::new(true))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl RustFmtAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorProtocol>,
        bin_path: Option<FilePath>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            executor,
            _bin_path: bin_path,
            filesystem,
        }
    }
}
