use std::path::Path;
use std::sync::Arc;

use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
use crate::code_analysis::contract_adapter_port::ILinterAdapterPort;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_error::AdapterError;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_common_vo::PatternList;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_operation_error::LinterOperationError;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;
use tracing::debug;

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

    fn _resolve_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }
        let current = match std::env::current_dir() {
            Ok(c) => c,
            Err(_) => return path.clone(),
        };
        let mut current = current;
        for _ in 0..10 {
            if current.join("Cargo.toml").exists()
                || current.join("lint_arwaky.config.yaml").exists()
                || current.join(".git").is_dir()
            {
                return FilePath::new(current.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| FilePath::new(".".to_string()).unwrap_or_default());
            }
            if !current.pop() {
                break;
            }
        }
        FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
    }
}

#[async_trait]
impl ILinterAdapterPort for RustFmtAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("rustfmt")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = self._resolve_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_toml = Path::new(working_dir_str).join("Cargo.toml");
        if !cargo_toml.exists() {
            debug!("Skipping rustfmt: Cargo.toml not found at {:?}", cargo_toml);
            return Ok(LintResultList::new(results));
        }

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
                Some(crate::shared_common::taxonomy_duration_vo::Timeout::new(
                    120.0,
                )),
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

        let output = result.stdout + &result.stderr;
        let mut current_file = String::new();
        for line in output.lines() {
            if line.starts_with("Diff in ") {
                current_file = line
                    .trim_start_matches("Diff in ")
                    .trim_end_matches(':')
                    .trim()
                    .to_string();
            } else if line.starts_with("--- ") || line.starts_with("+++ ") {
                continue;
            }

            if line.starts_with('+') && !line.starts_with("+++") {
                let resolved = self.path_norm.resolve_infrastructure_path(
                    FilePath::new(current_file.clone()).unwrap_or_else(|_| path.clone()),
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
            results.push(LintResult {
                file: FilePath::new("Cargo.toml".to_string()).unwrap_or_default(),
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw("rustfmt::unformatted"),
                message: LintMessage::new("Project is not formatted by rustfmt".to_string()),
                source: Some(AdapterName::raw("rustfmt")),
                severity: Severity::MEDIUM,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let working_dir = self._resolve_working_dir(path);
        let cmd = vec!["cargo".to_string(), "fmt".to_string()];
        let _ = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir,
                Some(crate::shared_common::taxonomy_duration_vo::Timeout::new(
                    120.0,
                )),
            )
            .await;
        Ok(ComplianceStatus::new(true))
    }
}
